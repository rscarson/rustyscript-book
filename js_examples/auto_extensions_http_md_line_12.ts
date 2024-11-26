// The abort signal is used to close the server
const ac = new AbortController();

try {
  const server = Deno.serve(
    { signal: ac.signal },
    (_req) => new Response("Hello, world")
  );
} catch (err) {
  console.error("Operation aborted");
}

// Close the server after 1 second
setTimeout(() => ac.abort(), 1000);
