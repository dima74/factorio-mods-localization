// единственный способ получить нормальные stack trac'ы для async/await методов — настроить babel для преобразования async/await в Promise.then, с последующим замещением глобальной реализации Promise'ов (через `global.Promise = require('bluebird');`)
// Все остальные способы не работают (https://github.com/nodejs/node/issues/11865)

// IMPORTANT:
//  unhandledRejection event is deprecated
//  (so we must always handle any possible rejection)

// proper logging on error (why node doesn't have it by default???)
// process
//     .on('unhandledRejection', (reason, p) => {
//         console.log('unhandledRejection');
//         console.error('Unhandled Rejection at Promise');
//         if (reason.host) {
//             console.error(reason.host + reason.path);
//         }
//         console.error(reason);
//     })
//     .on('uncaughtException', err => {
//         console.error(err, 'Uncaught Exception thrown');
//     });

export function handleReject(reason) {
    console.error(reason);
}