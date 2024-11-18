use rustyscript::{json_args, import};

fn main() {
    let mut module = import("js/my_module.js").expect("Something went wrong!");
    let value: String = module.call("exported_function_name", json_args!()).expect("Could not get a value!");
    println!("{value}");
}
