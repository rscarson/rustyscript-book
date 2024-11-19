# MultiThreading
rustyscript is not thread-safe, due to limitations of the underlying engine, deno_core

The `worker` feature (enabled by default) gets around this by defining worker threads, which can be used in parallel.  
Workers use queries sent over channels to communicate with the main thread.

> **Important Note:** You must call [`init_platform`](https://docs.rs/rustyscript/latest/rustyscript/fn.init_platform.html) If you are using runtimes on multiple threads.
> The only exception is `WorkerPool`, sen below, which will call `init_platform` for you.

[`DefaultWorker`](https://docs.rs/rustyscript/latest/rustyscript/worker/struct.DefaultWorker.html) is a very simple built-in worker implementation:

```rust
use deno_core::serde_json;
use rustyscript::{
    worker::{DefaultWorker, DefaultWorkerQuery, DefaultWorkerResponse},
    Error, Module,
};

fn main() -> Result<(), Error> {
    let worker = DefaultWorker::new(Default::default())?;

    // Instruct the worker to load a module
    // We can do with provided helper functions
    let module = Module::new("test.js", "export function add(a, b) { return a + b; }");
    let query = DefaultWorkerQuery::LoadModule(module);
    let response = worker.as_worker().send_and_await(query)?;
    let module_id = if let DefaultWorkerResponse::ModuleId(id) = response {
        id
    } else {
        let e = Error::Runtime(format!("Unexpected response: {:?}", response));
        return Err(e);
    };

    // Now we can call the function
    let query = DefaultWorkerQuery::CallFunction(
        Some(module_id),
        "add".to_string(),
        vec![1.into(), 2.into()],
    );
    let response = worker.as_worker().send_and_await(query)?;
    if let DefaultWorkerResponse::Value(v) = response {
        let value: i32 = serde_json::from_value(v)?;
        assert_eq!(value, 3);
    }

    Ok(())
}
```

> Note that the worker has built-in helpers for many operations - this example could be far more succinct using the `load_module` and `call_function` helpers,
> which take care of the query/response boilerplate, as seen [here](https://github.com/rscarson/rustyscript/blob/master/examples/default_threaded_worker.rs).

However, for many applications the default worker will not be sufficient, in which case you can implement your own worker - [See this example](https://github.com/rscarson/rustyscript/blob/master/examples/custom_threaded_worker.rs)

-----

The worker module also provides [`WorkerPool`](https://docs.rs/rustyscript/latest/rustyscript/worker/struct.WorkerPool.html), which can be used to manage multiple workers.
- See [this example](https://github.com/rscarson/rustyscript/blob/master/examples/worker_pool.rs)