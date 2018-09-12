import fs from 'fs';
import path from 'path';
import isEqual from 'lodash.isequal';

const MODS_DIRECTORY = '../../../factorio/factorio-mods';

function printLocalizationFoldersSubdirectories() {
    const subdirectories = {};
    const mods = fs.readdirSync(MODS_DIRECTORY);
    for (const mod of mods) {
        const languagesPath = path.join(MODS_DIRECTORY, mod, 'locale');
        if (!fs.existsSync(languagesPath)) {
            continue;
        }

        const languages = fs.readdirSync(languagesPath, { withFileTypes: true });
        for (const language of languages) {
            if (!language.isDirectory()) {
                continue;
            }
            const languagePath = path.join(languagesPath, language.name);
            const languageDirents = fs.readdirSync(languagePath, { withFileTypes: true });
            languageDirents
                .filter(dirent => dirent.isDirectory())
                .forEach(dirent => subdirectories[dirent.name] = true);
        }
    }
    console.log('Subdirectories:');
    console.log('\t' + Object.keys(subdirectories).join('\n\t'));
}

function printPercentageModsWithUnmatchedLocalizationFilesNames() {
    function getSortedLanguageFilesNames(languagePath) {
        return fs.readdirSync(languagePath).sort();
    }

    let numberNotMatchedMods = 0;
    let numberMods = 0;
    const mods = fs.readdirSync(MODS_DIRECTORY);
    for (const mod of mods) {
        const languagesPath = path.join(MODS_DIRECTORY, mod, 'locale');
        const languageEnPath = path.join(languagesPath, 'en');
        if (!fs.existsSync(languagesPath) || !fs.existsSync(languageEnPath)) {
            continue;
        }

        const languages = fs.readdirSync(languagesPath, { withFileTypes: true });
        const namesEn = getSortedLanguageFilesNames(languageEnPath);
        for (const language of languages) {
            if (!language.isDirectory()) {
                continue;
            }
            const languagePath = path.join(languagesPath, language.name);
            const namesLanguage = getSortedLanguageFilesNames(languagePath);
            if (!isEqual(namesEn, namesLanguage)) {
                ++numberNotMatchedMods;
                break;
            }
        }
        ++numberMods;
    }
    console.log('Not matched');
    console.log(`\t${Math.round(100 * numberNotMatchedMods / numberMods)}%`);
    console.log(`\t${numberNotMatchedMods} / ${numberMods}`);
}

printLocalizationFoldersSubdirectories();
printPercentageModsWithUnmatchedLocalizationFilesNames();

