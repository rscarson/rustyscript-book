use rustyscript::{
    extensions::deno_broadcast_channel::BroadcastChannel, Error, Module, Runtime, RuntimeOptions,
};

fn main() -> Result<(), Error> {
    // Let's extract the channel from the options
    let options = RuntimeOptions::default();
    let channel = options.extension_options.broadcast_channel.clone();

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
    let resource = channel.subscribe().unwrap();

    // Use a built-in helper function to serialize the data for transmission
    let data: Vec<u8> = runtime.call_function(None, "broadcast_serialize", &("foo"))?;
    tokio_runtime.block_on(async {
        channel
            .send(&resource, "my_channel".to_string(), data)
            .await
            .unwrap();
    });

    // And run the event loop to completion
    runtime.block_on_event_loop(Default::default(), None)?;
    Ok(())
}
