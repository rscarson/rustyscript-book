# Calling Rust from JavaScript
rustyscript supports registering rust functions to be callable from JavaScript. 
> A more advanced and performant way to call rust from JS is through [custom extensions](./custom_extensions.md).

```rust
use rustyscript::{serde_json, sync_callback, Error, Runtime};

fn main() -> Result<(), Error> {
    // Let's get a new runtime first
    let mut runtime = Runtime::new(Default::default())?;

    // We can use a normal function, if we wish
    // It can also be `move` if we want to capture some state
    runtime.register_function("echo", |args| {
        // Decode the input
        let input = args
            .first()
            .ok_or(Error::Runtime("No input".to_string()))
            .map(|v| serde_json::from_value::<String>(v.clone()))??;

        // Encode the output
        let output = format!("Hello, {input}!");
        Ok::<_, Error>(serde_json::Value::String(output))
    })?;

    // There is also a helper macro to create a callback
    // It will take care of deserializing arguments and serializing the result
    runtime.register_function(
        "add",
        sync_callback!(|a: i64, b: i64| {
            a.checked_add(b)
                .ok_or(Error::Runtime("Overflow".to_string()))
        }),
    )?;

    // The registered functions can now be called from JavaScript
    runtime.eval::<()>("rustyscript.functions.echo('test')")?;

    Ok(())
}
```

Async functions can be defined as well:

```rust
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
```