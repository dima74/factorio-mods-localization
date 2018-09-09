import main from './main';
import github from './github';
import Crowdin from './crowdin';
import './base';

async function onRepositoriesAdded() {
    await new Crowdin(null).deleteAllDirectories();

    const installationId = 207362;
    const repositories = [{
        id: 121222864,
        name: 'factorio-mod-example',
        full_name: 'dima74/factorio-mod-example',
        private: false,
    }];
    await github.init();
    await main.onRepositoriesAdded(installationId, repositories);
}

process.env.NODE_ENV = 'development';
onRepositoriesAdded();