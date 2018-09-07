import github from './src/github';
import webhooks from './src/webhooks';
import './src/base';

async function init() {
    await github.init();
    await webhooks.init();
}

init();

// function temp() {
//     import {spawn} from 'child_process';
//     const process = spawn('git', ['status']);
//
//     process.stdout.on('data', (data) => {
//         console.log(`stdout: ${data}`);
//     });
//
//     process.stderr.on('data', (data) => {
//         console.log(`stderr: ${data}`);
//     });
// }
