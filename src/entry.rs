use crate::error::Error;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

const KIND_ORDER: [&str; 7] = [
    "cli",
    "service",
    "app",
    "library",
    "scaffold",
    "workspace",
    "unverified",
];

/// One registry file. Unknown keys are refused so a new field is noticed.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub name: String,
    pub directory: String,
    pub kind: String,
    pub role: String,
    pub repository: Option<String>,
    pub binary: Option<String>,
    pub readme: Option<String>,
    #[serde(default)]
    pub usage: Vec<String>,
}

fn load_entry(path: &Path) -> Result<Entry, Error> {
    let text = fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.into(),
        source,
    })?;
    let mut entry: Entry = toml::from_str(&text).map_err(|source| Error::Parse {
        path: path.into(),
        source,
    })?;
    let fields = [
        ("name", &entry.name),
        ("directory", &entry.directory),
        ("kind", &entry.kind),
        ("role", &entry.role),
    ];
    for (field, value) in fields {
        if value.trim().is_empty() {
            return Err(Error::EmptyField {
                path: path.into(),
                field,
            });
        }
    }
    entry.role = entry.role.split_whitespace().collect::<Vec<_>>().join(" ");
    Ok(entry)
}

fn registry_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let io = |source| Error::Io {
        path: dir.into(),
        source,
    };
    let mut paths = Vec::new();
    for item in fs::read_dir(dir).map_err(io)? {
        let path = item.map_err(io)?.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn kind_rank(kind: &str) -> usize {
    KIND_ORDER
        .iter()
        .position(|k| *k == kind)
        .unwrap_or(KIND_ORDER.len())
}

/// Every registry entry, ordered by kind then name.
///
/// # Errors
/// The directory or a file cannot be read, a file is invalid, or none exists.
pub fn load_entries(dir: &Path) -> Result<Vec<Entry>, Error> {
    let paths = registry_files(dir)?;
    if paths.is_empty() {
        return Err(Error::NoEntries(dir.into()));
    }
    let mut entries = paths
        .iter()
        .map(|p| load_entry(p))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by(|a, b| (kind_rank(&a.kind), &a.name).cmp(&(kind_rank(&b.kind), &b.name)));
    Ok(entries)
}
