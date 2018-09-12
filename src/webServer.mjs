import assert from 'assert';
import express from 'express';
import bodyParser from 'body-parser';
import GithubWebHook from 'express-github-webhook';
import main from './main';
import database from './database';
import { IS_DEVELOPMENT } from './constants';
import crowdinApi, { getCrowdinDirectoryName } from './crowdin';

class WebServer {
    init() {
        const PORT = process.env.PORT || 5000;
        this.app = express();
        this.app.listen(PORT, () => console.log(`Listening on ${ PORT }`));
        this.initRoutes();
        this.initWebhooks();
    }

    initRoutes() {
        assert(process.env.WEBSERVER_SECRET);
        const authMiddleware = this.authMiddleware;

        this.app.get('/', (req, res) => res.send('It works!'));
        this.app.get('/updates', this.getUpdates);
        this.app.get('/triggerUpdate', authMiddleware, this.triggerUpdate);
        this.app.get('/deleteCrowdinExampleDirectory', authMiddleware, this.deleteCrowdinExampleDirectory);
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

        webhookHandler.on('installation_repositories', (repo, data) => {
            main.onRepositoriesAddedWebhook(data.installation.id, data.repositories_added);
        });

        webhookHandler.on('push', (repo, data) => {
            main.onPushWebhook(data);
        });
    }

    // all next webhooks are for debug

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
}

const webServer = new WebServer();
export default webServer;
