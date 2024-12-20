# Deno Extensions
## Safe Extensions

All the extensions mentioned below can be activated using the `safe_extensions` crate feature.

> [!TIP]
> **By default, `Rustyscript` includes only those extensions that maintain a secure, sandboxed runtime environment.**
>
> This means that the Javascript code you run has no access to system resources such as the file system, network, or environment variables.

The safe extensions included by default are:
- [`console`](console.md) - For logging
- [`crypto`](crypto.md) - For cryptographic functions
- [`url`](url.md) - For URL parsing
- [`web_stub`](web_stub.md) - A stub for the `web` extension, providing timers, and base64 encoding/decoding

The remaining extensions can be broadly categorized as either `io` or `network`.

> [!WARNING]
> With the exception of [`cron`](cron.md), [`webstorage`](webstorage.md), and [`ffi`](ffi.md), all remaining extensions depend on the [`web`](web.md) extension.