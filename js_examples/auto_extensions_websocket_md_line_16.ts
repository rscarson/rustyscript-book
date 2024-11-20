const ws = new WebSocket("ws://localhost:8080");

ws.onopen = () => {
  console.log("Connected");
  ws.send("Hello, world!");
  ws.close();
};
