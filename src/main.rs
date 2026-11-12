//! `ghp-lsp`: stdio language server for GHP (`.ghp`) templates.

use std::io::BufReader;

fn main() {
    // `--version` / `-V` print the version and exit before the server touches stdin.
    if std::env::args().skip(1).any(|a| a == "--version" || a == "-V") {
        println!("ghp-lsp {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let input = BufReader::new(std::io::stdin());
    let code = ghp_lsp::server::run(input, std::io::stdout().lock());
    std::process::exit(code);
}
