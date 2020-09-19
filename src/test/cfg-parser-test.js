import '../base.js';
import github from '../github.js';
import path from 'path';
import Repository from '../repository.js';
import { escapeStringsIfNeeded } from '../cfg-parser.js';
import fs from 'fs-extra';

async function cloneAllRepos() {
    const repositories = await github.getAllRepositories();
    for (const repository of repositories) {
        console.log(`git clone --depth 1 https://github.com/${repository.fullName} &`);
    }
}

async function escapeAllFilesInAllRepositories() {
    const REPOS_PATH = '../../temp/repos1';
    const repos = await github.getAllRepositories();
    for (const { fullName } of repos) {
        const repositoryDirectory = path.join(REPOS_PATH, fullName.split('/')[1]);
        const repository = new Repository(fullName, repositoryDirectory);

        if (!fs.existsSync(repository.localeEnPath)) continue;
        for (const filePath of await repository.getEnglishFiles()) {
            escapeStringsIfNeeded(filePath);
        }
        for (const filePaths of Object.values(await repository.getLocalizations())) {
            for (const filePath of filePaths) {
                escapeStringsIfNeeded(filePath);
            }
        }
    }
}

await github.init();
await escapeAllFilesInAllRepositories();
