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
