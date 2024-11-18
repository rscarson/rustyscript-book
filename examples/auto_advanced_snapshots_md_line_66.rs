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

