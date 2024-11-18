use std::path::{Path, PathBuf};

use rustyscript::{Module, Runtime};

fn get_targets() -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut targets = vec![];
    let Some(arg1) = std::env::args().nth(1) else {
        anyhow::bail!("Expected a target file or --all");
    };

    // First check for the --all flag
    if arg1 == "--all" {
        // Get all the files in the js_examples directory
        let js_examples = std::path::Path::new("js_examples");
        for entry in std::fs::read_dir(js_examples).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                targets.push(path);
            }
        }

        return Ok(targets);
    }

    // Else it is a file path
    let path = std::path::Path::new(&arg1);
    let path = Path::new("js_examples").join(path);
    if path.exists() {
        targets.push(path.to_path_buf());
    } else if path.with_extension("js").exists() {
        targets.push(path.with_extension("js").to_path_buf());
    } else if path.with_extension("ts").exists() {
        targets.push(path.with_extension("ts").to_path_buf());
    } else {
        anyhow::bail!("Target file does not exist: `{}`", path.display());
    }

    Ok(targets)
}

fn run_file(path: &Path) -> Result<(), anyhow::Error> {
    let mut runtime = Runtime::new(Default::default())?;

    let module = Module::load(path)?;
    runtime.load_module(&module)?;
    Ok(())
}

fn main() {
    let mut errors = vec![];

    let targets = get_targets().unwrap();
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
