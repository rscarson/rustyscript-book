# Cache
> Crate features: [`broadcast_channel`, `network_extensions`]
> <https://crates.io/crates/deno_broadcast_channel/>  
> <https://html.spec.whatwg.org/multipage/web-messaging.html>

Populates the global `BroadcastChannel` object.

Not sandbox safe. Off by default

## Options
**`RuntimeOptions::extension_options::cache`**
- The optional caching backend used by the extension.
- Default: `None`

To configure the cache, create an instance of the SQLite backend and pass it to the extension:
```rust
use rustyscript::{
    extensions::deno_cache::{CreateCache, SqliteBackedCache},
    ExtensionOptions, RuntimeOptions,
};
use std::sync::Arc;

fn main() {
    let storage_dir = std::path::Path::new("deno_cache").to_path_buf();
    let create_cache_fn = move || SqliteBackedCache::new(storage_dir.clone());
    let cache = CreateCache(Arc::new(create_cache_fn));

    let _options = RuntimeOptions {
        extension_options: ExtensionOptions {
            cache: Some(cache),
            ..Default::default()
        },
        ..Default::default()
    };
}

```

## Usage Example
```js
TODO
```