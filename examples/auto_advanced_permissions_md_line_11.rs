use rustyscript::{AllowlistWebPermissions, Error, Runtime, RuntimeOptions};
use std::sync::Arc;

fn main() -> Result<(), Error> {
    let permissions = Arc::new(AllowlistWebPermissions::default());
    let mut options = RuntimeOptions::default();
    options.extension_options.web.permissions = permissions.clone();

    let mut runtime = Runtime::new(options)?;

    // Fetching any URL will fail
    let result = runtime.eval::<()>("fetch('https://example.com')");
    assert!(result.is_err());

    // But if we allow it:
    permissions.allow_url("https://example.com");
    let result = runtime.eval::<()>("fetch('https://example.com')");
    assert!(result.is_ok());

    Ok(())
}
