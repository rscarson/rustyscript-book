const channel = new BroadcastChannel("my_channel");
channel.onmessage = (event) => {
  console.log(event.data);
};
channel.postMessage("Hello, world!");
channel.close();
