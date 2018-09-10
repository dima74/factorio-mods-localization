## App responsibilities
1. Webhook for app installation: create subfolder in crowdin
1. Every week: update github from crowdin
1. Webhook for every push to update crowdin from github

## Adding github app to repository
1. Download github repository (just files, without .git folder)
1. Check for `/locale` and `/locale/en`
1. Crowdin: 
    * api/add-directory
    * for each file in `/locale/en`: api/add-file
    * for each other `/locale` subfolder
        * for each subfolder file: api/upload-translation 

## Update github from crowdin (general)
1. Get list of all repositories for which github app is installed
1. Filter list to contain only repositories which also are presented on crowdin
1. Update github from crowdin (for each repository)

## Update github from crowdin (for each repository) 
1. Create installation token

    https://octokit.github.io/rest.js/#api-Apps-createInstallationToken

1. Clone repository `https://x-access-token:TOKEN@github.com/OWNER/REPO.git`

    https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/#http-based-git-access-by-an-installation

1. For each language:
    * download tar from crowdin
    * extract to cloned directory
1. Make git commit (or break if there are no changes)
1. Git push

## Notes
* We need to run some code every week (update github from crowdin). Standart Heroku scheduler mechanism is unreliable, costly or difficult to setup. But Heroku forces our app to restart approximately every 24 hours. So we just can run necessary code on app startup.

* Research: can we make our dyno not sleep if we will every minute send request from our app to itself (via https://factorio-mods-localization.herokuapp.com/)?

* Any localization folder (such as `/locale/en`, `/locale/ru`) may contain subfolder, and we should ignore subfolders, because factorio ignores them too. Here is [example](https://github.com/Karosieben/boblocale/tree/master/locale/en/old).

* In some mods files names doesn't match across localization folders

    Exampless:
    
    * `/locale/en/en.cfg` and `/locale/ru/ru.cfg`
    * `/locale/en/Angel.cfg` and `/locale/ru/Angel_ru.cfg`

    We researched >1000 mods and it turns out that only 8% of them has unmatched files names in different languages directories

    So for now we decided to support only matched files names in different languages (mod author has to rename languages files if their names don't match)