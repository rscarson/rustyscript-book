use rustyscript::{Error, Module, Runtime};

fn main() -> Result<(), Error> {
    let module = Module::new(
        "test.js",
        "
        export const my_func = async () => 42;
    ",
    );

    // Create a new runtime
    // We don't need to create a tokio runtime, as the runtime will create one for us
    let mut runtime = Runtime::new(Default::default())?;
    let handle = runtime.load_module(&module)?;
    let tokio_runtime = runtime.tokio_runtime();

    // Call the function, and await the result
    let value: u32 = tokio_runtime.block_on(async {
        runtime
            .call_function_async(Some(&handle), "my_func", &())
            .await
    })?;

    assert_eq!(value, 32);
    Ok(())
}
