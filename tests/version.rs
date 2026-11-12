//! `--version` / `-V` print the package version and exit 0 without starting the server.

use std::process::{Command, Stdio};

fn run(flag: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_ghp-lsp"))
        .arg(flag)
        .stdin(Stdio::null())
        .output()
        .expect("spawn ghp-lsp");
    assert!(out.status.success(), "{flag} must exit 0");
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn version_flags_print_the_package_version() {
    let want = format!("ghp-lsp {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(run("--version"), want);
    assert_eq!(run("-V"), want);
}
