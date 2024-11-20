const file = Deno.openSync("/dev/zero");
const buf = new Uint8Array(100);
file.readSync(buf);

const file = Deno.openSync("/dev/null", { write: true });
file.writeSync(buf);
