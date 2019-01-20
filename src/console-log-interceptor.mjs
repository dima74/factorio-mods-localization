const logLines = [];

for (const stream of [process.stdout, process.stderr]) {
    const write = stream.write;
    stream.write = (...args) => {
        write.call(stream, ...args);
        const consoleLine = args[0];
        const logLine = `[${new Date()}] ${consoleLine}`;
        logLines.push(logLine);
    };
}

export function getRepositoryLogs(fullName) {
    const [owner, repo] = fullName.split('/');
    return logLines.filter(line => line.includes(`[${fullName}]`) || line.includes('[*]') || line.includes(`[owner:${owner}]`));
}