use basalt_roc_parser::{parse_outline, ModuleKind, RocSymbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocSemanticFacts {
    pub module_kind: ModuleKind,
    pub pure_functions: Vec<RocSymbol>,
    pub task_functions: Vec<RocSymbol>,
}

pub fn extract_facts(source: &str) -> RocSemanticFacts {
    let outline = parse_outline(source);
    let mut pure_functions = Vec::new();
    let mut task_functions = Vec::new();

    for sym in outline.symbols {
        if sym.is_task {
            task_functions.push(sym);
        } else {
            pure_functions.push(sym);
        }
    }

    RocSemanticFacts {
        module_kind: outline.kind,
        pure_functions,
        task_functions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_pure_and_task_functions() {
        let src = "interface Foo exposes [a, b]\na : I64 -> I64\na = \\x -> x\nb : Task {} []\nb = Task.ok {}\n";
        let facts = extract_facts(src);
        assert_eq!(facts.module_kind, ModuleKind::Interface);
        assert_eq!(facts.pure_functions.len(), 1);
        assert_eq!(facts.task_functions.len(), 1);
    }
}
