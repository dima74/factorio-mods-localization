import assert from 'assert';
import express from 'express';
import bodyParser from 'body-parser';
import GithubWebHook from 'express-github-webhook';
import main from './main';
import database from './database';
import { IS_DEVELOPMENT } from './constants';
import crowdinApi, { getCrowdinDirectoryName } from './crowdin';
import github from './github';
import Raven from 'raven';
import { getRepositoryLogs } from './console-log-interceptor';

class WebServer {
    init() {
        const PORT = process.env.PORT || 5000;
        this.app = express();
        this.app.listen(PORT, () => console.log(`Listening on ${PORT}`));
        this.app.use(Raven.requestHandler());
        this.initRoutes();
        this.initWebhooks();
        this.app.use(Raven.errorHandler());
    }

    initRoutes() {
        assert(process.env.WEBSERVER_SECRET);
        const authMiddleware = this.authMiddleware;

        this.app.get('/', this.getMainPage);
        this.app.get('/updates', this.getUpdates);
        this.app.get('/logs/\*', this.getRepositoryLogs);
        this.app.get('/triggerUpdate', authMiddleware, this.triggerUpdate);
        this.app.get('/deleteCrowdinExampleDirectory', authMiddleware, this.deleteCrowdinExampleDirectory);
        this.app.get('/repositories', authMiddleware, this.getRepositories);
        this.initExampleRoutes();
    }

    // for debug only
    initExampleRoutes() {
        this.app.get('/error', (req, res) => {
            console.log('example console log');
            throw Error('Example error');
        });
        this.app.get('/errorAsync', async (req, res, next) => {
            console.log('before sleep');
            const sleep = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
            await sleep(500);
            console.log('after sleep');
            try {
                throw Error('Example async error');
            } catch (error) {
                next(error);
            }
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

    initWebhooks() {
        assert(process.env.WEBHOOKS_SECRET);
        const webhookHandler = GithubWebHook({ path: '/webhook', secret: process.env.WEBHOOKS_SECRET });
        this.app.use(bodyParser.json());
        this.app.use(webhookHandler);

        // webhookHandler.on('*', function (event, repo, data) {
        //     console.log(event, repo, data);
        // });

        function checkRepositorySelection(data) {
            const repositorySelection = data.installation.repository_selection;
            if (repositorySelection !== 'selected') {
                throw Error(`repository_selection=${repositorySelection} is currently unsupported`);
            }
        }

        webhookHandler.on('installation', (repo, data) => {
            checkRepositorySelection(data);
            main.onRepositoriesAddedWebhook(data.installation.id, data.repositories);
        });

        webhookHandler.on('installation_repositories', (repo, data) => {
            checkRepositorySelection(data);
            main.onRepositoriesAddedWebhook(data.installation.id, data.repositories_added);
        });

        webhookHandler.on('push', (repo, data) => {
            main.onPushWebhook(data);
        });
    }

    // all next webhooks are for debug

    getMainPage(req, res) {
        res.send('<p>Factorio mods localization</p><p>See <a href="https://github.com/dima74/factorio-mods-localization">GitHub repository</a> for documentation</p>');
    }

    async getUpdates(req, res, next) {
        res.type('text/plain');
        res.send(await database.getUpdatesInfo());
        next();
    }

    triggerUpdate(req, res) {
        res.type('text/plain').send('Triggered. See logs for details.');
        main.pushAllCrowdinChangesToGithub();
    }

    async deleteCrowdinExampleDirectory(req, res, next) {
        try {
            const crowdinName = getCrowdinDirectoryName('dima74/factorio-mod-example');
            await crowdinApi.deleteDirectory(crowdinName);
            res.type('text/plain').send('OK');
            next();
        } catch (error) {
            next(error);
        }
    }

    async getRepositories(req, res, next) {
        const repositories = await github.getAllRepositories();
        const response = repositories.map(({ installation, fullName }) => ({ installationId: installation.id, fullName }));
        res.type('text/plain').send(JSON.stringify(response, null, 2));
        next();
    }

    getRepositoryLogs(req, res) {
        const fullName = req.params[0];
        if (fullName.length < 7 || fullName.split('/') !== 2) {
            res.status(403).send('');
            return;
        }
        const logs = getRepositoryLogs(fullName);
        const response = logs.join('\n');
        res.type('text/plain').send(response);
    }
}

const webServer = new WebServer();
export default webServer;
