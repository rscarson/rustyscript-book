# IO Extensions
## KV
> Crate features: [`kv`, `io_extensions`]  
> <https://crates.io/crates/deno_kv>  
> <https://docs.deno.com/deploy/kv/manual>  
> <https://crates.io/crates/deno_kv#kv-connect>  
> <https://deno.com/deploy>  
> <https://github.com/denoland/denokv/blob/main/proto/kv-connect.md>

Provides a key/value store
Populates the global `Deno.openKv`, `Deno.AtomicOperation`, `Deno.KvU64`, and `Deno.KvListIterator` objects

### Options
**`RuntimeOptions::extension_options::kv_store`**  
A [`KvStore`](https://docs.rs/rustyscript/latest/rustyscript/struct.KvStore.html) defining the store to use for KV operations
- Default: `KvStore::new_local(None, None, KvConfig::default)` (An in-memory local store)

### Usage Example
```ts
const db = await Deno.openKv();
await db.set(["foo"], "bar");

const result = await db.get(["foo"]);
result.key; // ["foo"]
result.value; // "bar"
```