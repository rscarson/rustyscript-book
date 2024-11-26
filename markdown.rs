use comrak::{arena_tree::NodeEdge, nodes::NodeValue, Arena, ComrakOptions};
use mdbook::{config::BuildConfig, errors::Error};
use std::path::PathBuf;
pub struct Location {
    pub file: String,
    pub line: usize,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{file}:{line}", file = self.file, line = self.line)
    }
}

pub struct MarkdownFile {
    pub path: PathBuf,
    pub content: String,
}
impl MarkdownFile {
    /// Parse the markdown file and add the tests to the TestSet
    pub fn parse(self) -> Vec<MarkdownItem> {
        let filename = self.path.to_string_lossy().to_string();
        let arena = Arena::new();
        let root = comrak::parse_document(&arena, &self.content, &ComrakOptions::default());
        let mut items = vec![];

        for edge in root.traverse() {
            if let NodeEdge::Start(node) = edge {
                let line = node.data.borrow().sourcepos.start.line;
                let location = Location {
                    file: filename.clone(),
                    line,
                };

                match &node.data.borrow().value {
                    NodeValue::Link(link) => {
                        let item = MarkdownItem::Link(Link {
                            url: link.url.to_string(),
                            label: link.title.to_string(),
                            location,
                        });
                        items.push(item);
                    }

                    NodeValue::CodeBlock(code) => {
                        let info: Vec<&str> = code.info.split(',').collect();
                        let code = code.literal.clone();
                        let item = MarkdownItem::CodeBlock(CodeBlock {
                            metadata: info.iter().map(|s| s.to_string()).collect(),
                            code,
                            location,
                        });
                        items.push(item);
                    }

                    _ => {}
                }
            }
        }

        items
    }

    pub fn load_all() -> Result<Vec<MarkdownFile>, Error> {
        let book = mdbook::book::load_book(
            "src",
            &BuildConfig {
                build_dir: PathBuf::from("book"),
                create_missing: false,
                use_default_preprocessors: true,
                extra_watch_dirs: Vec::new(),
            },
        )?;
        let mut files = vec![];
        for item in book.iter() {
            if let mdbook::BookItem::Chapter(ref ch) = *item {
                if let Some(path) = ch.path.clone() {
                    let file = MarkdownFile {
                        path,
                        content: ch.content.clone(),
                    };

                    files.push(file);
                }
            }
        }
        Ok(files)
    }

    pub fn parse_all(files: Vec<Self>) -> Vec<MarkdownItem> {
        let mut items = vec![];
        for file in files {
            items.extend(file.parse());
        }
        items
    }
}

pub enum MarkdownItem {
    Link(Link),
    CodeBlock(CodeBlock),
}

pub struct Link {
    pub url: String,
    pub label: String,
    pub location: Location,
}
impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.url)
    }
}
impl Link {
    pub fn is_relative(&self) -> bool {
        matches!(
            reqwest::Url::parse(&self.url),
            Err(url::ParseError::RelativeUrlWithoutBase)
        )
    }

    pub fn local_path(&self) -> Option<PathBuf> {
        if self.is_relative() {
            let path = std::path::Path::new(&self.url);

            // relative to the file location of the link
            let file = std::path::Path::new(&self.location.file);
            let file = file.parent()?;
            let path = std::path::Path::new("src").join(file).join(path);
            Some(path)
        } else {
            None
        }
    }

    pub fn exists(&self) -> Result<(), anyhow::Error> {
        if let Some(path) = self.local_path() {
            if path.exists() {
                Ok(())
            } else {
                anyhow::bail!("Target file does not exist: `{}`", path.display());
            }
        } else {
            //
            // A client with a known user-agent
            let client = reqwest::blocking::ClientBuilder::new()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()?;

            //
            // Check if the URL is reachable
            let url = reqwest::Url::parse(&self.url)
                .map_err(|e| anyhow::anyhow!("Failed to parse {}: {e}", self.url))?;

            //
            // Check if the URL is reachable
            let url_ = url.clone();
            let mut status = client
                .head(url.clone())
                .header("Accept", "text/html")
                .send()
                .map_err(move |e| anyhow::anyhow!("{:?}", e.with_url(url_.clone())))?
                .status();

            //
            // If we get 'Not Allowed'
            // try again with a GET request
            if status == reqwest::StatusCode::METHOD_NOT_ALLOWED {
                let url_ = url.clone();
                status = client
                    .get(url_.clone())
                    .header("Accept", "text/html")
                    .send()
                    .map_err(move |e| anyhow::anyhow!("{:?}", e.with_url(url_.clone())))?
                    .status();
            }

            //
            // Check if the URL target is valid
            if !status.is_success() {
                anyhow::bail!(
                    "Got status {} while fetching `{url}`",
                    status.canonical_reason().unwrap_or("Unknown")
                );
            }

            Ok(())
        }
    }
}

pub struct CodeBlock {
    pub metadata: Vec<String>,
    pub code: String,
    pub location: Location,
}
impl CodeBlock {
    pub fn is_rust(&self) -> bool {
        self.metadata.iter().any(|s| s == "rust")
    }

    pub fn is_js(&self) -> Option<String> {
        if self.metadata.iter().any(|s| s == "js" || s == "javascript") {
            Some("js".to_string())
        } else if self.metadata.iter().any(|s| s == "ts" || s == "typescript") {
            Some("ts".to_string())
        } else {
            None
        }
    }

    pub fn extension(&self) -> String {
        if self.is_rust() {
            "rs".to_string()
        } else if let Some(ext) = self.is_js() {
            ext
        } else {
            "txt".to_string()
        }
    }
}
