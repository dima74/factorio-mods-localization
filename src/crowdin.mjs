import axios from 'axios';
import fs from 'fs';
import path from 'path';
import FormData from 'form-data';

function getCrowdinDirectoryName({ owner, repo }) {
    return `${repo} (${owner})`;
}

class CrowdinApi {
    constructor() {
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

    getCrowdinDirectory(repository) {
        return new CrowdinDirectory(this.axios, repository);
    }

    async getAllDirectoriesNames() {
        const info = (await this.axios.post('/info')).data;
        return info.files.map(directory => directory.name);
    }

    async filterRepositories(repositories) {
        const directoriesNames = await this.getAllDirectoriesNames();
        return repositories.filter(repository => directoriesNames.includes(getCrowdinDirectoryName(repository)));
    }

    // for debug
    async deleteAllDirectories() {
        for (const name of await this.getAllDirectoriesNames()) {
            const params = { name };
            await this.axios.post('/delete-directory', null, { params });
        }
    }
}

class CrowdinDirectory {
    constructor(axios, repository) {
        this.axios = axios;
        this.repository = repository;
        this.directoryName = getCrowdinDirectoryName(repository);
    }

    async onRepositoryAdded() {
        await this.createRepositoryDirectory();
        await this.addEnglishFiles();
        await this.addAllLocalizations();
    }

    getCrowdinFileInfo(filePath) {
        const fileName = path.basename(filePath).replace('.cfg', '.ini');
        return [`${this.directoryName}/${fileName}`, fileName];
    }

    async createRepositoryDirectory() {
        try {
            const params = {
                name: this.directoryName,
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
        const [crowdinFilePath, crowdinFileName] = this.getCrowdinFileInfo(filePath);
        console.log(`Upload file, ${urlPath}, ${crowdinFilePath}`);
        form.append(`files[${crowdinFilePath}]`, fs.createReadStream(filePath), crowdinFileName);
        const headers = form.getHeaders();
        return await this.axios.post(urlPath, form, { headers, params });
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
        for (const filePath of filesPaths) {
            if (!filePath.endsWith('.cfg') && !filePath.endsWith('.ini')) {
                console.warn('Locale file with unknown extension:', path.basename(filePath));
                continue;
            }
            await this.addTranslatedFile(languageCode, filePath);
        }
    }

    async addTranslatedFile(languageCode, filePath) {
        const params = { language: languageCode, auto_approve_imported: 1 };
        const response = await this.postLocalizationFile('/upload-translation', filePath, params);

        // check that all files have status 'uploaded'
        for (const [fileName, fileStatus] of Object.entries(response.data.files)) {
            if (fileStatus !== 'uploaded') {
                throw new Error(`Error during uploading file "${fileName}", status: ${fileStatus}`);
            }
        }
    }
}

const crowdinApi = new CrowdinApi();
export default crowdinApi;