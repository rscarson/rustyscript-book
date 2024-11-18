use rustyscript::{Runtime, Error};

fn main() -> Result<(), Error> {
    let mut runtime = Runtime::new()?;

    let number: i32 = runtime.eval("1 + 1")?;
    let string: String = runtime.eval("1 + 1")?;

    // You don't need to specify the type if it can be inferred
    let float = runtime.eval("1 + 1")? + 1.0f64;

    println!("Number: {}", number);
    println!("String: {}", string);
    println!("Float: {}", float);

    Ok(())
}
