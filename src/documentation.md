## Actions after adding github app to repository
1. Download github repository (just files, without .git folder)
1. Check for `/locale` and `/locale/en`
1. Crowdin: 
    * api/add-directory
    * for each file in `/locale/en`: api/add-file
    * for each other `/locale` subfolder
        * for each subfolder file: api/upload-translation
1. Set up every week update github from crowdin
1. Set up webhook for every push to update crowdin from github

## Notes
* Any localization folder (such as `/locale/en`, `/locale/ru`) may contain subfolder, and we should ignore subfolders, because factorio ignores them too. Here is [example](https://github.com/Karosieben/boblocale/tree/master/locale/en/old).

* In some mods files names doesn't match across localization folders

    Exampless:
    
    * `/locale/en/en.cfg` and `/locale/ru/ru.cfg`
    * `/locale/en/Angel.cfg` and `/locale/ru/Angel_ru.cfg`

    We researched >1000 mods and it turns out that only 8% of them has unmatched files names in different languages directories

    So for now we decided to support only matched files names in different languages (mod author has to rename languages files if their names don't match)