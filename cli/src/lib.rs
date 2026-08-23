use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

/// C to Rust migration with verification report. Verifier-first.
#[derive(Parser)]
#[command(name = "c2proof", version)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Migrate a flat C repo into a compiling Rust port PR + verification report.
    Migrate {
        repo_url: String,
        /// Run against the committed golden fixture instead of real c2rust.
        #[arg(long)]
        fixture: bool,
    },
}

pub const RUNNER_IMAGE: &str = "ghcr.io/akashpriyadarshii/c2proof/runner:c2rust-0.20.0";

impl Cli {
    pub fn run(self) -> i32 {
        match self.cmd {
            Cmd::Migrate { repo_url, fixture } => migrate(&repo_url, fixture),
        }
    }
}

pub fn migrate(repo_url: &str, use_fixture: bool) -> i32 {
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

pub fn run(repo_url: &str, use_fixture: bool) -> Result<()> {
    let tmp = tempfile_dir()?;
    let src = prepare_source(repo_url, &tmp)?;
    scan_gate(&src)?;

    let out = tmp.join("out-crate");
    if use_fixture {
        transpile_fixture(&out)?;
    } else {
        transpile_c2rust(&src, &out)?;
    }

    let verify = verify(&out)?;
    std::fs::write(
        out.join("REPORT.md"),
        render_report(repo_url, &verify, use_fixture),
    )
    .context("write REPORT.md")?;

    match publish_pr(&out, repo_url) {
        Ok(Some(url)) => println!("PR: {url}"),
        Ok(None) => {
            println!("PR skipped (set C2PROOF_GITHUB_TOKEN on a github.com repo to enable)")
        }
        Err(e) => bail!("PR publish failed: {e:#}"),
    }
    println!(
        "pipeline ok; REPORT.md written; build {}",
        if verify.build_ok { "green" } else { "FAILED" }
    );
    if !verify.build_ok {
        // CI logs need the reason; REPORT.md alone isn't visible mid-run
        eprintln!("--- build failure evidence ---\n{}", verify.error_excerpt);
    }
    Ok(())
}

// --- verification + report (T07) ---

pub struct Verification {
    pub build_ok: bool,
    pub clippy_warnings: usize,
    /// First error lines from compiler/clippy stderr when the build failed
    pub error_excerpt: String,
    /// per-file `unsafe fn` counts, sorted desc
    pub unsafe_fns: Vec<(String, usize)>,
}

/// Build gate + evidence collection. Clippy failure still produces a report
/// (arch doc: "cargo build fails → still open PR, report marks ❌"), so only
/// hard tool errors bubble up.
fn verify(out: &Path) -> Result<Verification> {
    if let Some(pin_path) = find_toolchain_pin(out) {
        eprintln!("toolchain pin found: {}", pin_path.display());
        let channel = ensure_toolchain_for(&pin_path)?;
        install_pinned_toolchain(&channel)?;
    }
    let mut res =
        cargo_capture(&["clippy"], out).context("cargo clippy (is rustup stable installed?)")?;
    if !res.status.success() {
        // Mechanism-independent fallback: whatever selected an absent toolchain
        // (pin file, env, cargo config), the error names it. Install + retry once.
        let stderr = String::from_utf8_lossy(&res.stderr);
        if let Some(channel) = stderr_missing_toolchain(&stderr) {
            eprintln!("clippy named missing toolchain '{channel}' — installing and retrying once");
            install_pinned_toolchain(&channel)?;
            res = cargo_capture(&["clippy"], out)
                .context("cargo clippy retry (is rustup stable installed?)")?;
        }
    }
    let stderr_raw = String::from_utf8_lossy(&res.stderr);
    let stderr = strip_ansi(&stderr_raw);
    let stdout_raw = String::from_utf8_lossy(&res.stdout);
    let stdout = strip_ansi(&stdout_raw);

    let clippy_warnings = stderr
        .lines()
        .filter(|l| l.trim_start().starts_with("warning: "))
        .count();

    let error_excerpt = if res.status.success() {
        String::new()
    } else {
        let mut lines = Vec::new();
        let mut capturing = false;
        for line in stderr.lines() {
            let t = line.trim_start();
            if t.starts_with("error") || t.starts_with("Caused by:") {
                capturing = true;
            }
            if capturing {
                lines.push(line);
                if lines.len() >= 20 {
                    break;
                }
            }
        }
        if lines.is_empty() {
            let source = if stderr.trim().is_empty() {
                &stdout
            } else {
                &stderr
            };
            let fallback: Vec<_> = source
                .lines()
                .filter(|l| !l.trim().is_empty())
                .rev()
                .take(20)
                .collect();
            fallback.into_iter().rev().collect::<Vec<_>>().join("\n")
        } else {
            lines.join("\n")
        }
    };
    Ok(Verification {
        build_ok: res.status.success(),
        clippy_warnings,
        error_excerpt,
        unsafe_fns: count_unsafe_fns(out),
    })
}

pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_esc = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_esc = true;
        } else if in_esc {
            if c.is_ascii_alphabetic() {
                in_esc = false;
            }
        } else {
            out.push(c);
        }
    }
    out
}

struct Captured {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn cargo_capture(args: &[&str], cwd: &Path) -> std::io::Result<Captured> {
    let out = Command::new("cargo")
        .args(args)
        .current_dir(cwd)
        .env_remove("CARGO_MANIFEST_DIR")
        .env("CARGO_TERM_COLOR", "never")
        .output()?;
    Ok(Captured {
        status: out.status,
        stdout: out.stdout,
        stderr: out.stderr,
    })
}

/// Regex-level per architecture.md — no syn.
fn count_unsafe_fns(crate_root: &Path) -> Vec<(String, usize)> {
    let mut v = Vec::new();
    fn walk(dir: &Path, crate_root: &Path, v: &mut Vec<(String, usize)>) {
        for e in std::fs::read_dir(dir).into_iter().flatten() {
            let Ok(e) = e else { continue };
            let p = e.path();
            if p.is_dir() {
                walk(&p, crate_root, v);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let n = std::fs::read_to_string(&p)
                    .map(|s| s.matches("unsafe fn").count())
                    .unwrap_or(0);
                v.push((
                    p.strip_prefix(crate_root)
                        .unwrap_or(&p)
                        .display()
                        .to_string(),
                    n,
                ));
            }
        }
    }
    walk(crate_root, crate_root, &mut v);
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.retain(|(_, n)| *n > 0);
    v
}

fn render_report(source: &str, v: &Verification, fixture_mode: bool) -> String {
    let mut r = String::new();
    r.push_str("# c2proof Verification Report\n\n");
    r.push_str(&format!("- Source: `{source}`\n"));
    r.push_str(&format!(
        "- Tool: c2rust 0.20.0{}\n",
        if fixture_mode {
            " (golden-fixture replay)"
        } else {
            ""
        }
    ));
    r.push_str(
        "- **This is a mechanical translation. It is NOT safe Rust and NOT reviewed code.**\n\n",
    );
    r.push_str(&format!(
        "## Build: {}\n\n",
        if v.build_ok {
            "✅ compiles (`cargo clippy` ran clean of errors)"
        } else {
            "❌ FAILED"
        }
    ));
    r.push_str(&format!(
        "## Clippy warnings captured: {}\n\n",
        v.clippy_warnings
    ));
    if !v.error_excerpt.is_empty() {
        r.push_str("## Error excerpt\n\n```text\n");
        r.push_str(&v.error_excerpt);
        r.push_str("\n```\n\n");
    }
    r.push_str("## Unsafe functions per file\n\n");
    if v.unsafe_fns.is_empty() {
        r.push_str("None detected (regex-level scan).\n");
    } else {
        r.push_str("| file | `unsafe fn` count |\n|---|---|\n");
        for (f, n) in &v.unsafe_fns {
            r.push_str(&format!("| `{f}` | {n} |\n"));
        }
    }
    r
}

// ponytail: tempdir via std env TMP, manual cleanup skipped — short-lived dirs fine for v0.1.
fn tempfile_dir() -> Result<PathBuf> {
    let base = std::env::var_os("C2PROOF_WORK_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let d = base.join(format!("c2proof-{}-{}", std::process::id(), fastrand_id()));
    std::fs::create_dir_all(&d)?;
    Ok(d)
}

fn fastrand_id() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
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

fn ensure_success(cmd: &str, out: std::process::Output) -> Result<()> {
    if out.status.success() {
        return Ok(());
    }
    let tail = String::from_utf8_lossy(&out.stderr);
    let tail = tail.lines().rev().take(10).collect::<Vec<_>>().join("\n");
    bail!("{cmd} failed:\n{tail}")
}

/// Refuse anything that is not a flat .c/.h directory. Exit code 2.
/// Metadata files (Makefile, .gitignore, LICENSE, README, *.md) are inert —
/// never parsed or executed, so their presence doesn't break scope.
pub fn scan_gate(dir: &Path) -> Result<()> {
    const INERT: &[&str] = &[
        "makefile",
        ".gitignore",
        ".gitattributes",
        "citation.cff",
        "contributing",
        "changelog",
    ];
    const INERT_DIRS: &[&str] = &["doc", "docs"];
    let mut c_files = 0usize;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let lower = name.to_lowercase();
        if entry.file_type()?.is_dir() {
            if name == ".git"
                || name == "target"
                || name.starts_with('.')
                || INERT_DIRS.contains(&lower.as_ref())
            {
                continue;
            }
            bail!("refused: subdirectory '{name}' present — v0.1.0 supports flat C projects only");
        }
        let inert = INERT.contains(&lower.as_ref())
            || lower.starts_with("license")
            || lower.starts_with("readme")
            || lower.ends_with(".md")
            || name.starts_with('.');
        if !(name.ends_with(".c") || name.ends_with(".h") || inert) {
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

fn fixture_root() -> PathBuf {
    // Test/CI override; defaults to the golden fixture regenerated by the e2e job.
    std::env::var_os("C2PROOF_FIXTURE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/tinyexpr")
        })
}

fn transpile_fixture(out: &Path) -> Result<()> {
    let fixture = fixture_root();
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
    let mut args = vec![
        "run".to_string(),
        "--rm".to_string(),
        "-v".to_string(),
        format!("{}:/work/src", src.canonicalize()?.display()),
        "-v".to_string(),
        format!("{}:/work/out", out.canonicalize()?.display()),
        RUNNER_IMAGE.to_string(),
        "transpile".to_string(),
    ];
    // c2rust 0.20 cannot parse a directory argument — list .c files explicitly
    let c_files = list_c_files(src)?;
    if c_files.is_empty() {
        bail!("refused: no .c entry points found under {}", src.display());
    }
    for f in &c_files {
        args.push(format!("/work/src/{f}"));
    }
    args.push("-o".to_string());
    args.push("/work/out".to_string());
    args.push("--emit-build-files".to_string());

    let status = Command::new("docker").args(&args).status()?;
    if !status.success() {
        bail!("c2rust container failed");
    }
    Ok(())
}

pub fn list_c_files(dir: &Path) -> Result<Vec<String>> {
    let mut v = Vec::new();
    for e in std::fs::read_dir(dir)? {
        let e = e?;
        let name = e.file_name().to_string_lossy().into_owned();
        if e.file_type()?.is_file() && name.ends_with(".c") && !name.starts_with('.') {
            v.push(name);
        }
    }
    v.sort();
    Ok(v)
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

// --- PR publish (T08) ---

const BRANCH: &str = "c2proof/port";

/// Returns Some(pr_url) when a PR was opened, None when publishing doesn't apply
/// (no token, or source isn't a github.com repo). Token stays out of argv where possible.
pub fn publish_pr(out: &Path, source_url: &str) -> Result<Option<String>> {
    let Ok(token) = std::env::var("C2PROOF_GITHUB_TOKEN") else {
        return Ok(None);
    };
    let Some((owner, repo)) = parse_github_slug(source_url) else {
        return Ok(None);
    };

    git(&["init", "-b", BRANCH, out.to_str().unwrap()]).context("git init in output crate")?;
    // never commit build artifacts into the PR
    std::fs::write(out.join(".gitignore"), "target/\n")?;
    git(&["-C", out.to_str().unwrap(), "add", "-A"])?;
    git(&[
        "-C",
        out.to_str().unwrap(),
        "-c",
        "user.name=c2proof[bot]",
        "-c",
        "user.email=c2proof@users.noreply.github.com",
        "commit",
        "-m",
        "c2proof: mechanical Rust port (unsafe, unreviewed — see REPORT.md)",
    ])
    .context("git commit (empty tree?)")?;
    git(&[
        "push",
        "--force",
        &format!("https://x-access-token:{token}@github.com/{owner}/{repo}.git"),
        &format!("{BRANCH}:{BRANCH}"),
        "--quiet",
    ])
    .context(
        "git push (check C2PROOF_GITHUB_TOKEN scopes: contents:write + pull_requests:write)",
    )?;

    open_pr(&token, &owner, &repo, source_url)
}

/// Accepts https://github.com/o/r, .../o/r.git, o/r.
pub fn parse_github_slug(url: &str) -> Option<(String, String)> {
    let s = url
        .trim_end_matches("/")
        .strip_suffix(".git")
        .unwrap_or(url);
    let s = s.strip_prefix("https://github.com/").unwrap_or(s);
    let mut it = s.split('/');
    match (it.next(), it.next()) {
        (Some(o), Some(r)) if !o.is_empty() && !r.is_empty() && it.next().is_none() => {
            Some((o.to_string(), r.to_string()))
        }
        _ => None,
    }
}

fn open_pr(token: &str, owner: &str, repo: &str, source_url: &str) -> Result<Option<String>> {
    let body = format!(
        r#"{{"title":"c2proof: mechanical Rust port","head":"{BRANCH}","base":"main","body":{}}}"#,
        serde_json_escape(&format!(
            "Mechanical c2rust translation of `{source_url}`.\n\n**Not safe Rust, not reviewed code.** Verification evidence in REPORT.md."
        ))
    );
    let out = Command::new("curl")
        .args([
            "-sf",
            "-X",
            "POST",
            "-H",
            &format!("Authorization: Bearer {token}"),
            "-H",
            "Accept: application/vnd.github+json",
            "-d",
            &body,
            &format!("https://api.github.com/repos/{owner}/{repo}/pulls"),
        ])
        .output()?;
    if out.status.success() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let url = stdout
            .split("\"html_url\":\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .map(|s| s.to_string());
        return Ok(Some(url.unwrap_or_else(|| {
            format!("https://github.com/{owner}/{repo}/pulls")
        })));
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    // -f swallows body; 422 = PR already exists → treat as success-ish but report it
    if stderr.contains("422") || stderr.contains("already exists") {
        println!("PR already open on {BRANCH}");
        return Ok(Some(format!("https://github.com/{owner}/{repo}/pulls")));
    }
    bail!("GitHub API rejected PR creation: {stderr}")
}

fn serde_json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Read and parse the pinned toolchain channel from `<crate_dir>/rust-toolchain`.
/// Parsing only — no rustup calls. Caller checks the file exists first (see
/// `verify`), so a missing file surfaces here as a read error.
const TOOLCHAIN_FILE_NAMES: [&str; 2] = ["rust-toolchain", "rust-toolchain.toml"];

/// Locate a rust-toolchain pin at crate root or one level below (nested
/// workspace layouts). Checks legacy bare filename and TOML name.
pub fn find_toolchain_pin(crate_dir: &Path) -> Option<std::path::PathBuf> {
    for name in TOOLCHAIN_FILE_NAMES {
        let p = crate_dir.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    for entry in std::fs::read_dir(crate_dir).ok()?.flatten() {
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            for name in TOOLCHAIN_FILE_NAMES {
                let p = entry.path().join(name);
                if p.is_file() {
                    return Some(p);
                }
            }
        }
    }
    None
}

/// Extract a toolchain name from rustup's "not installed for the toolchain
/// 'X'" error. This is the catch-all when no pin file is found but something
/// else selected an absent toolchain.
pub fn stderr_missing_toolchain(stderr: &str) -> Option<String> {
    stderr.lines().find_map(|l| {
        let idx = l.find("not installed for the toolchain '")?;
        let rest = &l[idx + "not installed for the toolchain '".len()..];
        let end = rest.find('\'')?;
        Some(rest[..end].to_string())
    })
}

/// Parse-only: read and extract the pinned channel. No rustup calls.
pub fn ensure_toolchain_for(pin_path: &Path) -> Result<String> {
    let contents = std::fs::read_to_string(pin_path)
        .with_context(|| format!("read {}", pin_path.display()))?;
    parse_toolchain_channel(&contents).ok_or_else(|| {
        anyhow!(
            "could not parse toolchain channel from {}",
            pin_path.display()
        )
    })
}

/// Regex-level, no serde. Supports plain-string ("nightly-2022-08-08") and
/// TOML `[toolchain]\nchannel = "..."` forms.
fn parse_toolchain_channel(contents: &str) -> Option<String> {
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return None;
    }
    for line in trimmed.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("channel") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let channel = rest.trim().trim_matches(|c| c == '"' || c == '\'').trim();
        if !channel.is_empty() {
            return Some(channel.to_string());
        }
    }
    let first = trimmed.lines().next().unwrap_or("").trim();
    if first.is_empty() {
        None
    } else {
        Some(first.to_string())
    }
}

/// Idempotent: skips install if rustup already lists `channel`.
fn install_pinned_toolchain(channel: &str) -> Result<()> {
    let list = Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .context("rustup toolchain list (is rustup installed and on PATH?)")?;
    let already_installed = String::from_utf8_lossy(&list.stdout)
        .lines()
        .any(|l| l.trim_start().starts_with(channel));
    if already_installed {
        return Ok(());
    }
    let status = Command::new("rustup")
        .args([
            "toolchain",
            "install",
            channel,
            "--profile",
            "minimal",
            "--component",
            "clippy",
        ])
        .status()
        .with_context(|| format!("rustup toolchain install {channel}"))?;
    if !status.success() {
        bail!(
            "failed to install pinned toolchain '{channel}' (from rust-toolchain) — install it manually: `rustup toolchain install {channel} --profile minimal --component clippy`"
        );
    }
    Ok(())
}
