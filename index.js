import './src/console-log-interceptor.js';
import github from './src/github.js';
import webServer from './src/webServer.js';
import main from './src/main.js';
import database from './src/database.js';
import crowdinApi from './src/crowdin.js';
import './src/base.js';
import './src/sentry.js';
import { IS_DEVELOPMENT } from './src/constants.js';
import { handleReject } from './src/base.js';

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

init().catch(handleReject);
