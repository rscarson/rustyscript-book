# Static Runtimes
Since the runtime must be mutable, and cannot be safely sent between threads, it can be tricky to use it in a static context.

To this end, rustyscript includes the `static_runtime` module:
```rust
use rustyscript::{static_runtime, RuntimeOptions, Error};

// Can have the default options
static_runtime!(MY_DEFAULT_RUNTIME);

// Or you can provide your own
static_runtime!(MY_CUSTOM_RUNTIME, {
    let timeout = std::time::Duration::from_secs(5);
    RuntimeOptions {
        timeout,
        ..Default::default()
    }
});

fn main() -> Result<(), Error> {
    MY_DEFAULT_RUNTIME::with(|runtime| {
        runtime.eval::<()>("console.log('Hello, World!')")
    })
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