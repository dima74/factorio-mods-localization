import fs from 'fs-extra';
import path from 'path';
import git from 'simple-git/promise';
import { GITHUB_COMMIT_MESSAGE, GITHUB_COMMIT_USER_NAME, GITHUB_COMMIT_USER_EMAIL } from './constants';

async function getDirectoryCfgFilesPaths(directory) {
    const dirents = await fs.readdir(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => !dirent.isDirectory())
        .filter(dirent => dirent.name.endsWith('.cfg'))
        .map(dirent => path.join(directory, dirent.name));
}

async function getSubdirectories(directory) {
    const dirents = await fs.readdir(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => dirent.isDirectory())
        .map(dirent => dirent.name);
}

export default class Repository {
    constructor(fullName, directoryPath) {
        this.fullName = fullName;
        this.localesPath = path.join(directoryPath, 'locale');
        this.localeEnPath = path.join(this.localesPath, 'en');
        this.git = git(directoryPath);
    }

    async checkForLocaleFolder() {
        if (!await fs.exists(this.localeEnPath)) {
            throw Error(`no /locale folder found in github repository, ${this.localeEnPath}`);
        }
    }

    async getEnglishFiles() {
        return await getDirectoryCfgFilesPaths(this.localeEnPath);
    }

    async getLocalizations() /* { [language_code]: [absolute_path_to_file, ...] } */ {
        const localizations = {};
        for (const languageCode of await getSubdirectories(this.localesPath)) {
            if (languageCode !== 'en') {
                const localePath = path.join(this.localesPath, languageCode);
                localizations[languageCode] = await getDirectoryCfgFilesPaths(localePath);
            }
        }
        return localizations;
    }

    async pushAllChanges() {
        const git = this.git;
        await git.addConfig('user.name', GITHUB_COMMIT_USER_NAME);
        await git.addConfig('user.email', GITHUB_COMMIT_USER_EMAIL);
        await git.add('.');
        const areChangesExists = (await git.status()).files.length > 0;
        if (areChangesExists) {
            await git.commit(GITHUB_COMMIT_MESSAGE);
            await git.push();
        }
        return areChangesExists;
    }
}