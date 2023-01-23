import { CROWDIN_TO_GITHUB_UPDATE_PERIOD_MILLISECONDS } from './constants.js';
import moment from 'moment';

class Database {
    async getLastUpdateTime() {
        // TODO: Query last activity of https://github.com/factorio-mods-helper
        return null;
    }

    async isLastUpdateLongEnough() {
        const lastUpdateTime = await this.getLastUpdateTime();
        if (!lastUpdateTime) {
            return true;
        }
        const millisecondsPassed = Date.now() - lastUpdateTime.getTime();
        return millisecondsPassed > CROWDIN_TO_GITHUB_UPDATE_PERIOD_MILLISECONDS;
    }

    async getUpdatesInfo() {
        const lastUpdateTime = await this.getLastUpdateTime();
        const nextUpdateTime = lastUpdateTime ? moment(lastUpdateTime).add(7, 'days').fromNow() : 'should be just now';
        return 'Next update ' + nextUpdateTime + '\n\n';
    }
}

const database = new Database();
export default database;
