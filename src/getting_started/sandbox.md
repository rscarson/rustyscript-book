# Getting Started
## The Sandbox

One of the guiding principles of `rustyscript` is to provide a safe sandboxed environment for running JavaScript code by default.
> It should not be possible to access the host system's resources such as the file system, network, or environment variables through JS in the default runtime configuration

Only the [`safe_extensions`](../extensions/safe_extensions.md), [`worker`](../advanced/multithreading.md), and [`snapshot_builder`](../advanced/snapshots.md) features can be enabled without breaking the sandbox.