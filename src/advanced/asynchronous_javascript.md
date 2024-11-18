# Asynchronous JavaScript
By default, rustyscript can resolve most asynchronous javascript, including promises, by blocking the current thread.  
In many cases, however, this will not be sufficient.

To this end, many functions will have `_async` and `_immediate` variants.
- `_async` functions will return a `Future` that that resolves when:
    - The event loop is resolved, and
    - If the value is a promise, the promise is resolved
- `_immediate` functions will return a value immediately, but will not resolve promises or advance the event loop.
    - Promises can be returned by specifying the return type as [`js_value::Promise`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Promise.html)
    - The event loop should be run using [`Runtime::await_event_loop`]

A long-form example can be found [here](https://github.com/rscarson/rustyscript/blob/master/examples/async_javascript.rs)

## Futures, with async functions
In the example below, we define a simple async function in javascript, and call it from rust.  
The runtime's own tokio runtime is used to resolve the future.

The event loop, and the implicit promise resolution, is handled by the runtime, transparently.  
This is the simplest way to use async functions.

```rust
use rustyscript::{js_value::Promise, Error, Module, Runtime};

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
    tokio_runtime.block_on(async {
        let value: u32 = runtime.call_function_async(Some(&handle), "my_func", &()).await?;
        assert_eq!(value, 42);
    })?;

    Ok(())
}
```

Another way is with [`js_value::Promise`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Promise.html)

Normally, calling a function would resolve the promise. So we need to use `call_function_immediate` instead.

```rust
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
    let value = promise.into_value()?;
    assert_eq!(value, 42);

    Ok(())
}
```

## Background Tasks

Sometimes, a module may begin background tasks, which would tie up the event loop for a long time, causing the `async` and `blocking` functions to hang.

In this case, you can use one of the following methods:
- [Runtime::await_event_loop](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html#method.await_event_loop)
- [Runtime::block_on_event_loop](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html#method.block_on_event_loop)
- [Runtime::advance_event_loop](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html#method.advance_event_loop)

Combined with the `immediate` variants of most functions, this will allow you full control over execution of the event loop.
> See [this example](https://github.com/rscarson/rustyscript/blob/master/examples/background_tasks.rs) for a demonstration.

## Threading
The last way to handle async functions is to use a separate thread.
> See [Multi-Threading](../advanced/multithreading.md) for more information.