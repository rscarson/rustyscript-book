# IO Extensions
## Cron
> Crate features: [`cron`, `io_extensions`]  
> <https://crates.io/crates/deno_cron>  

Populates the global `Deno.cron` object.  
**Not sandbox safe. Off by default**

### Usage Example
```javascript
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
```