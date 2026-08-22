use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::path::Path;

/// C to Rust migration with verification report. Verifier-first.
#[derive(Parser)]
#[command(name = "c2proof", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Migrate a flat C repo into a compiling Rust port PR + verification report.
    Migrate {
        repo_url: String,
        /// Run against the committed golden fixture instead of real c2rust.
        #[arg(long)]
        fixture: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Migrate { repo_url, fixture } => migrate(&repo_url, fixture),
    }
}

fn migrate(repo_url: &str, _fixture: bool) -> Result<()> {
    // ponytail: scan operates on a local path only; remote clone arrives with T06.
    if !Path::new(repo_url).is_dir() {
        bail!("not a local directory (remote clone lands in T06): {repo_url}");
    }
    scan_gate(Path::new(repo_url))?;
    println!("flat C project accepted; pipeline wiring lands in T06");
    Ok(())
}

/// Refuse anything that is not a flat .c/.h directory. Exit code 2.
fn scan_gate(dir: &Path) -> Result<()> {
    let mut c_files = 0usize;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type()?.is_dir() {
            if name == ".git" || name == "target" {
                continue;
            }
            bail!(
                "refused: subdirectory '{}' present — v0.1.0 supports flat C projects only",
                name
            );
        }
        if !(name.ends_with(".c")
            || name.ends_with(".h")
            || name == "LICENSE"
            || name == "README.md"
            || name.starts_with('.'))
        {
            bail!("refused: unexpected file '{}' — expected only .c/.h", name);
        }
        if name.ends_with(".c") || name.ends_with(".h") {
            c_files += 1;
        }
    }
    if c_files == 0 {
        bail!("refused: no .c/.h files found");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_flat() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.c"), "").unwrap();
        std::fs::write(d.path().join("a.h"), "").unwrap();
        assert!(scan_gate(d.path()).is_ok());
    }

    #[test]
    fn refuses_subdir() {
        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir(d.path().join("src")).unwrap();
        assert!(scan_gate(d.path()).is_err());
    }

    #[test]
    fn refuses_no_c() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("x.py"), "").unwrap();
        assert!(scan_gate(d.path()).is_err());
    }
}
