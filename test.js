// test.js
const rustcatch = require('./lib'); // Импортируем наш модуль

console.log('Запускаем тест... Нажимайте клавиши. Для выхода закройте окно (Ctrl+C).');

// Подписываемся на события
rustcatch.on('keydown', (event) => {
    console.log(`Нажата клавиша:`, event);
});

rustcatch.on('keyup', (event) => {
    console.log(`Отпущена клавиша:`, event);
});

rustcatch.on('mousemove', (event) => {
    // Раскомментируйте, если хотите утонуть в логах :)
    // console.log(`Мышь движется:`, event);
});

// Запускаем перехватчик
rustcatch.start();