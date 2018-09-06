import createApp from 'github-app';
import download from 'download';
import process from 'process';
import fs from 'fs';
import {ROOT} from './constants';

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
        const cert = process.env.GITHUB_APP_PRIVATE_KEY.replace(/\\n/g, '\n');
        const api = createApp({id, cert});
        // this.api = createApp({id: 13052, cert: fs.readFileSync('private/private-key.pem')});
        this.app = await api.asApp();
    }

    async downloadRepository({owner, repo}) {
        const defaultBranch = await this.getDefaultBranch({owner, repo});
        const archiveUrl = `https://github.com/${owner}/${repo}/archive/${defaultBranch}.zip`;
        console.log(archiveUrl);
        await download(archiveUrl, ROOT, {extract: true, strip: 1, mode: '666', headers: {accept: 'application/zip'}});
    }

    async getDefaultBranch({owner, repo}) {
        return (await this.app.repos.get({owner, repo})).default_branch;
    }
}

class Installation {
    constructor(github, id) {
        this.api = github.api.
    }
}

const app = new GitHub();
export default app;
