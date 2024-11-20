use rustyscript::{serde_json, Error, Runtime};

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

    // The registered functions can now be called from JavaScript
    runtime.eval::<()>("rustyscript.functions.echo('test')")?;

    Ok(())
}
