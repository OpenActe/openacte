use std::fmt;
use std::path::PathBuf;

/// Every way generation can fail; each names the file concerned.
#[derive(Debug)]
pub enum Error {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    EmptyField {
        path: PathBuf,
        field: &'static str,
    },
    NoEntries(PathBuf),
    Stale(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Parse { path, source } => write!(f, "{}: {source}", path.display()),
            Self::EmptyField { path, field } => {
                write!(f, "{}: field `{field}` is empty", path.display())
            }
            Self::NoEntries(dir) => write!(f, "no registry entry found in {}", dir.display()),
            Self::Stale(path) => write!(
                f,
                "{} is stale: run `cargo run` to regenerate it",
                path.display()
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}
