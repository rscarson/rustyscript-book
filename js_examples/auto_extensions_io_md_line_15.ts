const encoder = new TextEncoder();
const data = encoder.encode("Hello world");
const bytesWritten = await Deno.stdout.write(data); // 11
