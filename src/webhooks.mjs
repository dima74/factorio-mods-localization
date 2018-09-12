import express from 'express';
import bodyParser from 'body-parser';
import GithubWebHook from 'express-github-webhook';
import main from './main';
import database from './database';

async function init() {
    const PORT = process.env.PORT || 5000;
    const app = express();
    app.get('/', (req, res) => res.send('It works!'));
    app.get('/updates', async (req, res, next) => {
        res.type('text/plain');
        res.send(await database.getUpdatesInfo());
        next();
    });
    app.listen(PORT, () => console.log(`Listening on ${ PORT }`));
    app.use(bodyParser.json());

    const webhookHandler = GithubWebHook({ path: '/webhook' });
    app.use(webhookHandler);

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

export default { init };
