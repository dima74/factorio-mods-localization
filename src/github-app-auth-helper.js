// some useful information in my pull request: https://github.com/probot/github-app/pull/16

import GitHubApi from '@octokit/rest';
import authApp from '@octokit/auth-app';

// fucking @octokit/auth-app
const { createAppAuth } = authApp;

export default function ({ id, privateKey, debug = false }) {
    const auth = createAppAuth({ id, privateKey });

    async function asApp() {
        const { token } = await auth({ type: 'app' });

        const github = new GitHubApi({ debug });
        github.authenticate({ type: 'app', token });
        return github;
    }

    // Authenticate as the given installation
    async function asInstallation(installationId) {
        const { token } = await auth({ type: 'installation', installationId });

        const github = new GitHubApi({ debug });
        github.authenticate({ type: 'app', token });
        return github;
    }

    return { asApp, asInstallation };
}
