# Snapshots
Deno's runtime supports the use of memory snapshots to skip the lengthy process of spinning up a new runtime and loading all the necessary code.
This can give gains of ~10x in startup time, but comes with some caveats:
- The snapshot must be generated on the same system it will be used on
- The extensions in the snapshot builder runtime must be the same, and the same order as the runtime that will use the snapshot
- The `snapshot_builder` feature must be enabled on rustyscript - This feature DOES preserve the integrity of the sandbox

## Generating a snapshot
The snapshot must be provided to the runtime via the `startup_snapshot` field of [`RuntimeOptions`](https://docs.rs/rustyscript/latest/rustyscript/struct.RuntimeOptions.html), which as a type of `&'static [u8]`.

Therefore the snapshot will typically be created in `build.rs` (short of using `Box::leak`, lazy_static, or similar)

The [`SnapshotBuilder`](https://docs.rs/rustyscript/latest/rustyscript/snapshot_builder/struct.SnapshotBuilder.html) struct is used to generate the snapshot, and can be used to pre-load modules into the snapshot. It has methods similar to the normal [`Runtime`](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html) struct.

Below is a sample that generates a snapshot with a pre-loaded module:

You could also do this in `build.rs` and write to `concat!(env!("OUT_DIR"), "/snapshot.bin")`

```rust
use rustyscript::{Error, Module, SnapshotBuilder};
use std::fs;

static SNAPSHOT_PATH: &str = "examples/snapshot.bin";

fn main() -> Result<(), Error> {
    // A module we want pre-loaded into the snapshot
    let module = Module::new(
        "my_module.js",
        "globalThis.importantFunction = function() {
            return 42;
        }",
    );

    let snapshot = SnapshotBuilder::new(Default::default())?
        .with_module(&module)?
        .finish();

    fs::write(SNAPSHOT_PATH, snapshot)?;
    Ok(())
}
```

Then, in order to use the snapshot:
```rust
use rustyscript::{Runtime, RuntimeOptions};

static SNAPSHOT: &[u8] = include_bytes!("example_snapshot.bin");

fn main() -> Result<(), rustyscript::Error> {
    let mut runtime = Runtime::new(RuntimeOptions {
        startup_snapshot: Some(SNAPSHOT),
        ..Default::default()
    })?;
    let important_value: u32 = runtime.eval("importantFunction()")?;
    assert_eq!(important_value, 42);
    Ok(())
    }
```

The startup time of the runtime will fall from ~50ms, to less than 1 with the snapshot.

---------------------

You could also generate the snapshot at runtime, in a lazy_static block, or similar:

```rust
use rustyscript::{Error, Module, Runtime, RuntimeOptions, SnapshotBuilder};
use std::sync::OnceLock;

static SNAPSHOT: OnceLock<Box<[u8]>> = OnceLock::new();
fn get_snapshot() -> &'static [u8] {
    SNAPSHOT.get_or_init(|| {
        let module = Module::new(
            "my_module.js",
            "globalThis.importantFunction = function() {
            return 42;
        }",
        );

        SnapshotBuilder::new(Default::default())
            .unwrap()
            .with_module(&module)
            .unwrap()
            .finish()
    })
}

fn main() -> Result<(), Error> {
    let mut runtime = Runtime::new(RuntimeOptions {
        startup_snapshot: Some(get_snapshot()),
        ..Default::default()
    })?;
    runtime.eval::<()>("1 + 1")?;
    Ok(())
}

```