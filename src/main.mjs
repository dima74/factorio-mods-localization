import github from './github';
import Crowdin from './crowdin';

class Main {
    async onRepositoriesAdded(installationId, repositories) {
        const installation = await github.getInstallation(installationId);
        for (const repository of repositories) {
            await this.onRepositoryAdded(installation, repository);
        }
    }

    async onRepositoryAdded(installation, repositoryInfo) {
        const fullName = repositoryInfo.full_name;
        console.log('\nAdd repository', fullName);
        const repository = await installation.downloadRepository(fullName);
        if (!repository.checkForLocaleFolder()) {
            throw new Error('no /locale folder found in github repository');
        }
        const crowdin = new Crowdin(repository);
        await crowdin.onRepositoryAdded();
        console.log('Successfully added', fullName);
    }
}

const main = new Main();
export default main;
