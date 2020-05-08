import github from './github.js';
import crowdinApi from './crowdin.js';
import { getAllModifiedAndAddedFiles, moveTranslatedFilesToRepository } from './utility.js';
import { handleReject } from './base.js';

class Main {
    async onRepositoriesAddedWebhook(installationId, repositories) {
        const installation = await github.getInstallation(installationId);
        for (const repository of repositories) {
            await this.onRepositoryAdded(installation, repository).catch(handleReject);
        }
    }

    async onRepositoryAdded(installation, repositoryInfo) {
        const fullName = repositoryInfo.full_name;
        console.log(`\n[add-repository] [${fullName}] starting...`);
        const repository = await installation.cloneRepository(fullName);
        await repository.checkForLocaleFolder();
        await repository.checkTranslationFilesMatchEnglishFiles();
        const crowdin = crowdinApi.getCrowdinDirectory(repository);
        await crowdin.onRepositoryAdded();
        console.log(`[add-repository] [${fullName}] success`);
    }

    async pushAllCrowdinChangesToGithub() {
        console.log('\n[update-github-from-crowdin] [*] starting...');
        const repositories = await github.getAllRepositories();
        const repositoriesFiltered = await crowdinApi.filterRepositories(repositories);
        const translationsDirectory = await crowdinApi.downloadAllTranlations();
        for (const repository of repositoriesFiltered) {
            await this._pushRepositoryCrowdinChangesToGithub(translationsDirectory, repository).catch(handleReject);
        }
        console.log('[update-github-from-crowdin] [*] success');
    }

    async pushRepositoryCrowdinChangesToGithub(fullName) {
        console.log(`\n[update-github-from-crowdin] [${fullName}] starting...`);
        const repositories = await github.getAllRepositories();
        const repositoriesFiltered = await crowdinApi.filterRepositories(repositories);
        const translationsDirectory = await crowdinApi.downloadAllTranlations();

        const repository = repositoriesFiltered.filter(repository => repository.fullName === fullName)[0];
        if (!repository) return `Repository ${fullName} not found`;

        await this._pushRepositoryCrowdinChangesToGithub(translationsDirectory, repository);
        console.log(`[update-github-from-crowdin] [${fullName}] success`);
        return 'Ok';
    }

    async _pushRepositoryCrowdinChangesToGithub(translationsDirectory, { installation, fullName }) {
        const repository = await installation.cloneRepository(fullName);
        await moveTranslatedFilesToRepository(translationsDirectory, repository);
        const areChangesExists = await repository.pushAllChanges();
        if (areChangesExists) {
            console.log(`[update-github-from-crowdin] [${fullName}] pushed`);
        } else {
            console.log(`[update-github-from-crowdin] [${fullName}] no changes found`);
        }
    }

    async onPushWebhook(data) {
        console.log(`\n[push-webhook] [${data.repository.full_name}] starting...`);
        const modifiedFiles = getAllModifiedAndAddedFiles(data.commits);
        const modifiedLocaleEnFiles = modifiedFiles
            .filter(file => file.startsWith('locale/en'))
            .map(file => file.substring('locale/en/'.length));
        if (modifiedLocaleEnFiles.length === 0) {
            console.log(`[push-webhook] [${data.repository.full_name}] no modified/added english files found`);
            return;
        }

        const installation = await github.getInstallation(data.installation.id);
        const repository = await installation.cloneRepository(data.repository.full_name);
        const crowdin = crowdinApi.getCrowdinDirectory(repository);
        await crowdin.updateFilesOnCrowdin(modifiedLocaleEnFiles);
        console.log(`[push-webhook] [${data.repository.full_name}] success`);
    }
}

const main = new Main();
export default main;
