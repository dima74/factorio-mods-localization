import axios from 'axios';
import fs from 'fs';
import path from 'path';
import FormData from 'form-data';
import assert from 'assert';

class Crowdin {
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

    getCrowdinDirectoryName({ owner, repo }) {
        return `${repo} (${owner})`;
    }

    getCrowdinFilePath(repository, filePath) {
        const directoryName = this.getCrowdinDirectoryName(repository);
        // todo handle other case
        assert(filePath.endsWith('.cfg'));
        const fileName = path.basename(filePath).replace('.cfg', '.ini');
        return `${directoryName}/${fileName}`;
    }

    async createRepositoryDirectory(repository) {
        const name = this.getCrowdinDirectoryName(repository);
        try {
            await this.axios.post('/add-directory', null, { params: { name, recursive: 1 } });
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

    async addEnglishFiles(repository) {
        for (const filePath of repository.listEnglishFiles()) {
            await this.addEnglishFile(repository, filePath);
        }
    }

    async addEnglishFile(repository, filePath) {
        const form = new FormData();
        const crowdinFilePath = this.getCrowdinFilePath(repository, filePath);
        form.append(`files[${crowdinFilePath}]`, fs.createReadStream(filePath));
        const headers = form.getHeaders();
        const params = { type: 'ini' };
        const response = await this.axios.post('/add-file', form, { headers, params });
    }
}

const crowdin = new Crowdin();
export default crowdin;