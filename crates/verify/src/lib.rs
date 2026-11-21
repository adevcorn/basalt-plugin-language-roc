use std::collections::HashMap;
use basalt_roc_semantic_facts::extract_facts;
use wasmtime::{Engine, Linker, Module, Store};

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

/// Mock-platform host state for executing compiled Roc `wasm32` modules in-memory.
#[derive(Default)]
pub struct MockPlatformState {
    pub logs: Vec<String>,
    pub rng_seed: u64,
}

/// Executes a compiled `wasm32` module's exported `fn(i64) -> i64` in-memory
/// with mocked Roc platform host imports.
pub fn run_wasm32_i64(wasm_bytes: &[u8], export_name: &str, input: i64) -> Result<i64, String> {
    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| e.to_string())?;
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap(
            "env",
            "roc_mock_random_i64",
            |mut caller: wasmtime::Caller<'_, MockPlatformState>| -> i64 {
                let state = caller.data_mut();
                state.rng_seed = state.rng_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                state.rng_seed as i64
            },
        )
        .map_err(|e| e.to_string())?;

    let mut store = Store::new(&engine, MockPlatformState::default());
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| e.to_string())?;
    let func = instance
        .get_typed_func::<i64, i64>(&mut store, export_name)
        .map_err(|e| e.to_string())?;
    func.call(&mut store, input).map_err(|e| e.to_string())
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
