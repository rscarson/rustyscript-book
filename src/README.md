![Rustyscript - Effortless JS Integration for Rust](assets/rustyscript-logo-wide.png =500x)

[![Crates.io](https://img.shields.io/crates/v/rustyscript.svg)](https://crates.io/crates/rustyscript/)
[![Build Status](https://github.com/rscarson/rustyscript/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/rustyscript/actions?query=branch%3Amaster)
[![docs.rs](https://img.shields.io/docsrs/rustyscript)](https://docs.rs/rustyscript/latest/rustyscript/)
[![Static Badge](https://img.shields.io/badge/mdbook-user%20guide-blue)](https://rscarson.github.io/rustyscript-book/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/rustyscript/master/LICENSE)

rustyscript provides a quick and simple way to integrate a runtime javascript or typescript component from within Rust.

It uses the v8 engine through the `deno_core` crate, and aims to be as simple as possible to use without sacrificing flexibility or performance.  
I also have attempted to abstract away the v8 engine details so you can for the most part operate directly on rust types.


**Sandboxed**  
By default, the code being run is entirely sandboxed from the host, having no filesystem or network access.  
[extensions](extensions) can be added to grant additional capabilities that may violate sandboxing

**Flexible**  
The runtime is designed to be as flexible as possible, allowing you to modify capabilities, the module loader, and more.  
- Asynchronous JS is fully supported, and the runtime can be configured to run in a multithreaded environment.  
- Typescript is supported, and will be transpired into JS for execution.
- Node JS is supported experimentally, but is not yet fully compatible ([See the NodeJS Compatibility section](advanced/nodejs_compatibility.md))

**Unopinionated**  
Rustyscript is designed to be a thin wrapper over the Deno runtime, to remove potential pitfalls and simplify the API without sacrificing flexibility or performance.

### Extension Features

The crate includes the following features that can be turned on or off as needed:  
For a full list of available extensions, see the [extensions](extensions) section.

> **`safe_extensions`** `ON BY DEFAULT`  
> `console` `crypto` `url` `web_stub`  
> Deno extensions that maintain a secure, sandboxed runtime environment  

> **`io_extensions`**  
> `web` `cache` `cron` `ffi` `fs` `io` `kv` `webgpu`  
> Deno extensions that grant runtimes access to the file system (but may also grant some level of network access - use caution)

> **`network_extensions`**  
> `broadcast_channel` `http` `web` `websocket` `webstorage`  
> Deno extensions that grant runtimes access to system network resources

> **`node_experimental`**  
> `safe_extensions` `io_extensions` `network_extensions`  
> Experimental NodeJS compatibility

#### Javascript Isolation Features

> **`url_import`**  
> Enables executed Javascript to include modules from arbitrary URLs

> **`fs_import`**  
> Enables executed Javascript to include modules from the file system, without needing to load them from rust first

#### Additional Features

> **`worker`** `ON BY DEFAULT`  
> Enables the multithreaded [`worker`](https://docs.rs/rustyscript/latest/rustyscript/worker/index.html) API

> **`snapshot_builder`**  
> Enables the [`snapshot_builder`](advanced/snapshots.md) API