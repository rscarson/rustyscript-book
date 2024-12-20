# Network Extensions
## Broadcast Channel
> Crate features: [`broadcast_channel`, `network_extensions`]
> <https://crates.io/crates/deno_broadcast_channel/>  
> <https://html.spec.whatwg.org/multipage/web-messaging.html>

Populates the global `BroadcastChannel` object.  
**Not sandbox safe. Off by default**

### Options
**`RuntimeOptions::extension_options::broadcast_channel`**
- The channel object used by the extension.
- Default: `InMemoryBroadcastChannel::default()`

The channel can be cloned, and shared between runtimes to communicate between them.

### Usage Example
Below is an example of using the broadcast_channel extension to transfer data between Rust and JavaScript.
The same technique can be used to communicate between seperate runtimes, by sharing the channel object.

```rust
use rustyscript::{
    BroadcastChannelWrapper, Error, Module, Runtime, RuntimeOptions,
};

fn main() -> Result<(), Error> {
    // Let's extract the channel from the options
    let options = RuntimeOptions::default();
    let channel = options.extension_options.broadcast_channel.clone();
    let channel = BroadcastChannelWrapper::new(&channel, "my_channel")?;

    // Set up our runtime
    let mut runtime = Runtime::new(options)?;
    let tokio_runtime = runtime.tokio_runtime();

    // Load our javascript
    let module = Module::new(
        "test.js",
        "
        const channel = new BroadcastChannel('my_channel');
        channel.onmessage = (event) => {
            console.log('Got message: ' + event.data);
            channel.close();
        };
    ",
    );
    tokio_runtime.block_on(runtime.load_module_async(&module))?;

    // Use a built-in helper function to serialize the data for transmission
    channel.send_sync(&mut runtime, "foo")?;

    // And run the event loop to completion
    runtime.block_on_event_loop(Default::default(), None)?;
    Ok(())
}
```