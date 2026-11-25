use openacte_skill::{Error, check, generate, load_entries, render, skill_path};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const CLI: &str = "name = \"a\"\ndirectory = \"a\"\nkind = \"cli\"\nrole = \"does  a\\n thing\"\nbinary = \"a\"\nrepository = \"https://x/a\"\nusage = [\"a run\"]\n";
const LIB: &str = "name = \"b\"\ndirectory = \"b\"\nkind = \"library\"\nrole = \"lib\"\n";

fn repo(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().unwrap();
    let registry = dir.path().join("registry");
    fs::create_dir_all(&registry).unwrap();
    for (name, text) in files {
        fs::write(registry.join(name), text).unwrap();
    }
    dir
}

fn load(dir: &TempDir) -> Result<Vec<openacte_skill::Entry>, Error> {
    load_entries(&dir.path().join("registry"))
}

#[test]
fn fields_are_read_and_role_whitespace_collapses() {
    let entries = load(&repo(&[("a.toml", CLI)])).unwrap();
    assert_eq!(entries[0].role, "does a thing");
    assert_eq!(entries[0].usage, ["a run"]);
    assert_eq!(entries[0].binary.as_deref(), Some("a"));
}

#[test]
fn missing_required_key_names_the_file() {
    let bad = "name = \"x\"\ndirectory = \"x\"\nkind = \"cli\"\n";
    let message = load(&repo(&[("bad.toml", bad)])).unwrap_err().to_string();
    assert!(
        message.contains("bad.toml") && message.contains("role"),
        "{message}"
    );
}

#[test]
fn unknown_key_is_refused() {
    let text = format!("{LIB}surprise = 1\n");
    let message = load(&repo(&[("u.toml", &text)])).unwrap_err().to_string();
    assert!(
        message.contains("u.toml") && message.contains("surprise"),
        "{message}"
    );
}

#[test]
fn empty_required_value_is_refused() {
    let text = LIB.replace("role = \"lib\"", "role = \"  \"");
    let error = load(&repo(&[("e.toml", &text)])).unwrap_err();
    assert!(matches!(error, Error::EmptyField { field: "role", .. }));
}

#[test]
fn empty_registry_is_an_error() {
    assert!(matches!(load(&repo(&[])), Err(Error::NoEntries(_))));
}

#[test]
fn missing_registry_directory_is_an_io_error() {
    let dir = TempDir::new().unwrap();
    assert!(matches!(load(&dir), Err(Error::Io { .. })));
}

#[test]
fn invalid_toml_is_a_parse_error() {
    let error = load(&repo(&[("x.toml", "not = = toml")])).unwrap_err();
    assert!(matches!(error, Error::Parse { .. }));
}

#[test]
fn files_that_are_not_toml_are_ignored() {
    let entries = load(&repo(&[("a.toml", CLI), ("notes.txt", "garbage")])).unwrap();
    assert_eq!(entries.len(), 1);
}

#[test]
fn order_is_kind_then_name() {
    let entries = load(&repo(&[("b.toml", LIB), ("a.toml", CLI)])).unwrap();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["a", "b"]);
}

#[test]
fn unknown_kind_sorts_last() {
    let odd = CLI.replace("kind = \"cli\"", "kind = \"zzz\"");
    let entries = load(&repo(&[("a.toml", &odd), ("b.toml", LIB)])).unwrap();
    assert_eq!(entries[0].name, "b");
}

#[test]
fn missing_repository_is_said_not_hidden() {
    let text = render(&load(&repo(&[("b.toml", LIB)])).unwrap());
    assert!(text.contains("- repository: no remote registered"));
}

#[test]
fn cli_entry_shows_binary_usage_and_directory() {
    let text = render(&load(&repo(&[("a.toml", CLI)])).unwrap());
    assert!(text.starts_with("---\nname: openacte\n"));
    assert!(text.contains("- binary: `a`") && text.contains("- usage: `a run`"));
    assert!(text.contains("- directory: `a`") && !text.contains("/Users/"));
}

#[test]
fn generate_writes_a_skill_that_check_accepts() {
    let dir = repo(&[("a.toml", CLI)]);
    generate(dir.path()).unwrap();
    check(dir.path()).unwrap();
}

#[test]
fn check_fails_on_a_stale_skill() {
    let dir = repo(&[("a.toml", CLI)]);
    generate(dir.path()).unwrap();
    fs::write(skill_path(dir.path()), "old").unwrap();
    assert!(matches!(check(dir.path()), Err(Error::Stale(_))));
}

#[test]
fn check_fails_when_the_skill_is_missing() {
    let dir = repo(&[("a.toml", CLI)]);
    assert!(matches!(check(dir.path()), Err(Error::Io { .. })));
}

#[test]
fn generate_reports_an_unwritable_target() {
    let dir = repo(&[("a.toml", CLI)]);
    fs::write(dir.path().join("skills"), "a file, not a directory").unwrap();
    assert!(matches!(generate(dir.path()), Err(Error::Io { .. })));
}

#[test]
fn committed_skill_matches_the_registry() {
    check(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
}

#[test]
fn page_tells_to_grow_a_repository_or_create_one() {
    let entries = load(&repo(&[("a.toml", CLI)])).unwrap();
    let page = render(&entries);
    assert!(
        page.contains("## When no repository does the job"),
        "{page}"
    );
    assert!(page.contains("add it to that repository"), "{page}");
    assert!(page.contains("create a new repository"), "{page}");
}
