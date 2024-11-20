# Safe Extensions
## Url
> Crate features: [`url`, `safe_extensions`]  
> <https://crates.io/crates/deno_url/>  
> <https://url.spec.whatwg.org/>
> <https://wicg.github.io/urlpattern/>

Populates the global `URL`, `URLPattern`, `URLSearchParams` objects  
**This extensions is sandbox safe. It is enabled by default.**

### Usage Example
```js
const url = new URL("https://example.com");
const pattern = new URLPattern("https://example.com/*");
const params = new URLSearchParams("a=1&b=2");
```