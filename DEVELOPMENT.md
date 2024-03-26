## Webserver routes
* `/` - Main page with link to GitHub repository
* `/webhook` - Github app webhooks handler
* `/triggerUpdate?secret=X` - Update all repositories
* `/triggerUpdate?secret=X&repo=REPO` - Update specific repository
* `/importRepository?secret=X&repo=REPO` - Readd repository to Crowdin (both english files and translations)
* `/importEnglish?secret=X&repo=REPO` - Overwrites english files on Crowdin based on GitHub


## fly.io configuration
* First time
```sh
yay -S flyctl-bin
fly secrets set KEY=VALUE
fly launch
```

* Deploy from local
```sh
fly deploy --image-label $(git rev-parse HEAD)
```

* Deploy from github
  https://fly.io/docs/app-guides/continuous-deployment-with-github-actions/
  See .github/workflows/fly.yml
  `FLY_API_TOKEN` - get using `fly tokens create deploy -x 999999h` 

* Get git commit hash for current release
```sh
fly image show  # column TAG
```


## Needed environment variables
From https://github.com/settings/apps/factorio-mods-localization-helper:
* `GITHUB_APP_ID` - App ID
* `GITHUB_APP_PRIVATE_KEY` - Private keys (convert pem file content to one line by replacing newlines with \n)
* `GITHUB_APP_WEBHOOKS_SECRET` - Webhook secret

From https://crowdin.com/project/factorio-mods-localization/tools/api:
* `CROWDIN_PROJECT_ID`
* `CROWDIN_API_KEY`

From https://github.com/settings/tokens for https://github.com/factorio-mods-helper:
* `GITHUB_PERSONAL_ACCESS_TOKEN` - classic token with `repo:public_repo` scope

From https://diralik.sentry.io/settings/projects/factorio-mods-localization/keys/
* SENTRY_DSN - `https://...@....ingest.sentry.io/...`

* `GIT_COMMIT_USER_NAME` - "Factorio Mods Helper"
* `GIT_COMMIT_USER_EMAIL` - Email of https://github.com/factorio-mods-helper
* `GIT_COMMIT_MESSAGE` - "Update translations from Crowdin"

* `RUST_LOG` - "fml=info"
* `WEBSERVER_SECRET` - any string for `/triggerUpdate` route


## GitHub Apps
Main
id: 13052
private key: MIIEog...

Fml-test
id: 97456
private key: MIIEow...

## Crowdin projects
Main id: 307377
Fml-test id: 613717
Api key for both: 2ad62d...


## Weekly updates from crowdin to github
https://cron-job.org/


## .cfg/.ini files format
Factorio uses .cfg extension, Crowdin uses .ini extension, so we just change the extension when needed

General format is (https://wiki.factorio.com/Tutorial:Localisation):
```ini
[section]
key=value
```

Notes:
* `key=foo # bar` - Crowdin will export it as `key="foo # bar"`, I wrote to support to disable quoting 
* `key=foo ; bar` - Crowdin will threat `; bar` as comment, we need to wrap value in quotes `"` before uploading english file to Crowdin
* Multiline english strings are not allowed
* If translated string has new lines, e.g.:
  ```
  line1
  line2
  line3
  ```
  then Crowdin will export it in format
  ```
  key=line1
  line2=
  line3=
  ```
  Probably our helper should replace it to `key=line1\nline2\nline3`
