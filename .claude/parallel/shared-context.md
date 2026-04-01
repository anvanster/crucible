# Parallel Exploration: Code Decorators for Architecture Verification
**Created:** 2026-02-21
**Status:** EXPLORING

## Problem Statement
Crucible currently validates architecture definitions (JSON files in `.crucible/`) against each other — checking circular deps, layer boundaries, type existence, etc. But the critical gap is verifying that **actual source code** matches the architecture definitions. The existing `rust_parser.rs` does basic regex-based extraction of `pub struct/fn/enum/type` and `use crate::` imports, and `sync.rs` compares discovered code against architecture. But there's no mechanism for developers to **annotate their code** with architecture metadata that Crucible can verify. The idea is to implement "code decorators" — annotations in source code (comments, attributes, macros, or doc tags) that declare architectural intent, which Crucible can then parse and validate against the `.crucible/` definitions. This would bridge the gap between "architecture says X" and "code actually does X."

## Relevant Code

### Core Types (crucible-core/src/types.rs)
- `Module` — has `module`, `version`, `layer`, `exports`, `dependencies`
- `Export` — has `export_type` (Class/Function/Interface/Type/Enum/Event/Trait), `methods`, `properties`, `annotations`
- `Method` — has `inputs`, `returns`, `throws`, `calls`, `effects`, `is_async`, `annotations`
- `Property` — has `prop_type`, `required`, `annotations`
- Annotations already exist as `Vec<String>` on Method and Property (e.g., `@phi`, `@requires-auth`, `@audit-log`)

### Existing Code Parsing (crucible-core/src/claude/rust_parser.rs)
- `RustParser` — regex-based, extracts `pub struct/enum/fn/type` and `use crate::` imports
- `DiscoveredModule` — has `name`, `file_path`, `exports`, `imports`
- No support for parsing annotations/decorators from code

### Sync System (crucible-core/src/claude/sync.rs)
- `SyncManager` — compares discovered code modules against architecture definitions
- `SyncReport` — tracks new_modules, new_exports, new_dependencies
- Can generate module definitions from discovered code
- Currently Rust-only

### Validator (crucible-core/src/validator.rs)
- 7 validation rules: circular deps, layer boundaries, type existence, call targets, used deps, declared deps, event/trait structure
- `ValidationIssue` with rich suggestions (found/expected, Levenshtein fuzzy matching, doc links)
- `ChangeTracker` for incremental validation

### Compliance (crucible-compliance/src/validator.rs)
- Already uses annotations (`@phi`, `@encrypted`, `@requires-auth`) for compliance validation
- Validates annotations against compliance framework rules
- Pattern: collect_property_annotations → validate against rules

### Discovery (crucible-core/src/claude/discovery.rs)
- `ArchitectureDiscovery` — mostly TODO/stub, not implemented

### Key Dependencies
- `serde`/`serde_json` for JSON
- `petgraph` for dependency graphs
- `walkdir`/`ignore` for file traversal
- `handlebars` for templates
- No tree-sitter or AST parsing — everything is regex-based

## Evaluation Criteria
1. Maintenance cost — How many files must someone read to understand this? (target: ≤3)
2. Scale path at 10x — Is the upgrade path "add things" or "rewrite things"?
3. Validation speed — How quickly can we prove the core idea works?
4. Integration effort — How much existing code needs to change?
5. Risk level — What's the probability of a fundamental flaw?

## Hard Constraints
- Must integrate with existing project architecture
- Must not break existing tests
- Must be implementable incrementally (no big-bang rewrites)
- Must work across at least Rust and TypeScript (the two languages Crucible focuses on)
- Must use existing annotation system (`Vec<String>` on Method/Property) as the validation target

## Soft Constraints
- Prefer solutions that reduce overall complexity
- Prefer solutions that make future changes easier
- Prefer language-agnostic decorator syntax where possible
- Should work with the existing regex-based parser approach (no mandatory tree-sitter dependency)

## Out of Scope
- Full AST parsing / tree-sitter integration (can be mentioned as future enhancement)
- Runtime enforcement (this is static/build-time verification only)
- IDE integration (LSP, VS Code extension)
- Automatic code generation from decorators
