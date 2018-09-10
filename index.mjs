import github from './src/github';
import webhooks from './src/webhooks';
import main from './src/main';
import database from './src/database';
import './src/base';

async function init() {
    await github.init();
    await webhooks.init();
    await database.init();

    if (await database.isLastUpdateLongEnough()) {
        await main.pushAllCrowdinChangesToGithub();
        await database.commitUpdate();
    }
}

init();