use rustyscript::{sync_callback, Error, Runtime};

fn main() -> Result<(), Error> {
    // Let's get a new runtime first
    let mut runtime = Runtime::new(Default::default())?;

    // We can use the helper macro to create a callback
    // It will take care of deserializing arguments and serializing the result
    runtime.register_function(
        "add",
        sync_callback!(|a: i64, b: i64| {
            a.checked_add(b)
                .ok_or(Error::Runtime("Overflow".to_string()))
        }),
    )?;

    // The registered functions can now be called from JavaScript
    runtime.eval::<()>("rustyscript.functions.add(1, 2)")?;

    Ok(())
}
