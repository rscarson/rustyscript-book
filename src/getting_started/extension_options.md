# Getting Started
## Extension Options

[Back to Runtime Options](runtime_options.md)

The built-in deno extensions have additional options that can be configured when creating a runtime. These options are provided in the[`ExtensionOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.ExtensionOptions.html) struct, which is a field of the [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html) struct.

The fields it contains depend on the features enabled in the `rustyscript` crate. Here is a list of the fields and the features they depend on:

- kv_store
    - Depends on the [`kv`](../extensions/kv.md) feature
    - Defines the key-value store to use for the `deno_kv` extension
- broadcast_channel
    - Depends on the [`broadcast_channel`](../extensions/broadcast_channel.md) feature
    - Defines the in-memory broadcast channel to use for the `deno_broadcast_channel` extension
- filesystem
    - Depends on the [`fs`](../extensions/fs.md) feature
    - Defines the filesystem implementation to use for the `deno_fs` extension
    - Default is `deno_fs::RealFs`
- cache
    - Depends on the [`cache`](../extensions/cache.md) feature
    - Defines the cache configuration to use for the `deno_cache` extension
- webstorage_origin_storage_dir
    - Depends on the [`webstorage`](../extensions/webstorage.md) feature
    - Defines the directory where the webstorage extension will store its data
- io_pipes
    - Depends on the [`io`](../extensions/io.md) feature
    - Defines the stdin/out/err pipes for the `deno_io` extension
- crypto_seed
    - Depends on the [`crypto`](../extensions/crypto.md) feature
    - Defines the seed for the `deno_crypto` extension

- node_resolver
    - Depends on the [`node_experimental`](../advanced/nodejs_compatibility.md) feature
    - Defines the package resolver to use for the `deno_node` extension
    - The `RustyResolver` type allows you to select the base dir for modules and the filesystem implementation to use

- web
    - Depends on the [`web`](../extensions/web.md) feature
    - Defines the options for the `deno_web`, `deno_fetch`, and `deno_net` extensions
    - Also defines permissions for related APIs
    - fields:
        - base_url: Base URL for some `deno_web` OPs
        - user_agent: User agent to use for fetch
        - root_cert_store_provider: Root certificate store for TLS connections for fetches and network OPs
        - proxy: Proxy for fetch
        - request_builder_hook: Request builder hook for fetch
        - unsafely_ignore_certificate_errors: List of domain names or IP addresses to ignore SSL errors for
        - client_cert_chain_and_key: Client certificate and key for fetch
        - file_fetch_handler: File fetch handler for fetch
        - permissions: Permissions manager for sandbox-breaking extensions
        - blob_store: Blob store for the web related extensions
