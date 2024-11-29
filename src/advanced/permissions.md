# Permissions
Many of the extension features allow for custom permissions to be set.  
This allows for a more fine-grained control over what can be accessed.

This can be done by specifying a value for the `permissions: Arc<dyn WebPermissions>,` field in `RuntimeOptions::web`.

The default value is `DefaultWebPermissions`, which simply allows everything.

[`AllowlistWebPermissionsSet`](https://docs.rs/rustyscript/latest/rustyscript/struct.AllowlistWebPermissions.html) is also built-in, and allows for specific permissions to be turned on or off:

```rust
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
```

> You can also implement the [`WebPermissions`](https://docs.rs/rustyscript/latest/rustyscript/trait.WebPermissions.html) trait yourself, and use that instead for even more precise control.