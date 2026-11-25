//! `skills/baba/SKILL.md` installs Baba when it is missing and does nothing when
//! it is not (Baba ADR-047). Hand-written, so these tests pin what must not
//! drift: the check comes first, the steps keep their order, every repository
//! is cloned, and nothing destructive is ever told.

use std::fs;
use std::path::PathBuf;

fn skill() -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("skills/baba/SKILL.md");
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn at(text: &str, needle: &str) -> usize {
    text.find(needle)
        .unwrap_or_else(|| panic!("the skill never says: {needle}"))
}

#[test]
fn frontmatter_names_the_skill_and_says_when_to_use_it() {
    let s = skill();
    assert!(
        s.starts_with("---\nname: baba\ndescription: "),
        "{}",
        &s[..60.min(s.len())]
    );
    let head = &s[..at(&s[3..], "---") + 3];
    assert!(
        head.contains("install"),
        "the description must say it installs"
    );
}

#[test]
fn the_check_comes_first_and_can_stop_everything() {
    let s = skill();
    let check = at(&s, "baba doctor");
    assert!(check < at(&s, "git clone"), "check before any clone");
    assert!(
        s.contains("already installed"),
        "a healthy Baba stops the run"
    );
}

#[test]
fn the_steps_keep_their_order() {
    let s = skill();
    let order = [
        "swipl",
        "git clone git@github.com:grosjeanbaptiste/baba.git",
        "git clone git@github.com:grosjeanbaptiste/baba-engine.git",
        "cargo run",
        "cargo build --release",
    ];
    let pos: Vec<usize> = order.iter().map(|n| at(&s, n)).collect();
    assert!(pos.windows(2).all(|w| w[0] < w[1]), "{pos:?}");
    assert!(
        s.rfind("baba doctor").unwrap() > *pos.last().unwrap(),
        "doctor closes the run"
    );
}

#[test]
fn all_six_sub_repositories_are_cloned() {
    let s = skill();
    for n in ["engine", "cli", "actions", "corpus", "packs", "config"] {
        let url = format!("git@github.com:grosjeanbaptiste/baba-{n}.git");
        assert!(s.contains(&url), "missing {url}");
    }
}

#[test]
fn nothing_destructive_is_ever_told() {
    let s = skill();
    for bad in ["rm -rf", "--force", "git reset", "git clean"] {
        assert!(!s.contains(bad), "the skill must not say: {bad}");
    }
    assert!(s.contains("never overwrite"), "existing checkouts are kept");
}
