# Deno Extensions
## Network Extensions

All the extensions mentioned below can be activated using the `network_extensions` crate feature.

These extensions grant runtimes access to system network resources

- [`web`](web.md) - Timers, events, text encoder/decoder, See [here](https://w3c.github.io/FileAPI) and [here](https://fetch.spec.whatwg.org/)
- [`webstorage`](webstorage.md) - Web storage API, API reference [here](https://html.spec.whatwg.org/multipage/webstorage.html)
- [`websocket`](websocket.md) - Websocket API, API reference [here](https://websockets.spec.whatwg.org)
- [`http`](http.md) - Fetch primitives, API reference [here](https://fetch.spec.whatwg.org/)
- [`broadcast_channel`](broadcast_channel.md) - Web messaging, API reference [here](https://html.spec.whatwg.org/multipage/web-messaging.html)