import assert from 'assert';
import Koa from 'koa';
import Router from 'koa-router';
import bodyParser from 'koa-bodyparser';
import main from './main.js';
import database from './database.js';
import { IS_DEVELOPMENT } from './constants.js';
import crowdinApi, { getCrowdinDirectoryName } from './crowdin.js';
import github from './github.js';
import Sentry from '@sentry/node';
import crypto from 'crypto';
import { getLogsAll, getRepositoryLogs } from './console-log-interceptor.js';
import { handleReject } from './base.js';

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

        this.router.get('/', this.getMainPage.bind(this));
        this.router.get('/updates', this.getUpdates.bind(this));
        this.router.get('/logs/:fullName*', this.getRepositoryLogs.bind(this));
        this.router.get('/logsAll', authMiddleware, this.getLogsAll.bind(this));
        this.router.get('/triggerUpdate', this.triggerUpdate.bind(this));
        this.router.get('/updateCrowdinEnglishFiles', authMiddleware, this.updateCrowdinEnglishFiles.bind(this));
        this.router.get('/deleteCrowdinExampleDirectory', authMiddleware, this.deleteCrowdinExampleDirectory.bind(this));
        this.router.get('/repositories', authMiddleware, this.getRepositories.bind(this));
        this.initExampleRoutes();
    }

    // for debug only
    initExampleRoutes() {
        this.router.get('/error1', async (ctx) => {
            console.log('/error1');
            throw Error('Example error 1');
        });
        this.router.get('/error2', async (ctx) => {
            console.log('/error2: before sleep');
            const sleep = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
            await sleep(500);
            console.log('/error2: after sleep');
            throw Error('Example error 2');
        });
        this.router.get('/error3', async (ctx) => {
            // should not log anything and not send error to sentry
            ctx.throw(403, '/error3');
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
        this.router.post('/webhook', bodyParser(), this.onWebhook.bind(this));
    }

    async onWebhook(ctx) {
        const data = ctx.request.body;
        const secret = process.env.WEBHOOKS_SECRET;
        const signatureExpected = 'sha1=' + crypto.createHmac('sha1', secret).update(JSON.stringify(data)).digest('hex');
        const signatureReceived = ctx.get('X-Hub-Signature');
        const isSignatureValid = crypto.timingSafeEqual(new Buffer(signatureReceived), new Buffer(signatureExpected));
        if (!isSignatureValid) ctx.throw(403, '[github-webhook] Failed to verify signature');

        const webhookName = ctx.get('X-GitHub-Event');
        this.handleWebhook(webhookName, data).catch(handleReject);
        ctx.status = 204;
    }

    async handleWebhook(name, data) {
        function checkRepositorySelection(data) {
            const repositorySelection = data.installation.repository_selection;
            if (repositorySelection !== 'selected') {
                const owner = data.installation.account.login;
                throw Error(`[owner:${owner}] repository_selection=${repositorySelection} is currently unsupported`);
            }
        }

        switch (name) {
            case 'installation':
                if (data.action === 'deleted') break;  // TODO what to do when user removes app?
                checkRepositorySelection(data);
                await main.onRepositoriesAddedWebhook(data.installation.id, data.repositories);
                break;
            case 'installation_repositories':
                checkRepositorySelection(data);
                await main.onRepositoriesAddedWebhook(data.installation.id, data.repositories_added);
                break;
            case 'push':
                await main.onPushWebhook(data);
        }
    }

    // all next webhooks are for debug

    async getMainPage(ctx) {
        ctx.body = '<p>Factorio mods localization</p><p>See <a href="https://github.com/dima74/factorio-mods-localization">GitHub repository</a> for documentation</p>';
    }

    async getUpdates(ctx) {
        ctx.body = await database.getUpdatesInfo();
    }

    async triggerUpdate(ctx) {
        const repo = ctx.query.repo;
        if (repo) {
            await this.triggerUpdateSingle(ctx, repo);
        } else {
            await this.triggerUpdateAll(ctx);
        }
    }

    async triggerUpdateSingle(ctx, repo) {
        try {
            ctx.body = await main.pushRepositoryCrowdinChangesToGithub(repo);
        } catch (e) {
            handleReject(e);
            ctx.body = 'Error during update, see logs for details';
        }
    }

    async triggerUpdateAll(ctx) {
        ctx.body = 'Triggered. See logs for details.';
        main.pushAllCrowdinChangesToGithub().catch(handleReject);
    }

    /**
     * Emulates reinstalling of GitHub app for all repositories.
     * This updates Crowdin based on GitHub english files.
     * Needed when the app was offline long time, so we miss commits webhooks.
     */
    async updateCrowdinEnglishFiles(ctx) {
        const repositories = await github.getAllRepositories();
        ctx.body = 'Triggered. See logs for details.';
        // no await since it can be longer then 30 seconds request timeout
        this.updateCrowdinEnglishFilesImpl(repositories).catch(handleReject)
    }

    async updateCrowdinEnglishFilesImpl(repositories) {
        for (const { fullName, installation } of repositories) {
            await main.updateCrowdinEnglishFiles(installation, fullName).catch(handleReject);
        }
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

    async getLogsAll(ctx) {
        const logs = getLogsAll();
        ctx.body = logs.join('') || `There are no logs yet.`;
    }

    async getRepositoryLogs(ctx) {
        const fullName = ctx.params.fullName;
        if (fullName === undefined || fullName.length < 7 || fullName.split('/').length !== 2) {
            ctx.throw(403);
        }
        const logs = getRepositoryLogs(fullName);
        ctx.body = logs.join('') || `There are no logs for "${fullName}" yet.`;
    }
}

const webServer = new WebServer();
export default webServer;
