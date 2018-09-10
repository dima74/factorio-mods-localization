import createApp from 'github-app';
import download from 'download';
import process from 'process';
import path from 'path';
import { ROOT } from './constants';
import uuid from 'uuid';
import Repository from './repository';

class GitHub {
    async init() {
        const id = process.env.GITHUB_APP_ID;
        const cert = process.env.GITHUB_APP_PRIVATE_KEY.replace(/\\n/g, '\n');

        if (!id || !cert) {
            console.error('Environment variables GITHUB_APP_ID and GITHUB_APP_PRIVATE_KEY must be set');
            process.exit(1);
        }

        this.apiHelper = createApp({ id, cert });
        // this.apiHelper = createApp({id: 13052, cert: fs.readFileSync('private/private-key.pem')});
        this.api = await this.apiHelper.asApp();
    }

    async getInstallation(id) {
        const installationApi = await this.apiHelper.asInstallation(id);
        return new Installation(id, installationApi);
    }

    async getInstallationRepositories(id) {
        const installation = await this.getInstallation(id);
        return await installation.getRepositories();
    }

    async getAllRepositories() /* [{installationId, owner, repo}...] */ {
        // todo pagination
        const installations = (await this.api.apps.getInstallations({ per_page: 100 })).data;
        const installationsRepositoriesPromises = installations.map(installation => this.getInstallationRepositories(installation.id));
        const installationsRepositories = await Promise.all(installationsRepositoriesPromises);
        return [].concat(...installationsRepositories);
    }


}

class Installation {
    constructor(id, api) {
        this.id = id;
        this.api = api;
    }

    async downloadRepository(fullName) {
        const [owner, repo] = fullName.split('/');
        if (process.env.NODE_ENV === 'development' && fullName === 'dima74/factorio-mod-example') {
            return new Repository({ owner, repo }, path.join(ROOT, '../../factorio-mod-example'));
        }

        const defaultBranch = await this.getDefaultBranch({ owner, repo });
        const archiveUrl = `https://github.com/${owner}/${repo}/archive/${defaultBranch}.zip`;
        // await download(archiveUrl, ROOT, { extract: true, strip: 1, mode: '666', headers: { accept: 'application/zip' } });
        const destinationDirectory = path.join(ROOT, uuid.v4());
        await download(archiveUrl, destinationDirectory, { extract: true, strip: 1, mode: '666', headers: { accept: 'application/zip' } });
        return new Repository({ owner, repo }, destinationDirectory);
    }

    async getDefaultBranch({ owner, repo }) {
        return (await this.api.repos.get({ owner, repo })).data.default_branch;
    }

    async getRepositories() {
        const response = await this.api.apps.getInstallationRepositories({ per_page: 100 });
        const repositories = response.data.repositories;
        return repositories.map(({ full_name }) => {
            const [owner, repo] = full_name.split('/');
            return { installationId: this.id, owner, repo };
        });
    }
}

const github = new GitHub();
export default github;
