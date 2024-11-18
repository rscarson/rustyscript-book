# Getting Started
## Runtime Options

To create a runtime, you will need to provide a [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html) struct. This struct contains all the configuration options for the runtime, such as the extensions to load, the entrypoint to use, and the maximum heap size.

It implements the `Default` trait, so you can create a default configuration by calling `RuntimeOptions::default()`.

`RuntimeOptions` has the following fields:

- `extensions`
    - A set of `deno_core` extensions to add to the runtime
    - See [Custom Extensions](../advanced/custom_extensions.md)
- `extension_options`
    - Additional options for the built-in extensions
    - See [Extension Options](extension_options.md)
- `default_entrypoint`
    - Function name to use as entrypoint if the module does not explicitely provide one
    - See [On Modules and import](modules.md)
- `timeout`
    - Amount of time to run async tasks before failing. Only affects blocking functions
- `max_heap_size`
    - Maximum heap size for the runtime. The runtime will fail gracefully if this limit is reached
- `import_provider`
    - Optional import provider for the module loader
    - Acts as cache, and handler for custom schema prefixes and data sources
- `startup_snapshot`
    - Optional snapshot to load into the runtime
    - This will reduce load times, but requires the same extensions to be loaded, in the same order
    - If provided, user-supplied extensions must be instantiated with `init_ops` instead of `init_ops_and_esm`
    - WARNING: Snapshots MUST be used on the same system they were created on
    - See [Snapshots](../advanced/snapshots.md)
- `isolate_params`
    - Optional configuration parameters for building the underlying v8 isolate
    - This can be used to alter the behavior of the runtime
    - See [v8::CreateParams](https://docs.rs/v8/130.0.1/v8/struct.CreateParams.html) for more information
- `shared_array_buffer_store`
    - Optional shared array buffer store to use for the runtime
    - Allows data-sharing between runtimes across threads
- `schema_whlist`
    - A whitelist of custom schema prefixes that are allowed to be loaded from javascript
    - By default only `http`/`https` (`url_import` crate feature), and `file` (`fs_import` crate feature) are allowed
