# Getting Started
## Calling Functions
You may have noticed the `&()` in the previous chapter's example - this is because the function `getValue` does not take any arguments.

Arguments are normally passed as references to tuples, for example:  
`&("test", 1)`, `&("test")`, or `&()`

They can be of any combination of types that implement `serde::Serialize`

Arguments can also be references to sized values, such as:  
`&"test".to_string()` or `&1`

> Up to 16 arguments can be passed in this way, if you need more, you can use [`big_json_args!`](https://docs.rs/rustyscript/latest/rustyscript/macro.big_json_args.html)  
>
> <div class="warning">
>   <strong>Important note</strong>
>
>   `big_json_args!` is significantly slower — benchmark tests show it can be nearly 1,000 times slower than using a smaller argument set.  
>   
>   Use sparingly to avoid performance bottlenecks.
> </div>
