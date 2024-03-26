# Translate your Factorio mod easily with the power of Crowdin

[![Crowdin](https://badges.crowdin.net/factorio-mods-localization/localized.svg)](https://crowdin.com/project/factorio-mods-localization)
[![Website factorio-mods-localization.fly.dev](https://img.shields.io/website-up-down-green-red/https/factorio-mods-localization.fly.dev.svg)](https://factorio-mods-localization.fly.dev/)
[![GitHub Actions Status](https://img.shields.io/github/actions/workflow/status/dima74/factorio-mods-localization/check.yml)](https://github.com/dima74/factorio-mods-localization/actions/workflows/check.yml)
[![GitHub license](https://img.shields.io/github/license/dima74/factorio-mods-localization.svg)](https://github.com/dima74/factorio-mods-localization/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/dima74/factorio-mods-localization.svg)](https://GitHub.com/dima74/factorio-mods-localization/issues/)

## Description
We provide service for simplifying [Factorio](https://www.factorio.com/) mods translation. You only need to install [our GitHub app][1]. After this, the following actions will be performed automatically:

* All existing English strings of your mod will be uploaded to [Crowdin](https://crowdin.com/)
* All existing translations will be uploaded too
* Every week our [FactorioBot](https://github.com/factorio-mods-helper) will fetch translation updates from Crowdin and commit them to your repository

## Motivation
There are a lot of Factorio mods hosted on GitHub. Most of them are translated using pull requests. I think it is not very convenient (because it is unclear which strings are untranslated yet and translators have to know how to use git). So, I created a helper tool for configuring the translation process on Crowdin, a powerful localization platform.

## Installation
1. Go to our [GitHub app page][1]
2. Click the install button
3. By default, the app will be installed for all repositories, and the app will automatically find repositories that have `locale` folder. Alternatively, you can manually select repositories for which app will be installed
4. Click the install button

You are done! Now share the link to [this Crowdin project][2] with translators.

Please note that **only Crowdin should be used for translation**.  GitHub pull requests must not be used for translation, otherwise translations will be lost after the next synchronization from Crowdin! Consider adding link to [Crowdin][2] to your repository Readme ([example](https://github.com/softmix/AutoDeconstruct/pull/6/files)).

## How to translate using Crowdin
We have a single Crowdin project. It consists of several folders, each folder corresponds to one mod. So, here are instructions on how to translate specific mod:

1. Go to [Crowdin project page][2]
2. Select language
3. Find the folder with your mod
4. Open the menu (click on three points) right of the folder name
5. Click "Open in Editor": ![menu](https://user-images.githubusercontent.com/6505554/85887708-bdfa5880-b801-11ea-99c1-766ad92ae4af.png)

Then Crowdin translation interface will be opened where you can translate strings.

## Notes

* To correctly upload your existing translations to Crowdin, files in any localization folder (such as `/locale/de`) **must have the same names as files in `/locale/en` folder**.
* If a repository has branch protection rules, then our helper will create a pull request (instead of pushing to the main branch directly).
* Please ask any questions or report bugs by creating a new [issue](https://github.com/dima74/factorio-mods-localization/issues).

## Multimods
It is possible to have multiple Factorio mods in a single GitHub repository. In this case please add file `factorio-mods-localization.json` with the list of mods to the root of the repository. See [`Omnimods/Omnimods` repository](https://github.com/Omnimods/Omnimods/commit/1e689afcf202776ffa0f675f73353f1fd67d2039) as an example.
```
├── factorio-mods-localization.json  // ["Mod1", "Mod2"]
├── Mod1
│   ├── locale/en
├── Mod2
│   ├── locale/en
```

## Detailed description of how it works
0. Mod author has a mod repository on GitHub
1. Mod author installs GitHub app (for mod repository)
2. Our service creates a subdirectory in our Crowdin project and uploads original English strings and existing translations into it
3. Every week our service takes translated strings from Crowdin and makes a commit to the GitHub repository (if there are any changes)
4. Every time original (locale/en) strings are changed, our service changes appropriate strings on Crowdin


  [1]: https://github.com/apps/factorio-mods-localization-helper
  [2]: https://crowdin.com/project/factorio-mods-localization
