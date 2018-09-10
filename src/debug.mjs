import main from './main';
import github from './github';
import crowdinApi from './crowdin';
import './base';

const installationId = 207362;
const repositories = [{
    id: 121222864,
    name: 'factorio-mod-example',
    full_name: 'dima74/factorio-mod-example',
    private: false,
}];
const fullName = 'dima74/factorio-mod-example';

async function onRepositoriesAdded() {
    await crowdinApi.deleteAllDirectories();
    await github.init();
    await main.onRepositoriesAdded(installationId, repositories);
}

async function pushAllCrowdinChangesToGithub() {
    await github.init();
    await main.pushAllCrowdinChangesToGithub();
}

async function pushRepositoryCrowdinChangesToGithub() {
    await github.init();
    await main.pushRepositoryCrowdinChangesToGithub({ installationId, fullName });
}

// onRepositoriesAdded();
pushAllCrowdinChangesToGithub();
// pushRepositoryCrowdinChangesToGithub();
// crowdinApi.downloadAllTranlations();