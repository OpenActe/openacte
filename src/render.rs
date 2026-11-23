use crate::entry::Entry;

const HEADER: &str = include_str!("skill_header.md");

fn render_entry(entry: &Entry) -> String {
    let remote = entry
        .repository
        .as_deref()
        .unwrap_or("no remote registered");
    let mut lines = vec![
        format!("### {} ({})", entry.name, entry.kind),
        String::new(),
        entry.role.clone(),
        String::new(),
        format!("- repository: {remote}"),
    ];
    if let Some(binary) = &entry.binary {
        lines.push(format!("- binary: `{binary}`"));
    }
    lines.extend(
        entry
            .usage
            .iter()
            .map(|usage| format!("- usage: `{usage}`")),
    );
    lines.push(format!("- directory: `{}`", entry.directory));
    lines.join("\n")
}

/// The whole skill file for these entries.
#[must_use]
pub fn render(entries: &[Entry]) -> String {
    let body = entries
        .iter()
        .map(render_entry)
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("{HEADER}\n{body}\n")
}
