// The abort signal can be used to cancel the job
const ac = new AbortController();

let cronHandler = () => new Promise((resolve) => {
    console.log("This will print once per minute.");
    resolve();
});

Deno.cron(
    'my-cron-job', '* * * * *',
    { signal: ac.signal },
    cronHandler
);

// Abort the cron job after 1 seconds
setTimeout(() => {
    ac.abort();
}, 1000);