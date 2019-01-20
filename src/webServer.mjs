import assert from 'assert';
import Koa from 'koa';
import Router from 'koa-router';
import bodyParser from 'koa-bodyparser';
import main from './main';
import database from './database';
import { IS_DEVELOPMENT } from './constants';
import crowdinApi, { getCrowdinDirectoryName } from './crowdin';
import github from './github';
import Sentry from '@sentry/node';
import crypto from 'crypto';
import { getRepositoryLogs } from './console-log-interceptor';
import { handleReject } from './base';

class WebServer {
    init() {
        this.router = new Router();
        this.initRoutes();
        this.initWebhooks();

        const app = new Koa();
        app
            .use(this.router.routes())
            .use(this.router.allowedMethods());
        app.on('error', err => Sentry.captureException(err));
        const PORT = process.env.PORT || 5000;
        app.listen(PORT, () => console.log(`Listening on ${PORT}`));
    }

    initRoutes() {
        assert(process.env.WEBSERVER_SECRET);
        const authMiddleware = this.authMiddleware;

        this.router.get('/', this.getMainPage);
        this.router.get('/updates', this.getUpdates);
        this.router.get('/logs/:fullName*', this.getRepositoryLogs);
        this.router.get('/triggerUpdate', authMiddleware, this.triggerUpdate);
        this.router.get('/deleteCrowdinExampleDirectory', authMiddleware, this.deleteCrowdinExampleDirectory);
        this.router.get('/repositories', authMiddleware, this.getRepositories);
        this.initExampleRoutes();
    }

    // for debug only
    initExampleRoutes() {
        this.router.get('/error1', async (ctx) => {
            console.log('/error1');
            throw Error('Example error 1');
        });
        this.router.get('/error2', async (ctx) => {
            console.log('before sleep');
            const sleep = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
            await sleep(500);
            console.log('after sleep');
            throw Error('Example error 2');
        });
        this.router.get('/error3', async (ctx) => {
            Promise.reject(Error('Example error 3')).catch(handleReject);
        });
    }

    async authMiddleware(ctx, next) {
        if (ctx.query.secret === process.env.WEBSERVER_SECRET || IS_DEVELOPMENT) {
            await next();
        } else {
            ctx.throw(403, 'Error: missed or incorrect secret');
        }
    }

    initWebhooks() {
        assert(process.env.WEBHOOKS_SECRET);
        this.router.post('/webhook', bodyParser(), this.onWebhook);
    }

    async onWebhook(ctx) {
        const data = ctx.request.body;
        const secret = process.env.WEBHOOKS_SECRET;
        const signatureExpected = 'sha1=' + crypto.createHmac('sha1', secret).update(JSON.stringify(data)).digest('hex');
        const signatureReceived = ctx.get('X-Hub-Signature');
        const isSignatureValid = crypto.timingSafeEqual(new Buffer(signatureReceived), new Buffer(signatureExpected));
        if (!isSignatureValid) ctx.throw(403, '[github-webhook] Failed to verify signature');

        function checkRepositorySelection(data) {
            const repositorySelection = data.installation.repository_selection;
            if (repositorySelection !== 'selected') {
                const owner = data.installation.account.login;
                throw Error(`[owner:${owner}] repository_selection=${repositorySelection} is currently unsupported`);
            }
        }

        const webhookName = ctx.get('X-GitHub-Event');
        switch (webhookName) {
            case 'installation':
                checkRepositorySelection(data);
                main.onRepositoriesAddedWebhook(data.installation.id, data.repositories).catch(handleReject);
                break;
            case 'installation_repositories':
                checkRepositorySelection(data);
                main.onRepositoriesAddedWebhook(data.installation.id, data.repositories_added).catch(handleReject);
                break;
            case 'push':
                main.onPushWebhook(data).catch(handleReject);
        }
        ctx.status = 204;
    }

    // all next webhooks are for debug

    async getMainPage(ctx) {
        ctx.body = '<p>Factorio mods localization</p><p>See <a href="https://github.com/dima74/factorio-mods-localization">GitHub repository</a> for documentation</p>';
    }

    async getUpdates(ctx) {
        ctx.body = await database.getUpdatesInfo();
    }

    async triggerUpdate(ctx) {
        ctx.body = 'Triggered. See logs for details.';
        main.pushAllCrowdinChangesToGithub().catch(handleReject);
    }

    async deleteCrowdinExampleDirectory(ctx) {
        const crowdinName = getCrowdinDirectoryName('dima74/factorio-mod-example');
        await crowdinApi.deleteDirectory(crowdinName);
        ctx.body = 'OK';
    }

    async getRepositories(ctx) {
        const repositories = await github.getAllRepositories();
        const response = repositories.map(({ installation, fullName }) => ({ installationId: installation.id, fullName }));
        ctx.body = JSON.stringify(response, null, 2);
    }

    async getRepositoryLogs(ctx) {
        const fullName = ctx.params.fullName;
        if (fullName.length < 7 || fullName.split('/').length !== 2) {
            ctx.throw(403);
        }
        const logs = getRepositoryLogs(fullName);
        ctx.body = logs.join('') || `There are no logs for "${fullName}" yet.`;
    }
}

const webServer = new WebServer();
export default webServer;
