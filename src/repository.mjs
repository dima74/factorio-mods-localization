import fs from 'fs';
import path from 'path';

export default class Repository {
    constructor(path) {
        this.path = path;
    }

    checkForLocaleFolder() {
        const localeEnPath = path.join(this.path, 'locale/en');
        return fs.existsSync(localeEnPath);
    }
}