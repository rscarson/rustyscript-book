// The abort signal is used to close the server
const ac = new AbortController();

const server = Deno.serve(
  { signal: ac.signal },
  (_req) => new Response("Hello, world")
);

// Close the server after 1 second
setTimeout(() => ac.abort(), 1000);
