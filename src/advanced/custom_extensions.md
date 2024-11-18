# Custom Extensions
> Note that the examples in this chapter require your crate to supply the **same version** of [`deno_core`](https://crates.io/crates/deno_core) as rustyscript uses - this version can be checked [here](https://crates.io/crates/rustyscript/latest/dependencies)

The most performant way to extend rustyscript is to use the `extension` feature of deno_core.

The following example demonstrates how to create a simple extension that adds two numbers together.

> **Important note:** All javascript files included in an extension MUST be included somewhere.
> I recommend using the file specified in `esm_entry_point` to include all other files.

```rust
use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use deno_core::{extension, op2};

#[op2(fast)]
#[bigint]
fn op_add_example(#[bigint] a: i64, #[bigint] b: i64) -> i64 {
    a + b
}

extension!(
    example_extension,                                  // The name of the extension
    ops = [op_add_example],                             // The ops to include in the extension
    esm_entry_point = "simple_extension.js",            // The entry point for the extension
    esm = [ dir "js_examples", "simple_extension.js" ], // The javascript files to include
);

fn main() -> Result<(), Error> {
    // If you were loading from a snapshot, you would use init_ops instead of init_ops_and_esm
    // let my_extension = example_extension::init_ops();
    let my_extension = example_extension::init_ops_and_esm();

    let mut runtime = Runtime::new(RuntimeOptions {
        extensions: vec![my_extension],
        ..Default::default()
    })?;

    let result: i64 = runtime.eval("my_add(5, 5)")?;
    assert_eq!(10, result);

    Ok(())
}
```

And the corresponding javascript file:

```javascript,norun
export const add = (a, b) => Deno.core.ops.op_add_example(a, b);
globalThis.my_add = add;
```

## Op2
`Op2` is a deno-provided macro that allows you to define an extension function.
- The supported types for arguments can be found listed in <https://github.com/denoland/deno_core/blob/main/ops/op2/valid_args.md>
- And return types, here: <https://github.com/denoland/deno_core/blob/main/ops/op2/valid_retvals.md>

The `fast` attribute used when possible to denote that the types involved can be converted fast. Don't stress too much on when to use this, as the compiler will tell you if you need it, or do not.

-----

Async functions can be defined as well. 

Let us break down a more complex example taken from rustyscript's own internals:
```rust,norun
#[op2(async)]
#[serde]
fn call_registered_function_async(
    #[string] name: String,
    #[serde] args: Vec<serde_json::Value>,
    state: &mut OpState,
) -> impl std::future::Future<Output = Result<serde_json::Value, Error>> {
    if state.has::<AsyncFnCache>() {
        let table = state.borrow_mut::<AsyncFnCache>();
        if let Some(callback) = table.get(&name) {
            return callback(args);
        }
    }

    Box::pin(std::future::ready(Err(Error::ValueNotCallable(name))))
}
```

- `#[op2(async)]` is used to denote that this is an async function - it must return a `Future` of some kind 
    - Potential pitfall: **the returned future cannot have a lifetime**
- `#[serde]` means that the return value will be a type decoded with `serde::Deserialize`.
    - In this fase, there return value is `Future<Output = Result<serde_json::Value, Error>>` - A future that resolves to a `serde_json::Value`, or an error.
- The arguments are annotated with `#[string]` and `#[serde]` to denote that they are a string, and a deserializable type, respectively.
    - The `state` argument is a special case that can be used for persistent storage inside of ops. In this case, it is used to store a cache.

And finally, the returned future is run through `Box::pin` before being returned.

-----

You can also customize the names of your extension modules  
If you instead define the extension as follows:
```rust,norun
extension!(
    example_extension,
    ops = [op_add_example],
    esm_entry_point = "example:calculator",
    esm = [ dir "examples/example_extension", "example:calculator" = "example_extension.js" ],
);
```

Then provide a schema whitelist in your `RuntimeOptions`:
```rust,norun
    let mut schema_whlist = HashSet::new();
    schema_whlist.insert("example:".to_string());
    options.schema_whlist = schema_whlist;
```

You would be able to `import { add } from "example:calculator";` from inside of your js modules.