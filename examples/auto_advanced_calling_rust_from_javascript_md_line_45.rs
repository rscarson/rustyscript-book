use rustyscript::{async_callback, Error, Runtime};

fn main() -> Result<(), Error> {
    let mut runtime = Runtime::new(Default::default())?;

    // There is also an async version
    runtime.register_async_function(
        "asyncEcho",
        async_callback!(|input: String| {
            async move { Ok::<_, Error>(format!("Hello, {input}!")) }
        }),
    )?;

    // The registered functions can now be called from JavaScript
    runtime.eval::<()>("rustyscript.async_functions.asyncEcho('test')")?;

    Ok(())
}
