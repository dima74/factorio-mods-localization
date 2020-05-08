import fs from 'fs-extra';
import path from 'path';
import assert from 'assert';
import recursiveReaddir from 'recursive-readdir';
import { getCrowdinDirectoryName, replaceCfgToIni, replaceIniToCfg } from './crowdin.js';

export async function deleteEmptyIniFiles(directory) {
    const files = await recursiveReaddir(directory);
    await Promise.all(files.map(file => deleteIniFileIfEmpty(file)));
}

async function deleteIniFileIfEmpty(file) {
    const content = await fs.readFile(file, 'utf8');
    const lines = content
        .split('\n')
        .map(line => line.trim())
        .filter(line => line && !line.startsWith('['));
    if (lines.length === 0) {
        await fs.unlink(file);
    }
}

export async function moveTranslatedFilesToRepository(translationsDirectory, repository) {
    const languages = await fs.readdir(translationsDirectory);
    for (const language of languages) {
        const languagePathCrowdin = path.join(translationsDirectory, language, getCrowdinDirectoryName(repository.fullName));
        assert(await fs.exists(languagePathCrowdin));
        const files = await fs.readdir(languagePathCrowdin);
        if (files.length === 0) {
            continue;
        }

        const languagePathRepository = path.join(repository.localesPath, language);
        if (!(await fs.exists(languagePathRepository))) {
            await fs.mkdir(languagePathRepository);
        }
        await Promise.all(files.map(file => {
            assert(file.endsWith('.ini'));
            const fileRenamed = replaceIniToCfg(file);
            const oldPath = path.join(languagePathCrowdin, file);
            const newPath = path.join(languagePathRepository, fileRenamed);
            return fs.rename(oldPath, newPath);
        }));
    }
}

export function getAllModifiedAndAddedFiles(commits) {
    const commitsFiles = commits.map(commit => [...commit.added, ...commit.modified]);
    return [].concat(...commitsFiles);
}

export async function getSubdirectories(directory) {
    const dirents = await fs.readdir(directory, { withFileTypes: true });
    return dirents
        .filter(dirent => dirent.isDirectory())
        .map(dirent => dirent.name);
}
