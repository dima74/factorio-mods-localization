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