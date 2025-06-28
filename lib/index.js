const addon = require('../native');
const { EventEmitter } = require('events');

console.log('Загруженный аддон:', addon); // <--- ДОБАВЬ ЭТУ СТРОКУ


class RustCatch extends EventEmitter {
    constructor() {
        super();
        this._isRunning = false;

        // Это наш главный колбэк. Rust будет вызывать его,
        // а он, в свою очередь, будет генерировать события.
        this._rustCallback = (event) => {
            if (event && event.type) {
                // event это, например, { type: 'keydown', key: 'KeyA' }
                // Генерируем событие с таким же именем, как тип.
                this.emit(event.type, event);
            }
        };
    }

    start() {
        if (this._isRunning) {
            console.warn("rustcatch уже запущен.");
            return;
        }
        // Передаем наш колбэк в нативный аддон.
        addon.start(this._rustCallback);
        this._isRunning = true;
        console.log("rustcatch: Глобальный перехватчик запущен.");
    }

    stop() {
        // Функционал остановки мы добавим позже.
        // Это более сложная задача.
        if (!this._isRunning) {
            return;
        }
        console.log("rustcatch: Функция stop() еще не реализована.");
        // this._isRunning = false;
    }
}

// Экспортируем один-единственный экземпляр нашего класса.
module.exports = new RustCatch();