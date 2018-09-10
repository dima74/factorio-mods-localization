import fs from 'fs';
import path from 'path';
import git from 'simple-git/promise';
import { GITHUB_COMMIT_MESSAGE, GITHUB_COMMIT_USER_NAME, GITHUB_COMMIT_USER_EMAIL } from './constants';

function getDirectoryFilesPaths(directory) {
    const dirents = fs.readdirSync(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => !dirent.isDirectory())
        .map(dirent => path.join(directory, dirent.name));
}

export default class Repository {
    constructor(fullName, directoryPath) {
        this.fullName = fullName;
        this.localesPath = path.join(directoryPath, 'locale');
        this.localeEnPath = path.join(this.localesPath, 'en');
        this.git = git(directoryPath);
    }

    checkForLocaleFolder() {
        if (!fs.existsSync(this.localeEnPath)) {
            throw new Error(`no /locale folder found in github repository, ${this.localeEnPath}`);
        }
    }

    getEnglishFiles() {
        return getDirectoryFilesPaths(this.localeEnPath);
    }

    getLocalizations() /* { [language_code]: [absolute_path_to_file, ...] } */ {
        const localizations = {};
        for (const languageCode of fs.readdirSync(this.localesPath)) {
            if (languageCode !== 'en') {
                const localePath = path.join(this.localesPath, languageCode);
                localizations[languageCode] = getDirectoryFilesPaths(localePath);
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