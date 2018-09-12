export const ROOT = process.env.NODE_ENV === 'development'
    ? '/home/dima/IdeaProjects/factorio/factorio-mods-localization/temp'
    : '/tmp';
// export const ROOT = '/tmp';

// week
export const CROWDIN_TO_GITHUB_UPDATE_PERIOD_MILLISECONDS = 7 * 24 * 60 * 60 * 1000;

export const GITHUB_COMMIT_MESSAGE = 'Update translations from Crowdin';
export const GITHUB_COMMIT_USER_NAME = 'Factorio Mods Helper';
export const GITHUB_COMMIT_USER_EMAIL = 'diraria+factorio-mods-localization@yandex.ru';