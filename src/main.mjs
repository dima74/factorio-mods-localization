import crowdin from './crowdin';
import github from './github';

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
            throw new Error('no /locale folder in github repository');
        }
        // todo handle case if /locale/en contains subfolders
        crowdin.createRepositoryDirectory(repository);
        await crowdin.addEnglishFiles(repository);

        console.log('Successfully added', fullName);
    }
}

const main = new Main();
export default main;
