use rustyscript::{AllowlistWebPermissions, Error, Runtime, RuntimeOptions};
use std::sync::Arc;

fn main() -> Result<(), Error> {
    let permissions = Arc::new(AllowlistWebPermissions::default());
    let mut options = RuntimeOptions::default();
    options.extension_options.web.permissions = permissions.clone();

    let mut runtime = Runtime::new(options)?;

    // Set up a fetch function
    runtime.eval::<()>(
        "globalThis.doFetch = async function() { await fetch('https://example.com') }",
    )?;

    // Fetching any URL will fail
    assert!(runtime.call_function::<()>(None, "doFetch", &()).is_err());

    // But if we allow it:
    permissions.allow_url("https://example.com/");
    runtime.call_function::<()>(None, "doFetch", &())?;

    Ok(())
}
