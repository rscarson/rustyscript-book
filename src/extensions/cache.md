# IO Extensions
## Cache
> Crate features: [`cache`, `io_extensions`]  
> <https://crates.io/crates/deno_cache>  
> <https://w3c.github.io/ServiceWorker/#cache-interface>  

Populates the global `Cache`, `CacheStorage`, and `caches` objects.  
**Not sandbox safe. Off by default**

> [!NOTE]
> [query options](https://w3c.github.io/ServiceWorker/#dictdef-cachequeryoptions) are not yet supported for the `Cache.match` method.

### Options
**`RuntimeOptions::extension_options::cache`**
- The optional persistent caching backend used by the extension.
- Default: A non-persistent, in-memory cache

The cache option can also be set to `None`, which will effectively disable the cache functionality.

To configure the persistent cache, create an instance of the SQLite backend and pass it to the extension:
```rust
use rustyscript::{CacheBackend, ExtensionOptions, RuntimeOptions};

fn main() {
    // Will store the cache in a directory called "deno_cache"
    let cache = CacheBackend::new_sqlite("deno_cache").unwrap();
    let _options = RuntimeOptions {
        extension_options: ExtensionOptions {
            cache: Some(cache),
            ..Default::default()
        },
        ..Default::default()
    };
}
```

### Usage Example
```javascript
let cache = await caches.open('my_cache');

fetch('http://web.simmons.edu/').then((response) => {
    cache.put('http://web.simmons.edu/', response);
});

cache.match('http://web.simmons.edu/').then((response) => {
    console.log('Got response from cache!');
});
```