import createApp from 'github-app';
import download from 'download';
import process from 'process';
import path from 'path';
import { ROOT } from './constants';
import uuid from 'uuid';
import Repository from './repository';

// async function f(installationId) {
//     const api = await app.asInstallation(installationId);
//     const repositories = await api.apps.getInstallationRepositories({});
//     console.log(repositories.data.repositories);
// }
//
// (async function main() {
//     const api = await app.asApp();
//     const installations = await api.apps.getInstallations({});
//     await f(installations.data[0].id);
// })();

class GitHub {
    async init() {
        const id = process.env.GITHUB_APP_ID;
        const cert = process.env.GITHUB_APP_PRIVATE_KEY;

        if (!id || !cert) {
            console.error('Environment variables GITHUB_APP_ID and GITHUB_APP_PRIVATE_KEY must be set');
            process.exit(1);
        }

        this.apiHelper = createApp({ id, cert: cert.replace(/\\n/g, '\n') });
        // this.apiHelper = createApp({id: 13052, cert: fs.readFileSync('private/private-key.pem')});
        this.api = await this.apiHelper.asApp();
    }

    async getInstallation(id) {
        const installationApi = await this.apiHelper.asInstallation(id);
        return new Installation(installationApi);
    }
}

class Installation {
    constructor(api) {
        this.api = api;
    }

    async downloadRepository(fullName) {
        const [owner, repo] = fullName.split('/');
        if (process.env.NODE_ENV === 'development' && fullName === 'dima74/factorio-mod-example') {
            return new Repository({owner, repo}, path.join(ROOT, '../../factorio-mod-example'));
        }

        const defaultBranch = await this.getDefaultBranch({ owner, repo });
        const archiveUrl = `https://github.com/${owner}/${repo}/archive/${defaultBranch}.zip`;
        // await download(archiveUrl, ROOT, { extract: true, strip: 1, mode: '666', headers: { accept: 'application/zip' } });
        const destinationDirectory = path.join(ROOT, uuid.v4());
        await download(archiveUrl, destinationDirectory, { extract: true, strip: 1, mode: '666', headers: { accept: 'application/zip' } });
        return new Repository({owner, repo}, destinationDirectory);
    }

    async getDefaultBranch({ owner, repo }) {
        return (await this.api.repos.get({ owner, repo })).data.default_branch;
    }
}

const github = new GitHub();
export default github;
