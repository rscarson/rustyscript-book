# Static Runtimes
Since the runtime must be mutable, and cannot be safely sent between threads, it can be tricky to use it in a static context.

To this end, rustyscript includes the `static_runtime` module:
```rust
use rustyscript::{static_runtime, RuntimeOptions, Error};

// Can have the default options
static_runtime!(MY_DEFAULT_RUNTIME);

// Or you can provide your own
static_runtime!(MY_CUSTOM_RUNTIME, {
    let mut options = RuntimeOptions::default();
    options.timeout = std::time::Duration::from_secs(5);
    options
});

fn main() -> Result<(), Error> {
    // And it can be accessed either with a lock:
    MY_DEFAULT_RUNTIME.with(|rt| {
        let mut lock = rt.lock()?;
        lock.runtime().eval::<()>("console.log('Hello, World!')")
    })?;

    // Or with a closure
    MY_CUSTOM_RUNTIME.with(|rt| {
        rt.with_runtime(|runtime| {
            runtime.eval::<()>("console.log('Hello, World!')")
        })
    })??;

    Ok(())
}
```

-----

Under the hood, a static runtime is effectively:
```rust,norun
thread_local! {
    static MY_RUNTIME: OnceCell<RefCell<Result<Runtime, Error>>>;
}
```

Which provides thread safety, static initialization, interior mutability, and initializer error handling.

> **Note:** While it is possible to initialize a `StaticRuntime` object directly, it is not recommended, as it bypasses the thread_local safety layer.