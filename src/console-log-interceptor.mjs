const logs = [];

for (const stream of [process.stdout, process.stderr]) {
    const write = stream.write;
    stream.write = (...args) => {
        write.call(stream, ...args);
        const s = args[0];
        logs.push(s.substr(0, s.length - 1));  // \n
    };
}

export function getRepositoryLogs(fullName) {
    return logs.filter(line => line.includes(`[${fullName}]`) || line.includes('[*]'));
}