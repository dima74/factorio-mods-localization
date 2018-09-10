import github from './src/github';
import webhooks from './src/webhooks';
import main from './src/main';
import './src/base';

async function init() {
    await github.init();
    await webhooks.init();

    // todo change it to push only every week, not every day
    await main.pushAllCrowdinChangesToGithub();
}

init();