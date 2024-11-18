# Using JavaScript Types in Rust
Many functions on a [`Runtime`](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html) are generic over a type parameter `T`, used to specify the expected return type from the executed javascript code.
- Any type implementing [`Serde::Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) can be used.

Here is a simple example of calling the same function with different return types:
```rust
use rustyscript::{Runtime, Error};

fn main() -> Result<(), Error> {
    let mut runtime = Runtime::new(Default::default())?;

    let number: i32 = runtime.eval("1 + 1")?;
    let string: String = runtime.eval("1 + 1")?;
    let float: f64 = runtime.eval("1 + 1")?;

    println!("Number: {}", number);
    println!("String: {}", string);
    println!("Float: {}", float);

    Ok(())
}
```

> If you don't care about the type of value being returned you can use:  
> [`Rustyscript::Undefined`](https://docs.rs/rustyscript/latest/rustyscript/type.Undefined.html)
> Or simply use `()`:
>
> For example, to call a function with side-effects, where no value is returned, you would do:  
> `runtime.eval::<()>("console.log('Hello, World!')");`

## Special Types
The [`js_value`](https://docs.rs/rustyscript/latest/rustyscript/js_value/index.html) module defines a number of special types that can be used, which map more-or-less directly to JavaScript types.

**It is important to note that all of these cannot outlive their runtime's, or be used on other runtimes**

### [`js_value::Function`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Function.html)
A Deserializable javascript function, that can be stored and used later.

Can be called with [`Function::call`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Function.html#method.call).
`async` and `immediate` variants exist
> See [Async JavaScript](../advanced/asynchronous_javascript.md) for details on `async` and `immediate`.

### [`js_value::Promise<T>`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Promise.html)
A stored javascript promise, that can be resolved or rejected later.

You can turn `Promise<T>` into `Future<Output = T>` by calling `Promise::into_future` This allows you to export multiple concurrent promises without borrowing the runtime mutably

You can also use `Promise::into_value` to block until the promise is resolved, and get the value.

### [`js_value::Map`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Map.html)
Read-only instance of a javascript Object. Allows conversion into a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) (Skips any non-utf8 keys).

This type can be faster for large objects than directly deserializing into a rust type.

### [`js_value::String`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.String.html)
A Javascript UTF-16 string, used to preserve data which can be lost converting to a Rust `String`.

### [`js_value::Value`](https://docs.rs/rustyscript/latest/rustyscript/js_value/enum.Value.html)
A generic type able to represent any JavaScript value. This mimics the behavior of the `any` type in TypeScript.

The primary use-case is to defer the normal type-decoding if the type is not known right away. This is done with [`Value::try_into`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Value.html#method.try_into), which must be called with the same runtime as the value was created with.

It can also be used to directly get the underlying deno value, using [`Value::into_v8`](https://docs.rs/rustyscript/latest/rustyscript/js_value/struct.Value.html#method.into_v8).