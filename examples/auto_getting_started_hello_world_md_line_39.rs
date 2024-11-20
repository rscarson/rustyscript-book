use rustyscript::{Runtime, RuntimeOptions, Module, Undefined};
use std::time::Duration;

fn main() -> Result<(), rustyscript::Error> {
    let module = Module::new(
        "test.js",
        "
        let internalValue = 0;
        export const load = (value) => internalValue = value;
        export const getValue = () => internalValue;
        "
    );

    // Create a new runtime
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(50), // Stop execution by force after 50ms
        default_entrypoint: Some("load".to_string()), // Run this as the entrypoint function if none is registered
        ..Default::default()
    })?;

    // The handle returned is used to get exported functions and values from that module.
    // We then call the entrypoint function, but do not need a return value.
    // Load can be called multiple times, and modules can import other loaded modules
    // Using `import './filename.js'`
    let module_handle = runtime.load_module(&module)?;
    runtime.call_entrypoint::<Undefined>(&module_handle, &(2))?;

    // Functions don't need to be the entrypoint to be callable!
    let _internal_value: i64 = runtime.call_function(Some(&module_handle), "getValue", &())?;
    Ok(())
}
