let cronHandler = () => new Promise((resolve) => {
    console.log('Hello, world!');
    resolve();
});

Deno.cron('my-cron-job', '* * * * *', cronHandler)
