#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => openacte_skill::generate(root),
        ["--check"] => openacte_skill::check(root),
        _ => {
            eprintln!("usage: openacte-skill [--check]");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("openacte-skill: {error}");
            ExitCode::FAILURE
        }
    }
}
