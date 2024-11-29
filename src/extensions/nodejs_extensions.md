# Deno Extensions
## NodeJS Extensions
> Crate features: [`node_experimental`]  
> <https://crates.io/crates/deno_node>  
> <https://crates.io/crates/deno_resolver>  
> <https://crates.io/crates/node_resolver>  
> <https://crates.io/crates/deno_npm>  
> <https://crates.io/crates/deno_semver>  
> <https://crates.io/crates/deno_napi>  
> <https://crates.io/crates/deno_runtime>  

Provides BYONM (bring-your-own-npm-module) support for Deno.  
See [NodeJS Compatibility](../advanced/nodejs_compatibility.md) for more information.


Includes a very large set of Deno APIs, most of which are needed to run Deno's NodeJS standard library polyfills.

> [!NOTE]
> The list of APIs below is not exhaustive and does not include the NodeJS standard library polyfills themselves.

### `fs_events`
Provides `Deno.watchFs`

### `os`
Provides:  
`Deno.env`, `Deno.exit`, `Deno.execPath`, `Deno.loadavg`, `Deno.osRelease`, `Deno.osUptime`, `Deno.hostname`, `Deno.systemMemoryInfo`, `Deno.networkInterfaces`, `Deno.gid`, `Deno.uid`


### `permissions`
Provides:  
`Deno.permissions`, `Deno.Permissions`, `Deno.PermissionStatus`

### `process`
Provides:  
`Deno.Process`, `Deno.run`, `Deno.kill`, `Deno.Command`, `Deno.Process`

### `signal`
Provides:  
`Deno.addSignalListener`, `Deno.removeSignalListener`

### `web_worker` / `worker_host`
Provides worker support for the NodeJS API