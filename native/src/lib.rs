use neon::prelude::*;
use neon::event::Channel;
use rdev::{listen, simulate, EventType, Key};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::panic;
use std::panic::set_hook; // <-- ДОБАВЛЕНИЕ 1: Импортируем set_hook
use once_cell::sync::Lazy;

static SHOULD_STOP: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
static LISTENER_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

fn event_listener(channel: Channel, callback: Arc<Root<JsFunction>>) {
    // --- НАШЕ УЛУЧШЕНИЕ ---
    // Устанавливаем "тихий" обработчик паники, который ничего не делает.
    // Это скроет сообщение "thread panicked" из консоли.
    set_hook(Box::new(|_| {})); // <-- ДОБАВЛЕНИЕ 2: Устанавливаем пустой хук

    let result = panic::catch_unwind(move || {
        listen(move |event| {
            if *SHOULD_STOP.lock().unwrap() {
                panic!("Stopping listener thread by request");
            }

            let callback = callback.clone();
            channel.send(move |mut cx| {
                let obj = cx.empty_object();

                match event.event_type {
                    EventType::KeyPress(key) => {
                        let event_type = cx.string("keydown");
                        obj.set(&mut cx, "type", event_type)?;
                        
                        let key_name = cx.string(format!("{:?}", key));
                        obj.set(&mut cx, "key", key_name)?;
                    },
                    EventType::KeyRelease(key) => {
                        let event_type = cx.string("keyup");
                        obj.set(&mut cx, "type", event_type)?;

                        let key_name = cx.string(format!("{:?}", key));
                        obj.set(&mut cx, "key", key_name)?;
                    },
                    _ => return Ok(()),
                };
                
                let func = callback.to_inner(&mut cx);
                let this = cx.undefined();
                let args = vec![ obj.upcast::<JsValue>() ];
                func.call(&mut cx, this, args)?;

                Ok(())
            });
        }).expect("Could not listen for events");
    });

    if result.is_err() {
        // Эта строка все еще будет печататься в консоль node,
        // но страшного сообщения о панике уже не будет.
        // println!("Listener thread has stopped cleanly via panic.");
    }
}

// ... остальной код (start, stop, main) остается БЕЗ ИЗМЕНЕНИЙ ...
fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if LISTENER_THREAD.lock().unwrap().is_some() {
        println!("Listener is already running.");
        return Ok(cx.undefined());
    }

    *SHOULD_STOP.lock().unwrap() = false;
    
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();
    let channel_callback = Arc::new(callback);

    println!("Starting listener thread...");
    let handle = thread::spawn(move || {
        event_listener(channel, channel_callback);
    });

    *LISTENER_THREAD.lock().unwrap() = Some(handle);

    Ok(cx.undefined())
}

fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if let Some(handle) = LISTENER_THREAD.lock().unwrap().take() {
        println!("Stopping listener thread...");
        *SHOULD_STOP.lock().unwrap() = true;

        if let Err(e) = simulate(&EventType::KeyPress(Key::ShiftRight)) {
            println!("Failed to send fake key event: {:?}", e);
        }

        handle.join().expect("Failed to join listener thread");
        // println!("Listener stopped successfully."); // Можно убрать, чтобы JS-часть выглядела чище
    } else {
        println!("Listener is not running.");
    }

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", start)?;
    cx.export_function("stop", stop)?;
    Ok(())
}