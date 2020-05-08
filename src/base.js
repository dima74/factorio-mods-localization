// единственный способ получить нормальные stack trac'ы для async/await методов — настроить babel для преобразования async/await в Promise.then, с последующим замещением глобальной реализации Promise'ов (через `global.Promise = require('bluebird');`)
// Все остальные способы не работают (https://github.com/nodejs/node/issues/11865)

// IMPORTANT:
//  unhandledRejection event is deprecated
//  (so we must always handle any possible rejection)

import Sentry from '@sentry/node';

export function handleReject(e) {
    Sentry.captureException(e);
}
