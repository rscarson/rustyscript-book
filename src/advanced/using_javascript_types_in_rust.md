# Using JavaScript Types in Rust
Many functions on a [`Runtime`](https://docs.rs/rustyscript/latest/rustyscript/struct.Runtime.html) are generic over a type parameter `T`.  
This generic is used to specify the expected return type from the executed javascript code.

Any type implementing [`Serde::Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) can be used as the type parameter `T`.

For example, you could call `runtime.eval::<i32>("1 + 1")` to evaluate the javascript code `1 + 1` and expect the result to be an `i32`.
- You could also get a string representation of the result by calling `runtime.eval::<String>("1 + 1")`.

If you don't care about the type of value being returned you can use [`Rustyscript::Undefined`](https://docs.rs/rustyscript/latest/rustyscript/type.Undefined.html), or simply use `()`:

For example, to call a function with side-effects, where no value is returned, you would do:  
`runtime.eval::<()>("console.log('Hello, World!')");`