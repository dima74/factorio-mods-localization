import assert from 'assert';
import express from 'express';
import router from 'express-promise-router';
import bodyParser from 'body-parser';
import GithubWebHook from 'express-github-webhook';
import main from './main';
import database from './database';
import { IS_DEVELOPMENT } from './constants';
import crowdinApi, { getCrowdinDirectoryName } from './crowdin';
import github from './github';
import Raven from 'raven';
import { getRepositoryLogs } from './console-log-interceptor';
import { handleReject } from './base';

class WebServer {
    init() {
        const PORT = process.env.PORT || 5000;
        const app = express();
        app.listen(PORT, () => console.log(`Listening on ${PORT}`));
        this.router = router();
        this.router.use(Raven.requestHandler());
        this.initRoutes();
        this.router.use(Raven.errorHandler());
        app.use('/webhook', this.getWebhookRouter());
        app.use('/', this.router);
    }

    initRoutes() {
        assert(process.env.WEBSERVER_SECRET);
        const authMiddleware = this.authMiddleware;

        this.router.get('/', this.getMainPage);
        this.router.get('/updates', this.getUpdates);
        this.router.get('/logs/\*', this.getRepositoryLogs);
        this.router.get('/triggerUpdate', authMiddleware, this.triggerUpdate);
        this.router.get('/deleteCrowdinExampleDirectory', authMiddleware, this.deleteCrowdinExampleDirectory);
        this.router.get('/repositories', authMiddleware, this.getRepositories);
        this.initExampleRoutes();
    }

    // for debug only
    initExampleRoutes() {
        this.router.get('/error', (req, res) => {
            console.log('example console log');
            throw Error('Example error');
        });
        this.router.get('/errorAsync', async (req, res) => {
            console.log('before sleep');
            const sleep = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
            await sleep(500);
            console.log('after sleep');
            throw Error('Example async error');
        });
        this.router.get('/errorAsync2', (req, res) => {
            Promise.reject(Error('Example async2 error')).catch(handleReject);
            res.send();
        });
    }

    authMiddleware(req, res, next) {
        if (req.query.secret === process.env.WEBSERVER_SECRET || IS_DEVELOPMENT) {
            next();
        } else {
            res.status(403);
            res.send('Error: missed or incorrect secret');
        }
    }

    getWebhookRouter() {
        assert(process.env.WEBHOOKS_SECRET);
        const router = express.Router();
        const webhookHandler = GithubWebHook({ path: '/', secret: process.env.WEBHOOKS_SECRET });
        router.use(bodyParser.json());
        router.use(webhookHandler);

        // webhookHandler.on('*', function (event, repo, data) {
        //     console.log(event, repo, data);
        // });

        function checkRepositorySelection(data) {
            const repositorySelection = data.installation.repository_selection;
            if (repositorySelection !== 'selected') {
                const owner = data.installation.account.login;
                throw Error(`[owner:${owner}] repository_selection=${repositorySelection} is currently unsupported`);
            }
        }

        webhookHandler.on('installation', (repo, data) => {
            checkRepositorySelection(data);
            main.onRepositoriesAddedWebhook(data.installation.id, data.repositories).catch(handleReject);
        });

        webhookHandler.on('installation_repositories', (repo, data) => {
            checkRepositorySelection(data);
            main.onRepositoriesAddedWebhook(data.installation.id, data.repositories_added).catch(handleReject);
        });

        webhookHandler.on('push', (repo, data) => {
            main.onPushWebhook(data).catch(handleReject);
        });

        return router;
    }

    // all next webhooks are for debug

    getMainPage(req, res) {
        res.send('<p>Factorio mods localization</p><p>See <a href="https://github.com/dima74/factorio-mods-localization">GitHub repository</a> for documentation</p>');
    }

    async getUpdates(req, res) {
        res.type('text/plain');
        res.send(await database.getUpdatesInfo());
    }

    triggerUpdate(req, res) {
        res.type('text/plain').send('Triggered. See logs for details.');
        main.pushAllCrowdinChangesToGithub().catch(handleReject);
    }

    async deleteCrowdinExampleDirectory(req, res) {
        const crowdinName = getCrowdinDirectoryName('dima74/factorio-mod-example');
        await crowdinApi.deleteDirectory(crowdinName);
        res.type('text/plain').send('OK');
    }

    async getRepositories(req, res) {
        const repositories = await github.getAllRepositories();
        const response = repositories.map(({ installation, fullName }) => ({ installationId: installation.id, fullName }));
        res.type('text/plain').send(JSON.stringify(response, null, 2));
    }

    getRepositoryLogs(req, res) {
        const fullName = req.params[0];
        if (fullName.length < 7 || fullName.split('/').length !== 2) {
            res.status(403).send('');
            return;
        }
        const logs = getRepositoryLogs(fullName);
        const response = logs.join('') || `There are no logs for "${fullName}" yet.`;
        res.type('text/plain').send(response);
    }
}

const webServer = new WebServer();
export default webServer;
