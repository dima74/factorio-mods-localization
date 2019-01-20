import './src/console-log-interceptor';
import github from './src/github';
import webServer from './src/webServer';
import main from './src/main';
import database from './src/database';
import crowdinApi from './src/crowdin';
import './src/base';
import './src/sentry';
import Raven from 'raven';
import { IS_DEVELOPMENT } from './src/constants';

async function init() {
    await crowdinApi.init();
    await github.init();
    await webServer.init();
    await database.init();

    if (!IS_DEVELOPMENT && await database.isLastUpdateLongEnough()) {
        await main.pushAllCrowdinChangesToGithub();
        await database.commitUpdate();
    }
}

Raven.context(init);