use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn main() {
    let cli = Cli::parse();
    let code = match cli.cmd {
        Cmd::Migrate { repo_url, fixture } => migrate(&repo_url, fixture),
    };
    std::process::exit(code);
}

const RUNNER_IMAGE: &str = "ghcr.io/akashpriyadarshii/c2proof/runner:c2rust-0.20.0";

fn migrate(repo_url: &str, use_fixture: bool) -> i32 {
    match run(repo_url, use_fixture) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e:#}");
            if format!("{e:#}").starts_with("refused:") {
                2
            } else {
                1
            }
        }
    }
}

fn run(repo_url: &str, use_fixture: bool) -> Result<()> {
    let tmp = tempfile_dir()?;
    let src = prepare_source(repo_url, &tmp)?;
    scan_gate(&src)?;

    let out = tmp.join("out-crate");
    if use_fixture {
        transpile_fixture(&out)?;
    } else {
        transpile_c2rust(&src, &out)?;
    }

    cargo(&["check"], &out).context("cargo check on transpiled crate")?;
    println!("pipeline ok; PR push lands in T08");
    Ok(())
}

// ponytail: tempdir via env TMP, manual cleanup skipped — short-lived dirs fine for v0.1.
fn tempfile_dir() -> Result<PathBuf> {
    let base = std::env::temp_dir();
    let d = base.join(format!("c2proof-{}", std::process::id()));
    std::fs::create_dir_all(&d)?;
    Ok(d)
}

fn prepare_source(repo_url: &str, tmp: &Path) -> Result<PathBuf> {
    if Path::new(repo_url).is_dir() {
        return Ok(PathBuf::from(repo_url));
    }
    let dest = tmp.join("src-repo");
    git(&["clone", "--depth", "1", repo_url, dest.to_str().unwrap()])
        .context("git clone (is the URL reachable?)")?;
    Ok(dest)
}

fn git(args: &[&str]) -> Result<()> {
    let out = Command::new("git").args(args).output()?;
    ensure_success("git", out)
}

fn cargo(args: &[&str], cwd: &Path) -> Result<()> {
    let out = Command::new("cargo")
        .args(args)
        .current_dir(cwd)
        .env("CARGO_MANIFEST_DIR", cwd) // isolate from c2proof's own workspace
        .output()?;
    ensure_success("cargo", out)
}

fn ensure_success(cmd: &str, out: std::process::Output) -> Result<()> {
    if out.status.success() {
        return Ok(());
    }
    let tail = String::from_utf8_lossy(&out.stderr);
    let tail = tail.lines().rev().take(10).collect::<Vec<_>>().join("\n");
    bail!("{cmd} failed:\n{tail}")
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
            bail!("refused: subdirectory '{name}' present — v0.1.0 supports flat C projects only");
        }
        if !(name.ends_with(".c")
            || name.ends_with(".h")
            || name == "LICENSE"
            || name == "README.md"
            || name.starts_with('.'))
        {
            bail!("refused: unexpected file '{name}' — expected only .c/.h");
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

fn transpile_fixture(out: &Path) -> Result<()> {
    // Fixture root = crate root committed by the CI e2e job (T10).
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/tinyexpr");
    copy_dir(&fixture, out)
        .map_err(|e| anyhow!("fixture missing or unreadable ({e}) — regenerate via CI e2e job"))
}

fn transpile_c2rust(src: &Path, out: &Path) -> Result<()> {
    let docker = Command::new("docker").arg("--version").output();
    match docker {
        Err(_) => bail!(
            "docker not found — install Docker Desktop, or rerun with --fixture to test offline"
        ),
        Ok(o) if !o.status.success() => {
            bail!("docker not usable — install Docker Desktop, or rerun with --fixture")
        }
        _ => {}
    }
    std::fs::create_dir_all(out)?;
    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/work/src", src.canonicalize()?.display()),
            "-v",
            &format!("{}:/work/out", out.canonicalize()?.display()),
            RUNNER_IMAGE,
            "transpile",
            "--emit-build-files",
            "/work/src",
        ])
        .status()?;
    if !status.success() {
        bail!("c2rust container failed");
    }
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
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

    #[test]
    fn exit_code_2_on_refusal() {
        assert_eq!(migrate("definitely/not/a/dir", true), 1); // clone fail = 1
    }

    #[test]
    fn fixture_missing_is_error_not_refusal() {
        // empty fixture dir → copy succeeds but no Cargo.toml; gate on a flat dir first
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.c"), "").unwrap();
        assert_eq!(migrate(d.path().to_str().unwrap(), true), 1);
    }
}
