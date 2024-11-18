# MultiThreading
rustyscript is not thread-safe, due to limitations of the underlying engine, deno_core

The `worker` feature (enabled by default) gets around this by defining worker threads, which can be used in parallel.  
Workers use queries sent over channels to communicate with the main thread.

[`DefaultWorker`](https://docs.rs/rustyscript/latest/rustyscript/worker/struct.DefaultWorker.html) is a very simple built-in worker implementation:

```rust
use rustyscript::{worker::{DefaultWorker, DefaultWorkerQuery}, Error, Module};

fn main() -> Result<(), Error> {
    let worker = DefaultWorker::new(Default::default())?;

    // Instruct the worker to load a module
    let module = Module::new("test.js", "export function add(a, b) { return a + b; }");
    let query = DefaultWorkerQuery::LoadMainModule(module);
    worker.send(query)?;

    // This module ID will be used to refer to the module in future queries
    let module_id = worker.receive()?;

    // Now we can call the function
    let query = DefaultWorkerQuery::CallFunction(Some(module_id), "add".to_string(), vec![1.into(), 2.into()]);
    let response = worker.send_and_await(query)?;
    if let DefaultWorkerQuery::Value(v) = response {
        assert_eq!(v, 3.into());
    }
}
```

> Note that the worker has built-in helpers for many operations - this example could be far more succinct, as seen [here](https://github.com/rscarson/rustyscript/blob/master/examples/default_threaded_worker.rs).

However, for many applications this will, of course, not be sufficient, in which case you can implement your own worker - [See this example](https://github.com/rscarson/rustyscript/blob/master/examples/custom_threaded_worker.rs)

-----

The worker module also provides [`WorkerPool`](https://docs.rs/rustyscript/latest/rustyscript/worker/struct.WorkerPool.html), which can be used to manage multiple workers.
- See [this example](https://github.com/rscarson/rustyscript/blob/master/examples/worker_pool.rs)