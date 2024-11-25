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
