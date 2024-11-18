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
