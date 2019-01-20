// TODO delete this file if my pull request will be merged https://github.com/probot/github-app/pull/16

const jwt = require('jsonwebtoken')
const GitHubApi = require('@octokit/rest')

module.exports = function ({id, cert, debug = false}) {
    function asApp (jwtExpirationPeriodInSeconds = 60) {
        const github = new GitHubApi({debug})
        github.authenticate({type: 'app', token: generateJwt(id, cert, jwtExpirationPeriodInSeconds)})
        // Return a promise to keep API consistent
        return Promise.resolve(github)
    }

    // Authenticate as the given installation
    function asInstallation (installationId) {
        return createToken(installationId).then(res => {
            const github = new GitHubApi({debug})
            github.authenticate({type: 'app', token: res.data.token})
            return github
        })
    }

    // https://developer.github.com/early-access/integrations/authentication/#as-an-installation
    function createToken (installationId) {
        return asApp().then(github => {
            return github.apps.createInstallationToken({
                installation_id: installationId
            })
        })
    }

    // Internal - no need to expose this right now
    function generateJwt (id, cert, expirationPeriodInSeconds) {
        const payload = {
            iat: Math.floor(new Date() / 1000), // Issued at time
            exp: Math.floor(new Date() / 1000) + expirationPeriodInSeconds, // JWT expiration time
            iss: id // Integration's GitHub id
        }

        // Sign with RSA SHA256
        return jwt.sign(payload, cert, {algorithm: 'RS256'})
    }

    return {asApp, asInstallation, createToken}
}
