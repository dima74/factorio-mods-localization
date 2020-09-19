import fs from 'fs-extra';
import path from 'path';
import git from 'simple-git/promise.js';
import { GITHUB_COMMIT_MESSAGE, GITHUB_COMMIT_USER_EMAIL, GITHUB_COMMIT_USER_NAME } from './constants.js';
import { getSubdirectories } from './utility.js';
import { normalizeLanguageCode } from './crowdin.js';

async function getDirectoryCfgFilesPaths(directory) {
    const dirents = await fs.readdir(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => dirent.isFile())
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

    checkForLocaleFolder() {
        if (!fs.existsSync(this.localeEnPath)) {
            throw Error(`[add-repository] [${this.fullName}] "/locale/en" subdirectory not found in github repository`);
        }
    }

    async checkTranslationFilesMatchEnglishFiles() {
        const localizations = await this.getLocalizations();
        for (const [languageCode, filePaths] of Object.entries(localizations)) {
            for (const filePath of filePaths) {
                const fileName = path.basename(filePath);
                if (!fs.existsSync(path.join(this.localeEnPath, fileName))) {
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
                localizations[normalizeLanguageCode(languageCode)] = await getDirectoryCfgFilesPaths(localePath);
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
