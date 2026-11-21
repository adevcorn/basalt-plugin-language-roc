#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleKind {
    App,
    Package,
    Platform,
    Interface,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocSymbol {
    pub name: String,
    pub line: usize,
    pub signature: Option<String>,
    pub is_task: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocOutline {
    pub kind: ModuleKind,
    pub symbols: Vec<RocSymbol>,
}

pub fn parse_outline(source: &str) -> RocOutline {
    let mut kind = ModuleKind::Unknown;
    let mut symbols = Vec::new();
    let mut pending_sig: Option<(String, String)> = None;

    for (idx, raw_line) in source.lines().enumerate() {
        let line_num = idx + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if kind == ModuleKind::Unknown {
            if line.starts_with("app ") || line == "app" {
                kind = ModuleKind::App;
            } else if line.starts_with("package ") || line == "package" {
                kind = ModuleKind::Package;
            } else if line.starts_with("platform ") || line == "platform" {
                kind = ModuleKind::Platform;
            } else if line.starts_with("interface ") || line == "interface" {
                kind = ModuleKind::Interface;
            }
        }

        if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
            if let Some((name, sig)) = line.split_once(" : ") {
                let name = name.trim();
                if is_ident(name) {
                    pending_sig = Some((name.to_string(), sig.trim().to_string()));
                    continue;
                }
            }

            if let Some((lhs, _rhs)) = line.split_once(" = ") {
                let name = lhs.trim();
                if is_ident(name) {
                    let sig = pending_sig
                        .take()
                        .and_then(|(n, s)| if n == name { Some(s) } else { None });
                    let is_task = sig.as_deref().is_some_and(|s| s.contains("Task "));
                    symbols.push(RocSymbol {
                        name: name.to_string(),
                        line: line_num,
                        signature: sig,
                        is_task,
                    });
                }
            }
        }
    }

    RocOutline { kind, symbols }
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_app_and_symbols() {
        let src = r#"
app [main] { pf: platform "https://example.com/platform.roc" }

add : I64, I64 -> I64
add = \a, b -> a + b

main : Task {} []
main = Stdout.line "hi"
"#;
        let outline = parse_outline(src);
        assert_eq!(outline.kind, ModuleKind::App);
        assert_eq!(outline.symbols.len(), 2);
        assert_eq!(outline.symbols[0].name, "add");
        assert!(!outline.symbols[0].is_task);
        assert_eq!(outline.symbols[1].name, "main");
        assert!(outline.symbols[1].is_task);
    }
}
