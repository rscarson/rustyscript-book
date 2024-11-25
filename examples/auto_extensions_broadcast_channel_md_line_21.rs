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
