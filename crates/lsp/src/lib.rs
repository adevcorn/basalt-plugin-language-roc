#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocLspConfig {
    pub server_command: String,
    pub args: Vec<String>,
    pub supports_in_memory_vfs: bool,
}

impl Default for RocLspConfig {
    fn default() -> Self {
        Self {
            server_command: "roc_language_server".to_string(),
            args: Vec::new(),
            supports_in_memory_vfs: true,
        }
    }
}
