# basalt-plugin-language-roc

Batteries-included **Roc** language pack for Basalt (`language-roc`).

## Included Capabilities

- **`crates/parser`** (`language@roc/v1`): Parses Roc module headers (`app`, `package`, `platform`, `interface`), top-level definitions, and type signatures.
- **`crates/semantic-facts`** (`semantic-facts@roc/v1`): Extracts module kind, exposed/imported symbols, type signatures, and `Task` effect boundaries.
- **`crates/project-model`** (`project-model@roc/v1`): Discovers `.roc` entrypoints and header-declared platform/package dependencies.
- **`crates/lsp`** (`lsp@roc/v1`): Bridges `roc_language_server` / in-memory diagnostics and hover types.
- **`crates/verify`** (`verify@roc/v1`): In-memory `wasm32` verification and mock-platform PBT harness for `fork_phantom`, `check`, and `test`.

## Shadow Mode

This pack advertises `supports_in_memory = true` and `default_shadow_mode = "minimal"`, allowing Basalt sessions to skip full source-tree shadow copies and run Roc edits, checks, and PBT directly against `PendingStore` + Slab.
