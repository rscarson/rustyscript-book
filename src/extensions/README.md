# Deno Extensions

The Deno JS runtime on which `Rustyscript` is based boasts very good compatibility with the complete JS standards.

This is done through a series of extensions, listed below, which each provide a part of the full API.  
This gives us the ability to use the full JS standard library, or just the parts we need.

By default, `Rustyscript` includes only those extensions that maintain a secure, sandboxed runtime environment.
> See the [Safe Extensions](safe_extensions.md) section for more information.

-----


Extensions can be activated using crate features, either individually or in groups:
- [`safe_extensions`](safe_extensions.md) - On by default
- [`io_extensions`](io_extensions.md) - For file system access
- [`network_extensions`](network_extensions.md) - For network access
- [`node_experimental`](nodejs_extensions.md) - For compatibility with Deno's NodeJS standard library polyfills