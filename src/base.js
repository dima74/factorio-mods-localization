// proper logging on error (why node doesn't have it by default???)
process
    .on('unhandledRejection', (reason, p) => {
        console.error('Unhandled Rejection at Promise');
        if (reason.host) {
            console.error(reason.host + reason.path);
        }
        console.error(reason);
        process.exit(1);
    })
    .on('uncaughtException', err => {
        console.error(err, 'Uncaught Exception thrown');
        process.exit(1);
    });