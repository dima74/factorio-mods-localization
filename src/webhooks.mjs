import express from 'express';
import bodyParser from 'body-parser';
import main from './main';
import GithubWebHook from 'express-github-webhook';

async function init() {
    const PORT = process.env.PORT || 5000;
    const app = express();
    app.get('/', (req, res) => res.send('It works!'));
    app.listen(PORT, () => console.log(`Listening on ${ PORT }`));
    app.use(bodyParser.json());

    const webhookHandler = GithubWebHook({path: '/webhook'});
    app.use(webhookHandler);

    // webhookHandler.on('*', function (event, repo, data) {
    //     console.log(event, repo, data);
    // });

    function onInstallationRepositories(repo, data) {
        main.onRepositoriesAdded(data.installation.id, data.repositories_added);
    }

    webhookHandler.on('installation_repositories', onInstallationRepositories);
}

export default {init};
