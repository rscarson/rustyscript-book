# Getting Started
## On Modules and Including them

By default, any javascript code can only import modules that have already been loaded with [`Runtime::load_module`](https://docs.rs/rustyscript/latest/rustyscript/runtime/struct.Runtime.html#method.load_module). This is a security feature to prevent arbitrary code from being loaded from the filesystem or network.

However, this can be changed with the `fs_import` and `url_import` features. These features allow the runtime to load modules from the filesystem or network respectively.

### URL Schemes

Custom URL Schemes can be added to a runtime with the `schema_whlist` field in the [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html) struct.
- [Usage Example](https://github.com/rscarson/rustyscript/blob/master/examples/runtime_extensions.rs)

### Entrypoints

When a module is loaded, the runtime will look for a function with the name provided in the `default_entrypoint` field of the [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html) struct, or which is exported as `default` in the module.

Additionally, you can call `rustyscript.register_entrypoint` from JS to register a function as an entrypoint at runtime

[`Runtime::call_entrypoint`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html#method.call_entrypoint) will call the entrypoint function with the provided arguments.

[Example 1](https://github.com/rscarson/rustyscript/blob/master/examples/entrypoint_functions.rs)  
[Example 2](https://github.com/rscarson/rustyscript/blob/master/examples/hello_world.rs)

### ImportProvider Trait

The `ImportProvider` trait can be implemented to provide custom module loading behavior

- It can be used to implement a cache: [Example](https://github.com/rscarson/rustyscript/blob/master/examples/module_loader_cache.rs)
- Or to provide custom import logic: [Example](https://github.com/rscarson/rustyscript/blob/master/examples/custom_import_logic.rs)