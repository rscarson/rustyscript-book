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

