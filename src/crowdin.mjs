import axios from 'axios';

class Crowdin {
    constructor() {
        const projectId = process.env.CROWDIN_PROJECT_ID;
        const apiKey = process.env.CROWDIN_API_KEY;
        if (!projectId || !apiKey) {
            console.error('Environment variables CROWDIN_PROJECT_ID and CROWDIN_API_KEY must be set');
            process.exit(1);
        }
        axios.defaults.baseURL = `https://api.crowdin.com/api/project/${projectId}`;
        // axios.defaults.baseURL = `https://httpbin.org/post`;
        axios.defaults.params = { key: apiKey, json: true };
    }

    async createDirectory(name) {
        await axios.post('/add-directory', null, { params: { name, recursive: 1 } });
    }
}

const crowdin = new Crowdin();
export default crowdin;