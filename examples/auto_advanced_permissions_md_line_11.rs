use rustyscript::{AllowlistWebPermissionsSet, Runtime, RuntimeOptions, WebOptions, Error};

fn main() -> Result<(), Error> {
    let permissions = Arc::new(AllowlistWebPermissionsSet::default());
    let runtime = Runtime::new(RuntimeOptions {
        web: Some(WebOptions {
            permissions,
            ..Default::default()
        }),
        ..Default::default()
    })?;

    // Fetching any URL will fail
    let result = runtime.eval::<()>("fetch('https://example.com')")?;
    assert!(result.is_err());

    // But if we allow it:
    permissions.allow_url("https://example.com");
    let result = runtime.eval::<()>("fetch('https://example.com')")?;
    assert!(result.is_ok());
}
