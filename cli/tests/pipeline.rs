//! Integration tests: run the real pipeline logic against temp dirs.
//! Goal: nothing reaches CI that can fail on logic we control.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Tests mutate process-global env vars (C2PROOF_FIXTURE_DIR, C2PROOF_WORK_DIR);
/// cargo runs them in parallel threads → serialize every env-touching test.
static ENV_LOCK: Mutex<()> = Mutex::new(());

// --- helpers ---

fn write_crate(root: &Path, name: &str) {
    fs::create_dir_all(root).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"lib.rs\"\n"),
    )
    .unwrap();
    fs::write(root.join("lib.rs"), "pub fn f() -> i32 { 42 }\n").unwrap();
}

fn write_flat_c(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("a.c"), "int f(void) { return 1; }\n").unwrap();
    fs::write(dir.join("a.h"), "int f(void);\n").unwrap();
}

struct FixtureGuard;
impl Drop for FixtureGuard {
    fn drop(&mut self) {
        std::env::remove_var("C2PROOF_FIXTURE_DIR");
    }
}

fn set_fixture_dir(path: &Path) -> FixtureGuard {
    std::env::set_var("C2PROOF_FIXTURE_DIR", path);
    FixtureGuard
}
// --- scan gate ---

#[test]
fn gate_accepts_flat() {
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    assert!(c2proof::scan_gate(d.path()).is_ok());
}

#[test]
fn gate_accepts_dotfiles_and_git() {
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::write(d.path().join(".gitignore"), "").unwrap();
    fs::create_dir(d.path().join(".git")).unwrap();
    assert!(c2proof::scan_gate(d.path()).is_ok());
}

#[test]
fn gate_refuses_subdir_exit2_reason() {
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::create_dir(d.path().join("src")).unwrap();
    let err = c2proof::scan_gate(d.path()).unwrap_err();
    assert!(err.to_string().contains("refused:"), "{err}");
}

#[test]
fn gate_refuses_configure_script() {
    // build systems we'd have to invoke are out of scope → refuse
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::write(d.path().join("configure"), "#!/bin/sh\n").unwrap();
    let err = c2proof::scan_gate(d.path()).unwrap_err();
    assert!(err.to_string().contains("refused:"));
}

#[test]
fn gate_refuses_no_c_files() {
    let d = tempfile::tempdir().unwrap();
    fs::write(d.path().join("x.py"), "").unwrap();
    assert!(c2proof::scan_gate(d.path()).is_err());
}

#[test]
fn gate_refuses_empty_dir() {
    let d = tempfile::tempdir().unwrap();
    assert!(c2proof::scan_gate(d.path()).is_err());
}

// --- exit-code mapping ---

#[test]
fn gate_accepts_metadata_files() {
    // tinyexpr ships a Makefile — inert metadata must not block migration
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::write(d.path().join("Makefile"), "all:\n\tcc a.c\n").unwrap();
    fs::write(d.path().join("LICENSE"), "MIT").unwrap();
    fs::write(d.path().join(".gitignore"), "o\n").unwrap();
    assert!(c2proof::scan_gate(d.path()).is_ok());
}

#[test]
fn gate_accepts_actual_tinyexpr_layout() {
    // exact upstream layout: github.com/codeplea/tinyexpr root listing.
    // This is the acceptance target — keep in sync with e2e.yml.
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    for f in [
        ".github",
        "doc", // dirs
        ".gitignore",
        "CITATION.cff",
        "CONTRIBUTING",
        "LICENSE",
        "Makefile",
        "README.md",
        "benchmark.c",
        "example.c",
        "example2.c",
        "example3.c",
        "minctest.h",
        "repl.c",
        "smoke.c",
        "tinyexpr.c",
        "tinyexpr.h",
    ] {
        let p = d.path().join(f);
        if f == ".github" || f == "doc" {
            fs::create_dir(&p).unwrap();
            fs::write(p.join("placeholder.md"), "x").unwrap();
        } else {
            fs::write(p, "x").unwrap();
        }
    }
    assert!(
        c2proof::scan_gate(d.path()).is_ok(),
        "raw tinyexpr clone must pass the gate"
    );
}

#[test]
fn gate_still_refuses_scripts() {
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::write(d.path().join("build.sh"), "echo hi").unwrap();
    assert!(c2proof::scan_gate(d.path()).is_err());
}

#[test]
fn exit_2_on_refusal_shape() {
    // flat dir but fixture broken? no — refusal must come from the gate itself
    let d = tempfile::tempdir().unwrap();
    write_flat_c(d.path());
    fs::create_dir(d.path().join("sub")).unwrap();
    assert_eq!(c2proof::migrate(d.path().to_str().unwrap(), true), 2);
}

#[test]
fn exit_1_on_unreachable_url() {
    assert_eq!(c2proof::migrate("/definitely/not/a/real/path-xyz", true), 1);
}

struct WorkDirGuard(PathBuf);
impl Drop for WorkDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
        std::env::remove_var("C2PROOF_WORK_DIR");
    }
}

fn set_work_dir() -> (WorkDirGuard, PathBuf) {
    let d = tempfile::tempdir_in(std::env::temp_dir()).unwrap().keep();
    std::env::set_var("C2PROOF_WORK_DIR", &d);
    (WorkDirGuard(d.clone()), d)
}

// --- report content + build-failure semantics ---

#[test]
fn build_failure_still_completes_and_reports() {
    // arch invariant: build fails → pipeline still completes, REPORT.md marks ❌, exit 0
    let _env = ENV_LOCK.lock().unwrap();
    let src = tempfile::tempdir().unwrap();
    write_flat_c(src.path());
    let fix = tempfile::tempdir().unwrap();
    write_crate(fix.path(), "broken-port");
    fs::write(
        fix.path().join("lib.rs"),
        "unsafe fn f() -> i32 { 42 }\nfn g() { f() }\n",
    )
    .unwrap();
    let _g = set_fixture_dir(fix.path());
    let (wd, path) = set_work_dir();

    let code = c2proof::migrate(src.path().to_str().unwrap(), true);
    assert_eq!(code, 0, "build failure must not abort the pipeline");

    // run() nests a c2proof-* workdir under the configured base
    let mut report_path = None;
    for e in fs::read_dir(&path).unwrap() {
        let p = e.unwrap().path().join("out-crate/REPORT.md");
        if p.exists() {
            report_path = Some(p);
            break;
        }
    }
    let report =
        fs::read_to_string(report_path.expect("REPORT.md not found under work dir")).unwrap();
    assert!(report.contains("❌ FAILED"), "{report}");
    assert!(report.contains("NOT safe Rust"), "{report}");
    assert!(
        report.contains("| `lib.rs` | 1 |"),
        "unsafe table row missing: {report}"
    );
    drop(wd);
}

#[test]
fn slug_parsing_variants() {
    assert_eq!(
        c2proof::parse_github_slug("https://github.com/o/r"),
        Some(("o".into(), "r".into()))
    );
    assert_eq!(
        c2proof::parse_github_slug("https://github.com/o/r.git"),
        Some(("o".into(), "r".into()))
    );
    assert_eq!(
        c2proof::parse_github_slug("o/r"),
        Some(("o".into(), "r".into()))
    );
    assert_eq!(
        c2proof::parse_github_slug("https://github.com/o/r/tree/main"),
        None
    );
    assert_eq!(c2proof::parse_github_slug("o"), None);
    // non-github host → no PR publish, pipeline continues
    assert!(matches!(
        c2proof::publish_pr(Path::new("."), "https://gitlab.com/o/r"),
        Ok(None)
    ));
}

// --- full happy path (fixture mode, real cargo check) ---

#[test]
fn full_pipeline_fixture_mode_green() {
    let _env = ENV_LOCK.lock().unwrap();
    let src = tempfile::tempdir().unwrap();
    write_flat_c(src.path());
    let fix = tempfile::tempdir().unwrap();
    write_crate(fix.path(), "tinyexpr-port");
    let _g = set_fixture_dir(fix.path());

    let code = c2proof::migrate(src.path().to_str().unwrap(), true);
    assert_eq!(code, 0, "fixture-mode pipeline must be green end-to-end");
}

// --- invariants that already bit us once ---

#[test]
fn runner_image_tag_is_lowercase() {
    // GHCR rejects uppercase repo names — this exact bug failed CI once. Never again.
    assert_eq!(
        c2proof::RUNNER_IMAGE.to_lowercase(),
        c2proof::RUNNER_IMAGE,
        "GHCR image reference must be lowercase"
    );
    assert!(c2proof::RUNNER_IMAGE.starts_with("ghcr.io/"));
    assert!(
        c2proof::RUNNER_IMAGE.contains(":c2rust-"),
        "image tag must pin c2rust version"
    );
}

// --- transpile arg construction (c2rust 0.20 rejects dir args) ---

#[test]
fn c2rust_args_list_c_files_explicitly() {
    let d = tempfile::tempdir().unwrap();
    fs::write(d.path().join("b.c"), "").unwrap();
    fs::write(d.path().join("a.c"), "").unwrap(); // sorted before b.c
    fs::write(d.path().join("h.h"), "").unwrap(); // headers not entry points
    fs::create_dir(d.path().join("sub")).unwrap();
    fs::write(d.path().join("sub/nested.c"), "").unwrap(); // flat scan only
    let files = c2proof::list_c_files(d.path()).unwrap();
    assert_eq!(files, vec!["a.c", "b.c"]);
}
