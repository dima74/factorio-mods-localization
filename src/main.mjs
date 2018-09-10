import github from './github';
import crowdinApi from './crowdin';

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
        repository.checkForLocaleFolder();
        const crowdin = crowdinApi.getCrowdinDirectory(repository);
        await crowdin.onRepositoryAdded();
        console.log('Successfully added', fullName);
    }

    async pushAllCrowdinChangesToGithub() {
        const repositories = await github.getAllRepositories();
        const repositoriesFiltered = await crowdinApi.filterRepositories(repositories);
        for (const repository of repositories) {
            await this.pushRepositoryCrowdinChangesToGithub(repository);
        }
    }

    async pushRepositoryCrowdinChangesToGithub(repository /* { installationId, owner, repo } */) {
        const response = await github.api.apps.createInstallationToken({ installation_id: repository.installationId });
        const token = response.data.token;
        console.log(token);

        import git from 'simple-git/promise';
        await git().clone(`https://x-access-token:${token}@github.com/${owner}/${repo}.git`, ROOT + uuid.v4());
    }
}

const main = new Main();
export default main;
