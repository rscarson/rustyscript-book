use rustyscript::{Runtime, Module};

fn main() -> Result<(), rustyscript::Error> {
    let module = Module::new(
        "test.js",
        "
        export default (string) => {
            console.log(`Hello world: string=${string}`);
            return 2;
        }
        "
    );

    let value: usize = Runtime::execute_module(
        &module, vec![],
        Default::default(),
        &("test"),
    )?;

    assert_eq!(value, 2);
    Ok(())
}
