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
