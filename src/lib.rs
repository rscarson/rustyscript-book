// Test runners for the Rustyscript book
// Can be run via CLI or through mdbook
//
#![allow(dead_code)]

use rustyscript::{Module, Runtime};

pub mod markdown;

fn run_file(path: &std::path::Path) -> Result<(), anyhow::Error> {
    let mut runtime = Runtime::new(Default::default())?;

    let module = Module::load(path)?;
    runtime.load_module(&module)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_links() {
        let files = markdown::MarkdownFile::load_all().unwrap();
        let items = markdown::MarkdownFile::parse_all(files);
        let links = items
            .iter()
            .filter_map(|item| match item {
                markdown::MarkdownItem::Link(link) => Some(link),
                _ => None,
            })
            .collect::<Vec<_>>();

        for link in links {
            if let Err(e) = link.exists() {
                panic!("- {}:\n{}\n", link.location, e);
            }
        }
    }

    fn test_js() {
        // Get all the files in the js_examples directory
        let mut targets = vec![];
        let js_examples = std::path::Path::new("js_examples");
        for entry in std::fs::read_dir(js_examples).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                targets.push(path);
            }
        }

        let mut errors = vec![];
        for target in targets {
            println!("Running `{}`", target.display());
            if let Err(err) = run_file(&target) {
                errors.push(format!("`{}`: {}", target.display(), err));
            }
            println!("\n\n\n");
        }

        if !errors.is_empty() {
            println!("Errors:");
            for err in errors {
                println!("{}", err);
            }
        }
    }
}
