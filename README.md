# Translate your Factorio mod easily with power of Crowdin

[![Website factorio-mods-localization.herokuapp.com](https://img.shields.io/website-up-down-green-red/https/factorio-mods-localization.herokuapp.com.svg)](https://factorio-mods-localization.herokuapp.com/)
[![GitHub license](https://img.shields.io/github/license/dima74/factorio-mods-localization.svg)](https://github.com/dima74/factorio-mods-localization/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/dima74/factorio-mods-localization.svg)](https://GitHub.com/dima74/factorio-mods-localization/issues/)
[![Dependencies](https://david-dm.org/dima74/factorio-mods-localization.png)](https://david-dm.org/dima74/factorio-mods-localization)
[![Heroku](https://heroku-badge.herokuapp.com/?app=factorio-mods-localization)](https://factorio-mods-localization.herokuapp.com/)

## Description
We provide service for simplify [Factorio](https://www.factorio.com/) mods translation. You only need to install [our GitHub app][1]. After this the following actions will be performed automatically:

* All existing english strings of your mod will be uploaded to [Crowdin](https://crowdin.com/)
* All existing translations will be uploaded too
* Every week our [FactorioBot](https://github.com/factorio-mods-helper) will fetch translations updates from Crowdin and commit them to your repository

## Motivation
There are a lot of Factorio mods hosted on GitHub. Most of them are translated using pull requests. I think that it is not very convenient (because it is not clear which strings are untranslated yet and also translator have to know how to use git). So I decide to create helper tool for configuring translation process on Crowdin, powerful localization platform.

## Installation
1. Go to our [GitHub app page][1]
2. Click install button
3. Choose repository with your Factorio mod
4. Click install button

You are done! Now share link to [this Crowdin project][2] with translators

## How to translate using Crowdin
We have single Crowdin project. It consists of several folders, each folder corresponds to one mod. So, here is instruction how to translate specific mod:

1. Go to [Crowdin project page][2]
2. Select language
3. Find folder with your mod
4. Open menu (click to three points) right of the folder namae
5. Click "Translate"

Then Crowdin translation interface will be opened where you can translate strings.

## Detail description of how it works
0. Mod author has mod repository on GitHub
1. Mod author installs GitHub app (for his mod repository)
2. Our service creates subdirectory in our Crowdin project and upload original strings and existing translations into it
3. Every week our service take strings from Crowdin and make commit to GitHub repository (if there are any changes)
4. Every time original (locale/en) strings are changed, our service changes appropriate strings on Crowdin 


  [1]: https://github.com/apps/factorio-mods-localization-helper
  [2]: https://crowdin.com/project/factorio-mods-localization