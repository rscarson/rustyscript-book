// Test runners for the Rustyscript book
// Can be run via CLI or through mdbook
//
#![allow(dead_code)]

use rustyscript::{ExtensionOptions, KvConfig, KvStore, Module, Runtime, RuntimeOptions};
use std::process::{Command, Stdio};

pub mod markdown;

fn run_file(path: &std::path::Path) -> Result<(), anyhow::Error> {
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: std::time::Duration::from_secs(5),
        extension_options: ExtensionOptions {
            kv_store: KvStore::new_local(None, None, KvConfig::default()),
            ..Default::default()
        },
        ..Default::default()
    })?;

    let module = Module::load(path)?;
    runtime.load_module(&module)?;
    Ok(())
}

/// Verify all links, panic if any are broken
pub fn test_links() {
    let files = markdown::MarkdownFile::load_all().unwrap();
    let items = markdown::MarkdownFile::parse_all(files);
    let links = items
        .iter()
        .filter_map(|item| match item {
            markdown::MarkdownItem::Link(link) => {
                println!("Checking link: `{}`", link.location);
                if let Err(e) = link.exists() {
                    let location = &link.location;
                    let e = e
                        .to_string()
                        .split('\n')
                        .map(|s| format!("   {}", s))
                        .collect::<Vec<_>>()
                        .join("\n");
                    Some(format!(" - {location}:\n{e}"))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if !links.is_empty() {
        eprintln!("{} broken links found:", links.len());
        for link in links {
            eprintln!("{link}");
        }
        panic!("Some links are broken");
    }
}

pub fn test_examples() {
    // Get all the rs files in the examples directory
    let mut targets = vec![];
    let js_examples = std::path::Path::new("examples");
    for entry in std::fs::read_dir(js_examples).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension() == Some("rs".as_ref()) {
            // Get only the filename, without the extension
            let filename = path.with_extension("");
            if let Some(filename) = filename.file_name() {
                let target = filename.to_string_lossy();
                targets.push(target.to_string());
            }
        }
    }

    // Run each example
    let mut errors = vec![];
    for target in targets {
        println!("Running `{}`", target);

        // To capture stderr as a string
        let stderr = std::process::Stdio::piped();
        let child = Command::new("cargo")
            .args(["run", "--example", &target])
            .stdout(Stdio::null())
            .stderr(stderr)
            .spawn();

        let child = match child {
            Ok(c) => c,
            Err(e) => {
                errors.push((target, e.to_string()));
                continue;
            }
        };

        match child.wait_with_output() {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                let err = String::from_utf8_lossy(&o.stderr);
                errors.push((target, err.to_string()));
            }
            Err(e) => {
                errors.push((target, e.to_string()));
            }
        }
    }

    if !errors.is_empty() {
        eprintln!("{} examples failed:", errors.len());
        for (target, e) in errors {
            let e = e
                .split('\n')
                .map(|s| format!("   {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            eprintln!(" - {}:\n{}", target, e);
        }
        panic!("Some examples failed");
    }
}

/// Run all the JS examples in the `js_examples` directory
/// Panics if any of the examples fail
pub fn test_js_examples() {
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
            let location = target.display();

            let err = err.to_string();
            if err.contains("operation canceled") {
                // Hack for HTTP server example abort
                continue;
            }

            let e = err
                .to_string()
                .split('\n')
                .map(|s| format!("   {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            errors.push(format!(" - {location}:\n   {e}"));
        }
    }

    if !errors.is_empty() {
        eprintln!("{} JS examples failed:", errors.len());
        for err in errors {
            eprintln!("{err}\n");
        }
        panic!("Some examples failed");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_links() {
        super::test_links();
    }

    #[test]
    fn test_js() {
        super::test_js_examples();
    }

    #[test]
    fn test_examples() {
        super::test_examples();
    }
}
