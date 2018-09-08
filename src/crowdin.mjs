import axios from 'axios';
import fs from 'fs';
import path from 'path';
import FormData from 'form-data';
import assert from 'assert';

class Crowdin {
    constructor(repository) {
        this.repository = repository;
        const projectId = process.env.CROWDIN_PROJECT_ID;
        const apiKey = process.env.CROWDIN_API_KEY;
        if (!projectId || !apiKey) {
            console.error('Environment variables CROWDIN_PROJECT_ID and CROWDIN_API_KEY must be set');
            process.exit(1);
        }
        this.axios = axios.create({
            // baseURL: 'https://httpbin.org/post',
            baseURL: `https://api.crowdin.com/api/project/${projectId}`,
            params: {
                key: apiKey,
                json: true,
            },
        });

        this.axios.interceptors.response.use(null, error => {
            if (error.response && error.response.data && error.response.data.error && error.response.data.error) {
                console.error(
                    error.response.data.error.message + '\n'
                    + '\t' + error.config.url + '\n'
                    + '\t' + JSON.stringify(error.config.params));
            }
            return Promise.reject(error);
        });
    }

    async onRepositoryAdded() {
        this.createRepositoryDirectory();
        await this.addEnglishFiles();
        await this.addAllLocalizations();
    }

    get crowdinDirectoryName() {
        const { owner, repo } = this.repository;
        return `${repo} (${owner})`;
    }

    getCrowdinFilePath(filePath) {
        const directoryName = this.crowdinDirectoryName;
        // todo handle other case
        assert(filePath.endsWith('.cfg'));
        const fileName = path.basename(filePath).replace('.cfg', '.ini');
        return `${directoryName}/${fileName}`;
    }

    async createRepositoryDirectory() {
        try {
            const params = {
                name: this.crowdinDirectoryName,
                recursive: 1,
            };
            await this.axios.post('/add-directory', null, { params });
        } catch (error) {
            if (error.response && error.response.data && error.response.data.error && error.response.data.error.code === 50) {
                // todo uncomment
                // throw new Error('[crowdin] directory already exists');
                // todo handle error (merge folders or something else)
            } else {
                throw error;
            }
        }
    }

    async addEnglishFiles() {
        for (const filePath of this.repository.getEnglishFiles()) {
            await this.addEnglishFile(filePath);
        }
    }

    async postLocalizationFile(urlPath, filePath, params = {}) {
        const form = new FormData();
        const crowdinFilePath = this.getCrowdinFilePath(filePath);
        console.log(`Upload file, ${urlPath}, ${crowdinFilePath}`);
        form.append(`files[${crowdinFilePath}]`, fs.createReadStream(filePath));
        const headers = form.getHeaders();
        await this.axios.post(urlPath, form, { headers, params });
    }

    async addEnglishFile(filePath) {
        await this.postLocalizationFile('/add-file', filePath);
    }

    async addAllLocalizations() {
        const localizations = this.repository.getLocalizations();
        for (const localization of Object.entries(localizations)) {
            await this.addLocalization(localization);
        }
    }

    async addLocalization(localization) {
        const [languageCode, filesPaths] = localization;
        const params = { language: languageCode, auto_approve_imported: 1 };
        for (const filePath of filesPaths) {
            await this.postLocalizationFile('/upload-translation', filePath, params);
        }
    }
}

export default Crowdin;