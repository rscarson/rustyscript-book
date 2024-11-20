# IO Extensions
## Io
> Crate features: [`io`, `io_extensions`]
> <https://crates.io/crates/deno_io>  

Provides low-level Io primitives
Populates the global `Deno.SeekMode`, `Deno.stdin`, `Deno.stdout`, and `Deno.stderr` objects

### Options
**`RuntimeOptions::extension_options::io_pipes`**
- Optional IO pipes to use for stdin, stdout, and stderr
- Default: `Some(deno_io::Stdio::default())`

### Usage Example
```ts
const encoder = new TextEncoder();
const data = encoder.encode("Hello world");
const bytesWritten = await Deno.stdout.write(data); // 11
```