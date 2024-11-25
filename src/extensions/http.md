# Network Extensions
## HTTP
> Crate features: [`http`, `network_extensions`]  
> <https://crates.io/crates/deno_http>  
> <https://fetch.spec.whatwg.org/>  

Implements server-side HTTP.  
Populates the `Deno.serve`, `Deno.serveHttp`, and `Deno.upgradeWebSocket` functions.  
**Not sandbox safe. Off by default**

### Usage Example
```ts
// The abort signal is used to close the server
const ac = new AbortController();

try {
  const server = Deno.serve(
    { signal: ac.signal },
    (_req) => new Response("Hello, world")
  );
} catch (err) {
  console.error("Operation aborted");
}

// Close the server after 1 second
setTimeout(() => ac.abort(), 1000);
```