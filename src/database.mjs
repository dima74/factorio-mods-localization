import pg from 'pg';
import { CROWDIN_TO_GITHUB_UPDATE_PERIOD_MILLISECONDS } from './constants';
import moment from 'moment';

const QUERY_CREATE_TABLE_IF_NOT_EXISTS = `
CREATE TABLE IF NOT EXISTS factorio 
(last_github_from_crowdin_update timestamptz)
;`;

const QUERY_GET_LAST_UPDATE_TIME = `
SELECT last_github_from_crowdin_update
FROM factorio
ORDER BY last_github_from_crowdin_update
DESC LIMIT 1
;`;

const QUERY_ADD_UPDATE_DATE = `
INSERT into factorio (last_github_from_crowdin_update) 
VALUES (current_timestamp)
;`;

class Database {
    async init() {
        const config = process.env.NODE_ENV === 'development'
            ? { user: 'postgres', database: 'postgres' }
            : {
                connectionString: process.env.DATABASE_URL,
                ssl: true,
            };
        this.client = new pg.Client(config);

        await this.client.connect();
        // await this.client.query(`DROP TABLE IF EXISTS factorio;`);
        await this.client.query(QUERY_CREATE_TABLE_IF_NOT_EXISTS);
    }

    async getLastUpdateTime() {
        const result = await this.client.query(QUERY_GET_LAST_UPDATE_TIME);
        return result.rows.length === 0 ? undefined : result.rows[0].last_github_from_crowdin_update;
    }

    async isLastUpdateLongEnough() {
        const lastUpdateTime = await this.getLastUpdateTime();
        if (!lastUpdateTime) {
            return true;
        }
        const millisecondsPassed = Date.now() - lastUpdateTime.getTime();
        return millisecondsPassed > CROWDIN_TO_GITHUB_UPDATE_PERIOD_MILLISECONDS;
    }

    async commitUpdate() {
        await this.client.query(QUERY_ADD_UPDATE_DATE);
    }

    async closeConnection() {
        await this.client.end();
    }

    // for debug
    async getAllTimestamps() {
        const result = await this.client.query(`SELECT last_github_from_crowdin_update FROM factorio;`);
        return result.rows.map(row => row.last_github_from_crowdin_update);
    }

    async getUpdatesInfo() {
        let info = '';

        const lastUpdateTime = await this.getLastUpdateTime();
        const nextUpdateTime = lastUpdateTime ? moment(lastUpdateTime).add(7, 'days').fromNow() : 'should be just now';
        info += 'Next update ' + nextUpdateTime + '\n\n';

        const timestamps = await this.getAllTimestamps();
        info += timestamps.length > 0 ? ['Passed updates:', ...timestamps].join('\n\t') : 'No passed updates yet';
        return info;
    }
}

const database = new Database();
export default database;