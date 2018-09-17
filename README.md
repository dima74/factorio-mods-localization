# Translate your Factorio mod easily with power of Crowdin

## Description
We provide service for simplify [Factorio](https://www.factorio.com/) mods translation. You only need to install [our GitHub app][1]. After this the following actions will be performed automatically:

* All existing english strings of your mod will be uploaded to [Crowdin](https://crowdin.com/)
* All existing translations will be uploaded too
* Every week our [FactorioBot](https://github.com/factorio-mods-helper) will fetch translations updates from Crowdin and commit them to your repository

## Motivation
TODO

## Installation
1. Go to our [GitHub app page][1]
2. Click install button
3. Choose repository with your factorio mod
4. Click install button

You are done! Now share link to [this Crowdin project](https://crowdin.com/project/factorio-mods-localization) with translators

## Usage as translator
TODO

## Detail description of how it works
0. Mod author has mod repository on GitHub
1. Mod author installs GitHub app (for his mod repository)
2. Our service creates subdirectory in our Crowdin project and upload original strings and existing translations into it
3. Every week our service take strings from Crowdin and make commit to GitHub repository (if there are any changes)
4. Every time original (locale/en) strings are changed, our service changes appropriate strings on Crowdin 


  [1]: https://github.com/apps/factorio-mods-localization-helper