fn main() {
    let result: i64 = rustyscript::evaluate("5 + 5").expect("The expression was invalid!");
    assert_eq!(result, 10);
}
