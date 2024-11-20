# IO Extensions
## Cron
> Crate features: [`cron`, `io_extensions`]  
> <https://crates.io/crates/deno_cron>  

Populates the global `Deno.cron` object.  
**Not sandbox safe. Off by default**

### Usage Example
```javascript
let cronHandler = () => new Promise((resolve) => {
    console.log('Hello, world!');
    resolve();
});

Deno.cron('my-cron-job', '* * * * *', cronHandler)
```