# Translate your Factorio mod easily with power of Crowdin

[![Website factorio-mods-localization.herokuapp.com](https://img.shields.io/website-up-down-green-red/https/factorio-mods-localization.herokuapp.com.svg)](https://factorio-mods-localization.herokuapp.com/)
[![GitHub license](https://img.shields.io/github/license/dima74/factorio-mods-localization.svg)](https://github.com/dima74/factorio-mods-localization/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/dima74/factorio-mods-localization.svg)](https://GitHub.com/dima74/factorio-mods-localization/issues/)
[![Dependencies](https://david-dm.org/dima74/factorio-mods-localization.png)](https://david-dm.org/dima74/factorio-mods-localization)
[![Heroku](https://heroku-badge.herokuapp.com/?app=factorio-mods-localization&svg=1)](https://factorio-mods-localization.herokuapp.com/)

## Description
We provide service for simplify [Factorio](https://www.factorio.com/) mods translation. You only need to install [our GitHub app][1]. After this the following actions will be performed automatically:

* All existing english strings of your mod will be uploaded to [Crowdin](https://crowdin.com/)
* All existing translations will be uploaded too
* Every week our [FactorioBot](https://github.com/factorio-mods-helper) will fetch translations updates from Crowdin and commit them to your repository

## Motivation
There are a lot of Factorio mods hosted on GitHub. Most of them are translated using pull requests. I think that it is not very convenient (because it is not clear which strings are untranslated yet and translators have to know how to use git). So I decided to create helper tool for configuring translation process on Crowdin, powerful localization platform.

## Installation
1. Go to our [GitHub app page][1]
2. Click install button
3. Choose repository with your Factorio mod
4. Click install button

You are done! Now share link to [this Crowdin project][2] with translators.

Please note that **only Crowdin should be used for translation**.  GitHub pull requests must not be used for translation, otherwise translations will be lost after next synchronization from Crowdin! Consider adding link to [Crowdin][2] to your repository Readme ([example](https://github.com/softmix/AutoDeconstruct/pull/6/files)).

## How to translate using Crowdin
We have single Crowdin project. It consists of several folders, each folder corresponds to one mod. So, here are instructions how to translate specific mod:

1. Go to [Crowdin project page][2]
2. Select language
3. Find folder with your mod
4. Open menu (click to three points) right of the folder name
5. Click "Open in Editor": ![menu](https://user-images.githubusercontent.com/6505554/85887708-bdfa5880-b801-11ea-99c1-766ad92ae4af.png)

Then Crowdin translation interface will be opened where you can translate strings.

## Notes

* In order to correctly upload your existing translations to Crowdin, files in any localization folder (such as `/locale/de`) **must have same names as files in `/locale/en` folder**.
* Please ask any questions or report bugs by creating new [issue](https://github.com/dima74/factorio-mods-localization/issues).

## Detailed description of how it works
0. Mod author has mod repository on GitHub
1. Mod author installs GitHub app (for mod repository)
2. Our service creates subdirectory in our Crowdin project and upload original english strings and existing translations into it
3. Every week our service takes translated strings from Crowdin and make commit to GitHub repository (if there are any changes)
4. Every time original (locale/en) strings are changed, our service changes appropriate strings on Crowdin 


  [1]: https://github.com/apps/factorio-mods-localization-helper
  [2]: https://crowdin.com/project/factorio-mods-localization
