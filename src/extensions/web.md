# Web
> Crate features: [`web`, `network_extensions`, `io_extensions`]  
> Mutually exclusive with the `web_stub` extension.  
> <https://crates.io/crates/deno_web>  
> <https://crates.io/crates/deno_fetch>  
> <https://crates.io/crates/deno_tls>  
> <https://crates.io/crates/deno_net>  
> <https://w3c.github.io/FileAPI>  
> <https://fetch.spec.whatwg.org/>

Base Deno web API kit encompassing the `deno_web`, `deno_tls`, `deno_fetch`, and `deno_net` extensions.

This extension is required by all non-safe extensions.

> The `web_stub` extension is a minimal subset of this extension, used to instantiate the safe extensions.

Also populates the following:
- `Deno.HttpClient` and `Deno.createHttpClient`
- `Deno.connect`, `Deno.listen`, `Deno.resolveDns`, `Deno.listenDatagram`
- `Deno.connectTls`, `Deno.listenTls`, and `Deno.startTls`
- `Deno.refTimer`, and `Deno.unrefTimer`

### Options
**`RuntimeOptions::extension_options::web`**  
The `WebOptions` struct contains the following fields:

`base_url`  
The base URL to use for relative URL resolution  
- Default: None

`user_agent`  
The user agent string to use for network requests
- Default: Empty String

`root_cert_store_provider`  
Root certificate store for TLS connections for fetches and network OPs
- Default: None

`proxy`  
Proxy to provide to fetch operations
- Default: None

`request_builder_hook`  
Request builder hook for fetch
- Default: None

`unsafely_ignore_certificate_errors`  
List of domain names or IP addresses for which fetches and network OPs will ignore SSL errors
- Default: Empty Vec

`client_cert_chain_and_key`  
Client certificate and key for fetch  
- Default: `deno_tls::TlsKeys::Null`

`file_fetch_handler`  
File fetch handler for fetch
- Default: `deno_fetch::DefaultFileFetchHandler`

`permissions`  
The permissions manager to use for this extension, and several others.  
See [Permissions](../advanced/permissions.md) for more information.
- Default: `DefaultWebPermissions` (Allows all operations)

`blob_store`
The blob store to use for fetch
- Default: `deno_web::BlobStore`

### Permissions
Fetch is affected by the following methods in the permissions trait:
- `check_url` - Check if a given URL is allowed to be fetched
- `check_read` - Check if a given path is allowed to be read

Net is affected by the following methods in the permissions trait:
- `check_host` - Check if a given host is allowed to be connected to
- `check_read` - Check if a given path is allowed to be read
- `check_write` - Check if a given path is allowed to be written to

Web is affected by the following methods in the permissions trait:
- `allow_hrtime` - Allow high-resolution time measurements in timers

### Usage Example
```javascript
fetch("https://jsonplaceholder.typicode.com/todos/1")
  .then((response) => response.json())
  .then((data) => console.log(data));
```