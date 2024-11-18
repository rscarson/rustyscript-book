# Getting Started
## Extension Options

[Back to Runtime Options](runtime_options.md)

The built-in deno extensions have additional options that can be configured when creating a runtime. These options are provided in the[`ExtensionOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.ExtensionOptions.html) struct, which is a field of the [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html) struct.

The fields it contains depend on the features enabled in the `rustyscript` crate. Here is a list of the fields and the features they depend on:

### kv_store [`kv`](../extensions/kv.md)
Defines the key-value store to use for the `deno_kv` extension

### broadcast_channel [`broadcast_channel`](../extensions/broadcast_channel.md)
Defines the in-memory broadcast channel to use for the `deno_broadcast_channel` extension

### filesystem [`fs`](../extensions/fs.md)
Defines the filesystem implementation to use for the `deno_fs` extension
- Default is `deno_fs::RealFs`

### cache [`cache`](../extensions/cache.md)
Defines the cache configuration to use for the `deno_cache` extension
- Disabled by default

### webstorage_origin_storage_dir [`webstorage`](../extensions/webstorage.md)
Defines the directory where the webstorage extension will store its data

### io_pipes [`io`](../extensions/io.md)
Defines the stdin/out/err pipes for the `deno_io` extension

### crypto_seed [`crypto`](../extensions/crypto.md)
Defines the seed for the `deno_crypto` extension
- Default is `None`

### node_resolver [`node_experimental`](../advanced/nodejs_compatibility.md)
Defines the package resolver to use for the `deno_node` extension
- Default is `RustyResolver::new()`
- The `RustyResolver` type allows you to select the base dir for modules and the filesystem implementation to use

### web [`web`](../extensions/web.md)
Defines the options for the `deno_web`, `deno_fetch`, and `deno_net` extensions
- Also defines permissions for related APIs
- fields:
    - **base_url**: Base URL for some `deno_web` OPs
    - **user_agent**: User agent to use for fetch
    - **root_cert_store_provider**: Root certificate store for TLS connections for fetches and network OPs
    - **proxy**: Proxy for fetch
    - **request_builder_hook**: Request builder hook for fetch
    - **unsafely_ignore_certificate_errors**: List of domain names or IP addresses to ignore SSL errors for
    - **client_cert_chain_and_key**: Client certificate and key for fetch
    - **file_fetch_handler**: File fetch handler for fetch
    - **permissions**: Permissions manager for sandbox-breaking extensions
    - **blob_store**: Blob store for the web related extensions
