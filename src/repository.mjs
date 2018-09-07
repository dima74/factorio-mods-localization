import fs from 'fs';
import path from 'path';

export default class Repository {
    constructor({owner, repo}, directoryPath) {
        this.owner = owner;
        this.repo = repo;
        // this.directoryPath = directoryPath;
        this.localeEnPath = path.join(directoryPath, 'locale/en');
    }

    checkForLocaleFolder() {
        return fs.existsSync(this.localeEnPath);
    }

    listEnglishFiles() {
        const filesNames = fs.readdirSync(this.localeEnPath);
        return filesNames.map(fileName => path.join(this.localeEnPath, fileName));
    }
}