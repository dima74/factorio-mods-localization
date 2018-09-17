import github from './src/github';
import webServer from './src/webServer';
import main from './src/main';
import database from './src/database';
import './src/base';
import './src/sentry';
import Raven from 'raven';

async function init() {
    await github.init();
    await webServer.init();
    await database.init();

    if (await database.isLastUpdateLongEnough()) {
        await main.pushAllCrowdinChangesToGithub();
        await database.commitUpdate();
    }
}

Raven.context(init);