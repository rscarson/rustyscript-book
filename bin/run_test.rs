fn main() {
    // Get 'mode' - the first argument passed to the program
    let mode = std::env::args().nth(1);
    match mode.unwrap_or_default().as_str() {
        "link" | "links" => rustyscript_book_test::test_links(),
        "example" | "examples" | "rust" => rustyscript_book_test::test_examples(),
        "js" | "javascript" => rustyscript_book_test::test_js_examples(),

        _ => {
            eprintln!("Usage: run_test [links|examples|js]");
            std::process::exit(1);
        }
    }
}
