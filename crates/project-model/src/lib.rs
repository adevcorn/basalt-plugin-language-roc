use basalt_roc_parser::{parse_outline, ModuleKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocProjectEntry {
    pub path: String,
    pub kind: ModuleKind,
}

pub fn discover_entries<'a>(files: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<RocProjectEntry> {
    files
        .into_iter()
        .filter(|(path, _)| path.ends_with(".roc"))
        .map(|(path, src)| {
            let outline = parse_outline(src);
            RocProjectEntry {
                path: path.to_string(),
                kind: outline.kind,
            }
        })
        .collect()
}
