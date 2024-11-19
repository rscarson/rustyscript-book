use rustyscript::{js_value::Promise, Error, Module, Runtime};

fn main() -> Result<(), Error> {
    let module = Module::new(
        "test.js",
        "
        export const my_func = async () => 42;
    ",
    );

    // Create a new runtime
    let mut runtime = Runtime::new(Default::default())?;
    let handle = runtime.load_module(&module)?;

    // Call the function, and get the promise
    let promise: Promise<u32> = runtime.call_function_immediate(Some(&handle), "my_func", &())?;

    // Resolve the promise
    // You could instead call `into_future` here, and await it, for a non-blocking version
    let value = promise.into_value(&mut runtime)?;
    assert_eq!(value, 42);

    Ok(())
}
