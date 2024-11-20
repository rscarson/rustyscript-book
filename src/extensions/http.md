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
Deno.serve((_req: Request) => {
  return new Response("Hello, world!");
});
```