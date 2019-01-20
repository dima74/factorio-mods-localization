const logLines = [];

function getDate() {
    const date = new Date().toUTCString();
    return date.substr(0, date.length - ' GMT'.length);
}

for (const stream of [process.stdout, process.stderr]) {
    const write = stream.write;
    stream.write = (...args) => {
        write.call(stream, ...args);
        const consoleLine = args[0]
            .replace('{ Error:', 'Error:')
            .replace(/    at .*\n/g, '');  // hack to remove stack traces
        const logLine = `[${getDate()}] ${consoleLine}`;
        logLines.push(logLine);
    };
}

export function getRepositoryLogs(fullName) {
    const [owner, repo] = fullName.split('/');
    return logLines.filter(line => line.includes(`[${fullName}]`) || line.includes('[*]') || line.includes(`[owner:${owner}]`));
}