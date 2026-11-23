//! Generate `skills/openacte/SKILL.md` from `registry/*.toml`.
//!
//! The registry is the single source: nothing here holds a repository fact.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod entry;
mod error;
mod render;

use std::fs;
use std::path::{Path, PathBuf};

pub use entry::{Entry, load_entries};
pub use error::Error;
pub use render::render;

/// Where the skill lives, under the repository root.
#[must_use]
pub fn skill_path(root: &Path) -> PathBuf {
    root.join("skills").join("openacte").join("SKILL.md")
}

fn rendered(root: &Path) -> Result<String, Error> {
    Ok(render(&load_entries(&root.join("registry"))?))
}

/// Write the skill from the registry.
///
/// # Errors
/// The registry cannot be read or is invalid, or the file cannot be written.
pub fn generate(root: &Path) -> Result<(), Error> {
    let text = rendered(root)?;
    let path = skill_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::Io {
            path: parent.into(),
            source,
        })?;
    }
    fs::write(&path, text).map_err(|source| Error::Io { path, source })
}

/// Fail when the committed skill differs from what the registry produces.
///
/// # Errors
/// The registry is invalid, the skill is missing, or the skill is stale.
pub fn check(root: &Path) -> Result<(), Error> {
    let want = rendered(root)?;
    let path = skill_path(root);
    let have = fs::read_to_string(&path).map_err(|source| Error::Io {
        path: path.clone(),
        source,
    })?;
    if have == want {
        Ok(())
    } else {
        Err(Error::Stale(path))
    }
}
