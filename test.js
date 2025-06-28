// test.js
const rustcatch = require('./lib');

console.log('Запускаем тест...');

rustcatch.on('keydown', (event) => {
    console.log(`Нажата клавиша:`, event);
});

rustcatch.on('keyup', (event) => {
    console.log(`Отпущена клавиша:`, event);
});

// Запускаем
rustcatch.start();

// Устанавливаем таймер для остановки через 15 секунд
setTimeout(() => {
    console.log('\n--- Попытка остановки через 15 секунд ---');
    rustcatch.stop();
    console.log('--- Модуль остановлен. Нажатия больше не должны отслеживаться. ---');
    console.log('--- Программа завершится через 5 секунд. ---');

    // Даем еще 5 секунд, чтобы убедиться, что события не приходят
    setTimeout(() => {
        process.exit(0);
    }, 5000);

}, 15000);