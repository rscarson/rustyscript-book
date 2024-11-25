# IO Extensions
## FFI
> Crate features: [`ffi`, `io_extensions`].  
> <https://crates.io/crates/deno_ffi>  

Populates the `Deno.dlopen`, `Deno.UnsafeCallback`, `Deno.UnsafePointer`, `Deno.UnsafePointerView`, `Deno.UnsafeFnPointer` globals.  
**Not sandbox safe. Off by default**


### Permissions
This extension is affected by the following methods in the permissions trait:
- `check_exec` - Check if FFI execution is allowed
- `check_read` - Of FFI exec is allowed, check if a specific file is allowed to be read

### Usage Example
```javascript
const buffer = new ArrayBuffer(1024);
const baseAddress = Deno.UnsafePointer.value(Deno.UnsafePointer.of(buffer));

const myCb = () => {
    console.log("Hello from FFI");
};
const cb = new Deno.UnsafeCallback({
    parameters: [],
    result: "void",
}, myCb);

const fnPointer = new Deno.UnsafeFnPointer(cb.pointer, {
    parameters: [],
    result: "void",
});

fnPointer.call();
cb.close();
```