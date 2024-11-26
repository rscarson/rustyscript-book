use rustyscript::{static_runtime, Error, RuntimeOptions};

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
    MY_DEFAULT_RUNTIME::with(|runtime| runtime.eval::<()>("console.log('Hello, World!')"))
}
