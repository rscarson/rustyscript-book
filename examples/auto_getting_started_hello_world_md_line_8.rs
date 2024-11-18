use rustyscript::{json_args, Runtime, Module};

fn main() -> Result<(), rustyscript::Error> {
    let module = Module::new(
        "test.js",
        "
        export default (string, integer) => {
            console.log(`Hello world: string=${string}, integer=${integer}`);
            return 2;
        }
        "
    );

    let value: usize = Runtime::execute_module(
        &module, vec![],
        Default::default(),
        json_args!("test", 5)
    )?;

    assert_eq!(value, 2);
    Ok(())
}
