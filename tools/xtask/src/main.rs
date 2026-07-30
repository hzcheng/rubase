use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let task = env::args().nth(1).unwrap_or_else(|| "help".to_owned());
    let root = workspace_root();

    match task.as_str() {
        "check" => check(&root),
        "test" => test(&root),
        "ci" => ci(&root),
        "package" => package(&root),
        "release-check" => release_check(&root),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        unknown => Err(Box::new(UnknownTask(unknown.to_owned()))),
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("xtask must live under tools/xtask")
        .to_owned()
}

fn check(root: &std::path::Path) -> Result<()> {
    cargo(root, ["fmt", "--all", "--check"])?;
    cargo(
        root,
        ["check", "--workspace", "--all-targets", "--all-features"],
    )?;
    cargo(
        root,
        [
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )
}

fn test(root: &std::path::Path) -> Result<()> {
    cargo(root, ["test", "--workspace", "--all-features"])
}

fn ci(root: &std::path::Path) -> Result<()> {
    check(root)?;
    test(root)?;
    cargo(root, ["doc", "--workspace", "--no-deps"])
}

fn package(root: &std::path::Path) -> Result<()> {
    cargo(root, ["package", "--package", "rubase", "--allow-dirty"])
}

fn release_check(root: &std::path::Path) -> Result<()> {
    ci(root)?;
    cargo(
        root,
        ["build", "--profile", "dist", "--package", "rubase-cli"],
    )?;
    package(root)
}

fn cargo<I, S>(root: &std::path::Path, arguments: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new("cargo")
        .args(arguments)
        .current_dir(root)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo exited with {status}").into())
    }
}

fn print_help() {
    println!(
        "\
Repository automation

Usage:
    cargo xtask <task>

Tasks:
    check          Format, compile, and lint the workspace
    test           Run all workspace tests
    ci             Run the complete pull-request validation suite
    package        Validate the public crate package contents
    release-check  Run CI, distribution builds, and package validation"
    );
}

#[derive(Debug)]
struct UnknownTask(String);

impl fmt::Display for UnknownTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown task `{}`; run `cargo xtask help`",
            self.0
        )
    }
}

impl Error for UnknownTask {}
