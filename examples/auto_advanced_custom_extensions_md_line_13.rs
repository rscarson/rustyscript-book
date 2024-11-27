use rustyscript::{Error, Runtime, RuntimeOptions};
use deno_core::{extension, op2};

#[op2(fast)]
#[bigint]
fn op_add_example(#[bigint] a: i64, #[bigint] b: i64) -> i64 {
    a + b
}

extension!(
    example_extension,                                              // The name of the extension
    ops = [op_add_example],                                         // The ops to include in the extension
    esm_entry_point = "ext:example_extension/simple_extension.js",  // The entry point for the extension
    esm = [ dir "js_examples", "simple_extension.js" ],             // The javascript files to include
);

fn main() -> Result<(), Error> {
    // If you were loading from a snapshot, you would use init_ops instead of init_ops_and_esm
    // let my_extension = example_extension::init_ops();
    let my_extension = example_extension::init_ops_and_esm();

    let mut runtime = Runtime::new(RuntimeOptions {
        extensions: vec![my_extension],
        ..Default::default()
    })?;

    let result: i64 = runtime.eval("my_add(5, 5)")?;
    assert_eq!(10, result);

    Ok(())
}
