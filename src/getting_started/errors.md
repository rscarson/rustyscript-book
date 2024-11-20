# Getting Started
## Errors
The crate includes an [error type](https://docs.rs/rustyscript/latest/rustyscript/error/enum.Error.html) that can catch any errors that can occur during runtime execution.

A few important variants include:

> `Error::JsError`  
> This is the most common error you will see, and is thrown by the JS runtime. It contains the error message and the stack trace.

> `Error::JsonDecode`  
> You will see this error if you try to decode a return value to an incompatible type.

> `Error::Timeout`  
> This error is thrown when runtime execution takes longer than the runtime's configured timeout.

> `Error::ValueNotCallable`  
> This error is thrown when you try to call a value that is not a function.

> `Error::ValueNotFound`  
> This error is thrown when you try to access a value that does not exist.

-----

You can get JS-side errors as a code-highlighted string by calling [`error.as_highlighted()`](https://docs.rs/rustyscript/latest/rustyscript/error/enum.Error.html#method.as_highlighted)

You will get a string of the form:
```
| let x = 1 + 2  
|       ^  
= Unexpected token '='  
```

You can customize this output to include the filename, line or column numbers.