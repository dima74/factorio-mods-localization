import fs from 'fs-extra';
import path from 'path';
import git from 'simple-git/promise';
import { GITHUB_COMMIT_MESSAGE, GITHUB_COMMIT_USER_NAME, GITHUB_COMMIT_USER_EMAIL } from './constants';
import { getSubdirectories } from './utility';

async function getDirectoryCfgFilesPaths(directory) {
    const dirents = await fs.readdir(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => !dirent.isDirectory())
        .filter(dirent => dirent.name.endsWith('.cfg'))
        .map(dirent => path.join(directory, dirent.name));
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

    async checkTranslationFilesMatchEnglishFiles() {
        const localizations = await this.getLocalizations();
        for (const [languageCode, filePaths] of Object.entries(localizations)) {
            for (const filePath of filePaths) {
                const fileName = path.basename(filePath);
                if (!await fs.exists(path.join(this.localeEnPath, fileName))) {
                    throw Error(`[add-repository] [${this.fullName}] matched english file not found for "${languageCode}/${fileName}"`);
                }
            }
        }
    }

    async getEnglishFiles() {
        return await getDirectoryCfgFilesPaths(this.localeEnPath);
    }

    async getLanguageCodes() {
        const codes = await getSubdirectories(this.localesPath);
        return codes.filter(code => code !== 'template');  // https://github.com/Karosieben/boblocale
    }

    async getLocalizations() /* { [language_code]: [absolute_path_to_file, ...] } */ {
        const localizations = {};
        for (const languageCode of await this.getLanguageCodes()) {
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