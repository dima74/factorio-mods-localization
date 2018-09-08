import fs from 'fs';
import path from 'path';

function getDirectoryFilesPaths(directory) {
    const dirents = fs.readdirSync(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => !dirent.isDirectory())
        .map(dirent => path.join(directory, dirent.name));
}

export default class Repository {
    constructor({ owner, repo }, directoryPath) {
        this.owner = owner;
        this.repo = repo;
        this.localesPath = path.join(directoryPath, 'locale');
        this.localeEnPath = path.join(this.localesPath, 'en');
    }

    checkForLocaleFolder() {
        return fs.existsSync(this.localeEnPath);
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
}