use rustyscript::SnapshotBuilder;
use rustyscript_book_test::markdown::{MarkdownFile, MarkdownItem};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn generate_snapshot() -> Result<(), rustyscript::Error> {
    let snapshot = SnapshotBuilder::new(Default::default())?
        .with_expression("globalThis.importantFunction = () => 42")?
        .finish();

    let mut file = File::create("examples/example_snapshot.bin")?;
    file.write_all(&snapshot)?;
    Ok(())
}

fn extract_examples() -> Result<Vec<(PathBuf, String)>, anyhow::Error> {
    // Get the list of examples
    let files = MarkdownFile::load_all().unwrap();
    let items = MarkdownFile::parse_all(files);
    let code_blocks = items
        .iter()
        .filter_map(|item| match item {
            MarkdownItem::CodeBlock(block) => Some(block),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Generate the examples
    let mut examples = Vec::new();
    for block in code_blocks {
        let filename = format!(
            "auto_{}_line_{}.{}",
            block.location.file.replace(['/', '\\', '.'], "_"),
            block.location.line,
            block.extension(),
        );
        let parent = if block.is_rust() {
            Path::new("examples")
        } else {
            Path::new("js_examples")
        };
        let path = parent.join(filename);

        examples.push((path, block.code.clone()));
    }

    Ok(examples)
}

fn reset_examples(dir: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    if !dir.as_ref().exists() {
        std::fs::create_dir(dir.as_ref())?;
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|f| f.to_str());
        if let Some(filename) = filename {
            if filename.starts_with("auto_") {
                std::fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    // First we clear any examples starting with `auto_`
    reset_examples("examples")?;
    reset_examples("js_examples")?;

    // Then we regenerate the examples
    let examples = extract_examples()?;
    for (path, src) in examples {
        println!("Generating example: {:?}", path);
        let mut file = File::create(&path)?;
        file.write_all(src.as_bytes())?;
    }

    // Finally we generate the snapshot
    generate_snapshot()?;

    Ok(())
}
