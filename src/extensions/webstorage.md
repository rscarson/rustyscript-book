# IO Extensions
## WebStorage
> Crate features: [`webstorage`, `io_extensions`]
> <https://crates.io/crates/deno_webstorage>
> <https://html.spec.whatwg.org/multipage/webstorage.html>  

Populates the global `Storage`, `localStorage`, and `sessionStorage` objects

### Options
**`RuntimeOptions::extension_options::webstorage_origin_storage_dir`**
- Optional directory for storage
- Default: `None`

If a directory is provided, then the storage will be persisted to that directory.  
Otherwise, most webstorage operations will be unavailable.

### Usage Example
```js,norun
localStorage.setItem("key", "value");
const value = localStorage.getItem("key");
console.log(value);
```