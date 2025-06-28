use neon::prelude::*;
use neon::event::Channel;
use rdev::{listen, EventType};
use std::thread;
use std::sync::Arc; // <-- ИСПРАВЛЕНИЕ 1: Импортируем Arc

// Эта функция будет запущена в отдельном потоке.
// Теперь она правильно принимает колбэк.
fn event_listener(channel: Channel, callback: Arc<Root<JsFunction>>) { // <-- ИСПРАВЛЕНИЕ 2: Принимаем второй аргумент
    if let Err(error) = listen(move |event| {
        // Клонируем 'Arc', чтобы передать его в замыкание. Это дешевая операция.
        let callback = callback.clone();
        channel.send(move |mut cx| {
            // Этот код выполнится в основном потоке Node.js.
            let obj = cx.empty_object();

            match event.event_type {
                EventType::KeyPress(key) => {
                    let event_type = cx.string("keydown");
                    let key_name = cx.string(format!("{:?}", key));
                    obj.set(&mut cx, "type", event_type)?;
                    obj.set(&mut cx, "key", key_name)?;
                },
                EventType::KeyRelease(key) => {
                    let event_type = cx.string("keyup");
                    let key_name = cx.string(format!("{:?}", key));
                    obj.set(&mut cx, "type", event_type)?;
                    obj.set(&mut cx, "key", key_name)?;
                },
                EventType::MouseMove { x, y } => {
                    let event_type = cx.string("mousemove");
                    let js_x = cx.number(x);
                    let js_y = cx.number(y);
                    obj.set(&mut cx, "type", event_type)?;
                    obj.set(&mut cx, "x", js_x)?;
                    obj.set(&mut cx, "y", js_y)?;
                }
                _ => {
                    return Ok(());
                }
            };
            
            // ИСПРАВЛЕНИЕ 3: Используем переданный колбэк, а не пытаемся получить его из аргументов.
            // .to_inner() безопасно извлекает JsFunction из Root.
            let func = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![ obj.upcast::<JsValue>() ];

            func.call(&mut cx, this, args)?;

            Ok(())
        });
    }) {
        println!("Ошибка rdev: {:?}", error)
    }
}

// Эта функция будет доступна из JavaScript.
fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Получаем колбэк и "рутируем" его, чтобы сборщик мусора его не удалил.
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();
    
    // Помещаем "рутированный" колбэк в Arc для потокобезопасного доступа.
    let channel_callback = Arc::new(callback);

    // Запускаем наш слушатель событий в новом потоке.
    thread::spawn(move || {
        // Теперь мы передаем оба аргумента, и все сходится.
        event_listener(channel, channel_callback);
    });

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", start)?;
    Ok(())
}