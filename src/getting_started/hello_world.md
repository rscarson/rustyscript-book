# Getting Started
## Hello World

Here is a very basic use of this crate to execute a JS module. It will:
- Create a basic runtime
- Load a javascript module,
- Call a function and the resulting value
```rust
use rustyscript::{json_args, Runtime, Module};

fn main() -> Result<(), rustyscript::Error> {
    let module = Module::new(
        "test.js",
        "
        export default (string, integer) => {
            console.log(`Hello world: string=${string}, integer=${integer}`);
            return 2;
        }
        "
    );

    let value: usize = Runtime::execute_module(
        &module, vec![],
        Default::default(),
        json_args!("test", 5)
    )?;

    assert_eq!(value, 2);
    Ok(())
}
```

Modules can also be loaded from the filesystem with [`Module::load`](https://docs.rs/rustyscript/latest/rustyscript/struct.Module.html#method.load) or [`Module::load_dir`](https://docs.rs/rustyscript/latest/rustyscript/struct.Module.html#method.load_dir)  
Or included statically with the [`module!`](https://docs.rs/rustyscript/latest/rustyscript/macro.module.html) and [`include_module!`](https://docs.rs/rustyscript/latest/rustyscript/macro.include_module.html) macros.

-----

Here is a more detailed version example above, which breaks down the steps instead of using the one-liner [`Runtime::execute_module`](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html#method.execute_module):
```rust
use rustyscript::{json_args, Runtime, RuntimeOptions, Module, Undefined};
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
    runtime.call_entrypoint::<Undefined>(&module_handle, json_args!(2))?;

    // Functions don't need to be the entrypoint to be callable!
    let _internal_value: i64 = runtime.call_function(Some(&module_handle), "getValue", json_args!())?;
    Ok(())
}
```

### Single Expression Evaluation
If all you need is the result of a single javascript expression, you can use:
```rust
fn main() {
    let result: i64 = rustyscript::evaluate("5 + 5").expect("The expression was invalid!");
    assert_eq!(result, 10);
}
```

-----

Or, if you just need to import one Javascript module for use in rust:
```rust
use rustyscript::{json_args, import};

fn main() {
    let mut module = import("js/my_module.js").expect("Something went wrong!");
    let value: String = module.call("exported_function_name", json_args!()).expect("Could not get a value!");
    println!("{value}");
}
```

There are a few other utilities included, such as [`validate`](https://docs.rs/rustyscript/latest/rustyscript/fn.validate.html) and [`resolve_path`](https://docs.rs/rustyscript/latest/rustyscript/fn.resolve_path.html)

----