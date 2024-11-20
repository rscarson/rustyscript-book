# Getting Started
## The Sandbox

One of the guiding principles of `rustyscript` is to provide a safe sandboxed environment for running JavaScript code by default.
> It should not be possible to access the host system's resources such as the file system, network, or environment variables through JS in the default runtime configuration

Only the [`safe_extensions`](../extensions/safe_extensions.md), [`worker`](../advanced/multithreading.md), and [`snapshot_builder`](../advanced/snapshots.md) features can be enabled without breaking the sandbox.

With the default configuration and crate features, sandboxing is enforced by the following mechanisms:
- **Op safety** - All the ops provided by default have been vetted and whitelisted to ensure they are safe
- **Import isolation** - The module loader will by default only allow modules that have been loaded with `Runtime::load_module`
    - A couple of crate features can change this:
        - `fs_import` will allow loading modules from the filesystem
        - `url_import` will allow loading modules from network location
- **Extension limiting** - Only a subset of extensions are enabled by default, using a safe stub of the deno_web API
    - See the [extensions](../extensions/README.md) section for more information on the available optional extensions

> **Note:** Extension is a Deno term referring to a subset of the JS standard API.
> rustyscript provides these as crate features that can be enabled or disabled at compile time.