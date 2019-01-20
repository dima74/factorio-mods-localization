import assert from 'assert';
import Sentry from '@sentry/node';
import { IS_DEVELOPMENT } from './constants';

assert(process.env.SENTRY_DSN);
Sentry.init({
    dsn: process.env.SENTRY_DSN,
    integrations: [new Sentry.Integrations.Transaction()],
    // https://github.com/getsentry/sentry-javascript/issues/1600#issuecomment-426010114
    beforeSend: (event, hint) => {
        console.error(hint.originalException || hint.syntheticException);
        // returning null drops the event, so nothing will be send to sentry in development
        return IS_DEVELOPMENT ? null : event;
    },
});