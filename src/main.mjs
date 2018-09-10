import github from './github';
import crowdinApi from './crowdin';
import { moveTranslatedFilesToRepository } from './utility';

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
        const repository = await installation.cloneRepository(fullName);
        repository.checkForLocaleFolder();
        const crowdin = crowdinApi.getCrowdinDirectory(repository);
        await crowdin.onRepositoryAdded();
        console.log('Successfully added', fullName);
    }

    async pushAllCrowdinChangesToGithub() {
        const repositories = await github.getAllRepositories();
        const repositoriesFiltered = await crowdinApi.filterRepositories(repositories);
        const translationsDirectory = await crowdinApi.downloadAllTranlations();
        for (const repository of repositoriesFiltered) {
            await this.pushRepositoryCrowdinChangesToGithub(translationsDirectory, repository);
        }
    }

    async pushRepositoryCrowdinChangesToGithub(translationsDirectory, { installation, fullName }) {
        const repository = await installation.cloneRepository(fullName);
        await moveTranslatedFilesToRepository(translationsDirectory, repository);
        const areChangesExists = await repository.pushAllChanges();
        if (areChangesExists) {
            console.log('Successfully pushed', fullName);
        }
    }
}

const main = new Main();
export default main;
