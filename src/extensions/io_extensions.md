# Deno Extensions
## IO Extensions

All the extensions mentioned below can be activated using the `io_extensions` crate feature.

These extensions grant runtimes access to the file system - but may also grant some level of network access - use caution.

- [`fs`](fs.md) - For file system access
- [`io`](io.md) - input/output primitives (stdio streams, etc)
- [`cache`](cache.md) - Cache support, API reference [here](https://w3c.github.io/ServiceWorker/#cache-interface)
- [`ffi`](ffi.md) - Deno FFI support
- [`webgpu`](webgpu.md) - WebGPU support, API reference [here](https://gpuweb.github.io/gpuweb/)
- [`kv`](kv.md) - Key-value store, API reference [here](https://github.com/denoland/denokv/blob/main/proto/kv-connect.md)
- [`cron`](cron.md) - Implements scheduled tasks (crons) API