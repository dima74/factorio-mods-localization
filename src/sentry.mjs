import assert from 'assert';
import Raven from 'raven';
import { IS_DEVELOPMENT } from './constants';

if (!IS_DEVELOPMENT) {
    assert(process.env.SENTRY_DSN);
}
const config = {
    captureUnhandledRejections: true,
    sendTimeout: 10,
    autoBreadcrumbs: true,
};
Raven.config(!IS_DEVELOPMENT && process.env.SENTRY_DSN, config).install();