use rustyscript::{Runtime, Error};

fn main() -> Result<(), Error> {
    let mut runtime = Runtime::new(Default::default())?;

    let number: i32 = runtime.eval("1 + 1")?;
    let string: String = runtime.eval("`${1 + 1}`")?;
    let float: f64 = runtime.eval("1 + 1")?;

    println!("Number: {}", number);
    println!("String: {}", string);
    println!("Float: {}", float);

    Ok(())
}
