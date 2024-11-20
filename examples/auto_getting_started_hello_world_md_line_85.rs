use rustyscript::{import};

fn main() {
    let mut module = import("js/my_module.js").expect("Something went wrong!");
    let value: String = module.call("exported_function_name", &()).expect("Could not get a value!");
    println!("{value}");
}
