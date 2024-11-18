# Permissions
Many of the extension features allow for custom permissions to be set.  
This allows for a more fine-grained control over what can be accessed.

This can be done by specifying a value for the `permissions: Arc<dyn WebPermissions>,` field in `RuntimeOptions::web`.

The default value is `DefaultWebPermissions`, which simply allows everything.

[`AllowlistWebPermissionsSet`](https://docs.rs/rustyscript/latest/rustyscript/struct.AllowlistWebPermissionsSet.html) is also built-in, and allows for specific permissions to be turned on or off:

```rust
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
```

> You can also implement the [`WebPermissions`](https://docs.rs/rustyscript/latest/rustyscript/trait.WebPermissions.html) trait yourself, and use that instead for even more precise control.