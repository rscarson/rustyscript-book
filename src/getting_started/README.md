# Getting Started

[![Crates.io](https://img.shields.io/crates/v/rustyscript.svg)](https://crates.io/crates/rustyscript/)
[![Build Status](https://github.com/rscarson/rustyscript/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/rustyscript/actions?query=branch%3Amaster)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/rustyscript/master/LICENSE)

## Rustyscript - Effortless JS Integration for Rust
This crate is meant to provide a quick and simple way to integrate a runtime javascript or typescript component from within rust.

It uses the v8 engine through the `deno_core` crate, and is meant to be as simple as possible to use without sacrificing flexibility or performance.

I also have attempted to abstract away the v8 engine details so you can for the most part operate directly on rust types.

- By default, the code being run is entirely sandboxed from the host, having no filesystem or network access.
    - It can be extended to include those capabilities and more if desired - See the [extensions](../extensions) section for more information
- Asynchronous JS code is supported (I suggest using the timeout option when creating your runtime)
- Loaded JS modules can import other modules
- Typescript is supported by default, and will be transpiled into JS for execution
- Node JS is supported experimentally, but is not yet fully compatible [See the NodeJS Compatibility section](../advanced/nodejs_compatibility.md)


The crate includes the following features that can be turned on or off as needed:

#### Extension Features
For a full list of available extensions, see the [extensions](../extensions) section.

> **`safe_extensions`** `ON BY DEFAULT`  
> Deno extensions that maintain a secure, sandboxed runtime environment

> **`io_extensions`**  
> Deno extensions that grant runtimes access to the file system (but may also grant some level of network access - use caution)

> **`network_extensions`**  
> Deno extensions that grant runtimes access to system network resources

#### Javascript Isolation Features

> **`url_import`**  
> Enables executed Javascript to include modules from arbitrary URLs

> **`fs_import`**  
> Enables executed Javascript to include modules from the file system, without needing to load them from rust first

#### Additional Features

> **`worker`** `ON BY DEFAULT`  
> Enables the multithreaded [`worker`](https://docs.rs/rustyscript/latest/rustyscript/worker/index.html) API

> **`snapshot_builder`**  
> Enables the [`snapshot_builder`](../advanced/snapshots.md) API