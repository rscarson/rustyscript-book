const db = await Deno.openKv();
await db.set(["foo"], "bar");

const result = await db.get(["foo"]);
result.key; // ["foo"]
result.value; // "bar"
