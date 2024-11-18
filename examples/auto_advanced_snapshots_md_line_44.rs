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
