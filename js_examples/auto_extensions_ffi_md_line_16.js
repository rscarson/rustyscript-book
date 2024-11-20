const buffer = new ArrayBuffer(1024);
const baseAddress = Deno.UnsafePointer.value(Deno.UnsafePointer.of(buffer));

const throwCb = () => throw new Error("Error");
const cb = new Deno.UnsafeCallback({
    parameters: [],
    result: "void",
}, throwCb);

const fnPointer = new Deno.UnsafeFnPointer(cb.pointer, {
    parameters: [],
    result: "void",
});

assertThrows(() => fnPointer.call());
cb.close();
