# Broadcast Channel
> Crate features: [`broadcast_channel`, `network_extensions`]
> <https://crates.io/crates/deno_broadcast_channel/>  
> <https://html.spec.whatwg.org/multipage/web-messaging.html>

Populates the global `BroadcastChannel` object.

Not sandbox safe. Off by default

## Options
**`RuntimeOptions::extension_options::broadcast_channel`**
- The channel object used by the extension.

## Usage Example
```js
const channel = new BroadcastChannel("my_channel");
channel.onmessage = (event) => {
  console.log(event.data);
};
channel.postMessage("Hello, world!");
channel.close();
```