//!
//! So the below is literally just skeptic's implementation, but with some modifications
//! for my use case (like better error management and not running from a test).
//!
use cargo_metadata::Edition;
use rustyscript::deno_core::anyhow;
use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    time::SystemTime,
};
use walkdir::WalkDir;

pub fn handle_test(filename: &str, code: &str) -> Result<(), anyhow::Error> {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target/debug"));
    let target_dir = PathBuf::from(target_dir);

    let out_dir = tempfile::Builder::new().prefix("rs-test").tempdir()?;
    let testcase_path = out_dir.path().join(filename);

    let target_triple = current_platform::CURRENT_PLATFORM;
    let root_dir = std::env::current_dir()?;

    let code = code.trim_start_matches('#').replace("\n# ", "\n");
    fs::write(&testcase_path, code.as_bytes())?;

    // OK, here's where a bunch of magic happens using assumptions
    // about cargo internals. We are going to use rustc to compile
    // the examples, but to do that we've got to tell it where to
    // look for the rlibs with the -L flag, and what their names
    // are with the --extern flag. This is going to involve
    // parsing fingerprints out of the lockfile and looking them
    // up in the fingerprint file.

    let mut deps_dir = target_dir.clone();
    deps_dir.push("deps");

    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));
    let mut cmd = Command::new(rustc);
    cmd.arg(testcase_path)
        .arg("--verbose")
        .arg("--crate-type=bin");

    // Find the edition

    // This has to come before "-L".
    let metadata_path = root_dir.join("Cargo.toml");
    let metadata = get_cargo_meta(&metadata_path).expect("failed to read Cargo.toml");
    let edition = *metadata
        .packages
        .iter()
        .map(|package| &package.edition)
        .max_by_key(|edition| u64::from_str(edition.as_str()).unwrap())
        .unwrap_or(&Edition::E2021);
    if edition != Edition::E2021 {
        cmd.arg(format!("--edition={}", edition));
    }

    cmd.arg("-L")
        .arg(&target_dir)
        .arg("-L")
        .arg(&deps_dir)
        .arg("--target")
        .arg(target_triple)
        .arg("--crate-name")
        .arg("test");

    for dep in get_rlib_dependencies(root_dir, target_dir).expect("failed to read dependencies") {
        cmd.arg("--extern");
        cmd.arg(format!(
            "{}",
            dep.libname,
            dep.rlib.to_str().expect("filename not utf8"),
        ));
    }

    let binary_path = out_dir.path().join("out.exe");
    cmd.arg("-o").arg(&binary_path);
    println!("{:?}", cmd);
    interpret_output("Compilation error: ", cmd)?;
    println!("Compiled to {}", binary_path.display());

    let mut cmd = Command::new(out_dir.path());
    cmd.current_dir(out_dir.path());
    interpret_output("Runtime error: ", cmd)?;

    Ok(())
}

fn interpret_output(prefix: &str, mut command: Command) -> Result<(), anyhow::Error> {
    let output = command.output()?;
    if !output.status.success() {
        let output = String::from_utf8(output.stderr)?.replace("\\n", "\n");
        Err(anyhow::anyhow!("{prefix}\n{output}"))
    } else {
        Ok(())
    }
}

// Retrieve the exact dependencies for a given build by
// cross-referencing the lockfile with the fingerprint file
fn get_rlib_dependencies(
    root_dir: PathBuf,
    target_dir: PathBuf,
) -> Result<Vec<Fingerprint>, anyhow::Error> {
    let lock = LockedDeps::from_path(root_dir)?;

    let fingerprint_dir = target_dir.join(".fingerprint/");
    let locked_deps: HashMap<String, String> = lock.collect();
    let mut found_deps: HashMap<String, Fingerprint> = HashMap::new();

    for finger in WalkDir::new(fingerprint_dir)
        .into_iter()
        .filter_map(|v| Fingerprint::from_path(v.ok()?.path()).ok())
    {
        let locked_ver = match locked_deps.get(&finger.name()) {
            Some(ver) => ver,
            None => continue,
        };

        // TODO this should be refactored to something more readable
        match (found_deps.entry(finger.name()), finger.version()) {
            (Entry::Occupied(mut e), Some(ver)) => {
                // we find better match only if it is exact version match
                // and has fresher build time
                if *locked_ver == ver && e.get().mtime < finger.mtime {
                    e.insert(finger);
                }
            }
            (Entry::Vacant(e), ver) => {
                // we see an exact match or unversioned version
                if ver.unwrap_or_else(|| locked_ver.clone()) == *locked_ver {
                    e.insert(finger);
                }
            }
            _ => (),
        }
    }

    Ok(found_deps
        .into_iter()
        .filter_map(|(_, val)| if val.rlib.exists() { Some(val) } else { None })
        .collect())
}

// An iterator over the root dependencies in a lockfile
#[derive(Debug)]
struct LockedDeps {
    dependencies: Vec<cargo_metadata::NodeDep>,
}

fn get_cargo_meta<P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>>(
    path: P,
) -> Result<cargo_metadata::Metadata, anyhow::Error> {
    Ok(cargo_metadata::MetadataCommand::new()
        .manifest_path(&path)
        .exec()?)
}

impl LockedDeps {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<LockedDeps, anyhow::Error> {
        let path = path.as_ref().join("Cargo.toml");
        let metadata = get_cargo_meta(&path)?;
        let deps = metadata
            .resolve
            .ok_or(anyhow::anyhow!("Missing dependency metadata"))?
            .nodes
            .into_iter()
            .flat_map(|node| node.deps.into_iter());
        let dependencies = deps.collect();
        Ok(LockedDeps { dependencies })
    }
}

impl Iterator for LockedDeps {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        let dep = self.dependencies.pop()?;
        let name = dep.name;
        let val = dep.pkg.repr;
        Some((name.replace("-", "_"), val.to_owned()))
    }
}

#[derive(Debug)]
struct Fingerprint {
    libname: String,
    version: Option<String>, // version might not be present on path or vcs deps
    rlib: PathBuf,
    mtime: SystemTime,
}

fn guess_ext(mut path: PathBuf, exts: &[&str]) -> Result<PathBuf, anyhow::Error> {
    for ext in exts {
        path.set_extension(ext);
        if path.exists() {
            return Ok(path);
        }
    }
    anyhow::bail!("Fingerprint")
}

impl Fingerprint {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Fingerprint, anyhow::Error> {
        let path = path.as_ref();

        // Use the parent path to get libname and hash, replacing - with _
        let mut captures = path
            .parent()
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .ok_or(anyhow::anyhow!("Fingerprint"))?
            .rsplit('-');
        let hash = captures.next().ok_or(anyhow::anyhow!("Fingerprint"))?;
        let mut libname_parts = captures.collect::<Vec<_>>();
        libname_parts.reverse();
        let libname = libname_parts.join("_");

        path.extension()
            .and_then(|e| if e == "json" { Some(e) } else { None })
            .ok_or(anyhow::anyhow!("Fingerprint"))?;

        let mut rlib = PathBuf::from(path);
        rlib.pop();
        rlib.pop();
        rlib.pop();
        rlib.push("deps");
        let mut dll = rlib.clone();
        rlib.push(format!("lib{}-{}", libname, hash));
        dll.push(format!("{}-{}", libname, hash));
        rlib = guess_ext(rlib, &["rlib", "so", "dylib"]).or_else(|_| guess_ext(dll, &["dll"]))?;

        Ok(Fingerprint {
            libname,
            version: None,
            rlib,
            mtime: fs::metadata(path)?.modified()?,
        })
    }

    fn name(&self) -> String {
        self.libname.clone()
    }

    fn version(&self) -> Option<String> {
        self.version.clone()
    }
}
