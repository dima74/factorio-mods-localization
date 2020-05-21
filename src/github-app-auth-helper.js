// some useful information in my pull request: https://github.com/probot/github-app/pull/16

import GitHubApi from '@octokit/rest';
import authApp from '@octokit/auth-app';

const { createAppAuth } = authApp;

export default function ({ id, privateKey, debug = false }) {
    const auth = createAppAuth({ id, privateKey });

    async function asApp() {
        const { token } = await auth({ type: 'app' });
        return new GitHubApi.Octokit({ auth: token, debug });
    }

    // Authenticate as the given installation
    async function asInstallation(installationId) {
        const { token } = await auth({ type: 'installation', installationId });
        return new GitHubApi.Octokit({ auth: token, debug });
    }

    return { asApp, asInstallation };
}
