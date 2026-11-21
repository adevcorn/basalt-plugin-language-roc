use std::collections::HashMap;
use basalt_roc_semantic_facts::extract_facts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyLane {
    InMemoryWasm32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyPlan {
    pub lane: VerifyLane,
    pub pure_targets: Vec<String>,
    pub mocked_task_targets: Vec<String>,
}

/// Read-only in-memory VFS interface backed by Basalt's `PendingStore` + Slab.
pub trait InMemoryVfs: Send + Sync {
    fn read_bytes(&self, path: &str) -> Option<Vec<u8>>;
    fn exists(&self, path: &str) -> bool {
        self.read_bytes(path).is_some()
    }
}

/// Simple in-memory VFS overlay (pending overrides committed) for embedded Roc compilation.
#[derive(Default)]
pub struct OverlayVfs {
    pub committed: HashMap<String, Vec<u8>>,
    pub pending: HashMap<String, Vec<u8>>,
}

impl InMemoryVfs for OverlayVfs {
    fn read_bytes(&self, path: &str) -> Option<Vec<u8>> {
        self.pending
            .get(path)
            .or_else(|| self.committed.get(path))
            .cloned()
    }
}

pub fn plan_verification(source: &str) -> VerifyPlan {
    let facts = extract_facts(source);
    VerifyPlan {
        lane: VerifyLane::InMemoryWasm32,
        pure_targets: facts.pure_functions.into_iter().map(|s| s.name).collect(),
        mocked_task_targets: facts.task_functions.into_iter().map(|s| s.name).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_vfs_prefers_pending_over_committed() {
        let mut vfs = OverlayVfs::default();
        vfs.committed.insert("main.roc".into(), b"old".to_vec());
        vfs.pending.insert("main.roc".into(), b"new".to_vec());
        assert_eq!(vfs.read_bytes("main.roc").as_deref(), Some(b"new".as_slice()));
    }
}
