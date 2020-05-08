import './base.js';
import main from './main.js';
import github from './github.js';
import crowdinApi from './crowdin.js';
// import pushPayload from './temp/pushPayload.js';

const installationId = 207362;
const repositories = [{
    id: 121222864,
    name: 'factorio-mod-example',
    full_name: 'dima74/factorio-mod-example',
    private: false,
}];
const fullName = 'dima74/factorio-mod-example';

async function onRepositoriesAddedWebhook() {
    await crowdinApi.init();
    await crowdinApi.deleteAllDirectories();
    await github.init();
    await main.onRepositoriesAddedWebhook(installationId, repositories);
}

async function pushAllCrowdinChangesToGithub() {
    await github.init();
    await main.pushAllCrowdinChangesToGithub();
}

async function pushRepositoryCrowdinChangesToGithub() {
    await github.init();
    await main.pushRepositoryCrowdinChangesToGithub({ installationId, fullName });
}

async function onPushWebhook() {
    await github.init();
    await main.onPushWebhook(pushPayload);
}

// onRepositoriesAddedWebhook();
// pushAllCrowdinChangesToGithub();
// pushRepositoryCrowdinChangesToGithub();
// crowdinApi.downloadAllTranlations();
// onPushWebhook();
