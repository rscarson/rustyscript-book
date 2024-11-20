# Network Extensions
## WebSocket
> Crate features: [`websocket`, `network_extensions`]  
> <https://crates.io/crates/deno_websocket>  
> <https://html.spec.whatwg.org/multipage/web-sockets.html>  

Populates the global `WebSocket` and `WebSocketStream` objects

### Options
Uses the `user_agent`, `root_cert_store_provider`, and `unsafely_ignore_certificate_errors` fields of `RuntimeOptions::extension_options::web`

### Permissions
This extension is affected by the `check_url` function in the permissions trait, which checks if a given URL is allowed to be accessed

### Usage Example
```ts
const ws = new WebSocket("ws://localhost:8080");

ws.onopen = () => {
  console.log("Connected");
  ws.send("Hello, world!");
  ws.close();
};
```