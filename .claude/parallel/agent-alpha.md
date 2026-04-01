# Agent Alpha: Conservative Approach -- Extend Existing Patterns

## Codebase Analysis Summary

After reading every relevant file, here is what already exists:

1. **Annotations are already a first-class concept.** `Method.annotations: Vec<String>` and `Property.annotations: Vec<String>` in `types.rs` already hold strings like `@phi`, `@requires-auth`, `@audit-log`, `@encrypted`. The compliance validator (`crucible-compliance/src/validator.rs`) already reads and validates these annotations against framework rules.

2. **The parser is regex-based and line-oriented.** `RustParser` in `rust_parser.rs` walks lines looking for `pub struct`, `pub enum`, `pub fn`, `pub type`, and `use crate::` prefixes. It operates on `&str` lines with `strip_prefix` and `split_whitespace`. No AST, no multi-line parsing, no comment parsing.

3. **The sync system already compares discovered code to architecture definitions.** `SyncManager` in `sync.rs` takes `DiscoveredModule` (name, file_path, exports, imports) and diffs it against `Module` from JSON definitions. It generates `SyncReport` with new_modules/new_exports/new_dependencies.

4. **The validator already produces rich issues.** `ValidationIssue` has rule, severity, message, location, found, expected, suggestion, doc_link. The pattern is: check function returns `Option<Vec<ValidationIssue>>`, caller dispatches by severity.

5. **`DiscoveredModule` currently has no annotation field.** It only has name, file_path, exports (Vec<String>), imports (Vec<String>). This is the critical gap.

---

## Proposed Approaches

### Approach 1: Extend RustParser to Extract Comment-Based Annotations

**Philosophy:** The simplest decorator is a structured comment next to existing code -- parsed the exact same way the parser already parses `pub struct` lines.

**How it works:** Add annotation extraction to `RustParser::extract_exports` by looking at comment lines immediately preceding or on the same line as `pub` items. The annotation format is `// @crucible:annotation-name` or `/// @crucible:layer(core)` -- reusing the existing `@`-prefix convention from `types.rs`. Extend `DiscoveredModule` to carry a `HashMap<String, Vec<String>>` mapping export names to their discovered annotations. Extend `SyncManager` to compare discovered annotations against `Method.annotations` and `Property.annotations` in architecture definitions. Add a new validation rule `check_annotation_mismatch` to `Validator` using the exact same `Option<Vec<ValidationIssue>>` pattern as the existing 7 rules.

**Maintenance cost:** Low -- 3 files change (rust_parser.rs, sync.rs, validator.rs). All follow existing patterns. No new modules, no new types beyond extending `DiscoveredModule`.

**10x scale path:** Add -- Add a `TypeScriptParser` with the same interface as `RustParser` (returns `Vec<DiscoveredModule>`). Add more annotation types to the `@crucible:` prefix namespace. The `DiscoveredModule` struct and `SyncManager` comparison logic stay identical.

**Fastest validation:** Add annotation extraction to `RustParser::extract_exports` and a unit test showing `// @crucible:phi` on a `pub struct` field produces the right `DiscoveredModule.annotations` entry. Estimated: ~2 hours for parser + test, ~2 hours for sync comparison, ~2 hours for validator rule. Total: ~6 hours.

**Biggest risk:** Comment-based annotations are easy to accidentally delete or misformat. No compile-time enforcement. But this is the same risk the existing `@phi` annotations in JSON already have, so it is not a new failure mode.

**Tradeoffs:** Gain: zero new dependencies, zero new modules, minimal diff, works immediately with existing sync/validate pipeline. Give up: no IDE support (no red squiggles), no compile-time enforcement, comments can drift from code silently (but that is exactly what the validator catches).

**Code sketch:**

```rust
// In rust_parser.rs -- extend DiscoveredModule
pub struct DiscoveredModule {
    pub name: String,
    pub file_path: String,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
    pub annotations: HashMap<String, Vec<String>>,  // NEW: export_name -> annotations
}

// In rust_parser.rs -- extend extract_exports to also return annotations
// Pattern: look for "// @crucible:" comments on preceding line or same line
fn extract_exports_with_annotations(content: &str) -> (Vec<String>, HashMap<String, Vec<String>>) {
    let mut exports = Vec::new();
    let mut annotations: HashMap<String, Vec<String>> = HashMap::new();
    let mut pending_annotations: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Collect @crucible annotations from comments
        if let Some(rest) = trimmed.strip_prefix("// @crucible:") {
            pending_annotations.push(format!("@{}", rest.trim()));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("/// @crucible:") {
            pending_annotations.push(format!("@{}", rest.trim()));
            continue;
        }

        // Check for pub items (existing logic)
        let export_name = if let Some(rest) = trimmed.strip_prefix("pub struct ") {
            rest.split_whitespace().next().map(|n| n.trim_end_matches('<').to_string())
        } else if let Some(rest) = trimmed.strip_prefix("pub fn ") {
            rest.split('(').next().map(|n| n.trim().to_string())
        } else if let Some(rest) = trimmed.strip_prefix("pub enum ") {
            rest.split_whitespace().next().map(|n| n.trim_end_matches('<').to_string())
        } else {
            None
        };

        if let Some(name) = export_name {
            if !pending_annotations.is_empty() {
                annotations.insert(name.clone(), pending_annotations.clone());
                pending_annotations.clear();
            }
            exports.push(name);
        } else if !trimmed.is_empty() && !trimmed.starts_with("//") {
            // Non-comment, non-export line -- clear pending annotations
            pending_annotations.clear();
        }
    }

    (exports, annotations)
}

// In sync.rs -- extend SyncReport
pub struct SyncReport {
    // ... existing fields ...
    pub annotation_mismatches: HashMap<String, Vec<AnnotationMismatch>>,  // NEW
}

pub struct AnnotationMismatch {
    pub export_name: String,
    pub in_code: Vec<String>,       // annotations found in source
    pub in_architecture: Vec<String>, // annotations in .crucible JSON
}

// In validator.rs -- new validation rule, same pattern as existing 7
fn check_code_annotations(
    &self,
    discovered: &[DiscoveredModule],
) -> Option<Vec<ValidationIssue>> {
    let mut issues = Vec::new();
    // For each discovered module, find matching architecture module
    // Compare annotations on each export
    // Emit ValidationIssue for mismatches using existing severity/suggestion pattern
    if issues.is_empty() { None } else { Some(issues) }
}
```

**Source code comment format (Rust):**
```rust
// @crucible:layer(core)
// @crucible:phi
// @crucible:requires-auth
pub struct PatientRecord {
    // @crucible:encrypted
    pub ssn: String,
}
```

**Source code comment format (TypeScript):**
```typescript
// @crucible:layer(application)
// @crucible:requires-auth
// @crucible:audit-log
export class PatientService {
    // @crucible:phi
    // @crucible:encrypted
    ssn: string;
}
```

---

### Approach 2: Annotation-Only Validation (No Parser Changes)

**Philosophy:** Skip parsing source code for annotations entirely. Instead, add a new validation rule that checks architecture JSON annotations are internally consistent -- ensuring that if a module declares `@phi` on a property, it also declares `@encrypted` and `@requires-auth` on the methods that access it. This is what the compliance validator already does, just promoted to the core validator.

**How it works:** Add a `check_annotation_consistency` rule to `Validator` that validates annotation relationships within the JSON definitions. No source code parsing changes at all. The "decorator" is the JSON annotation itself -- developers declare annotations in `.crucible/modules/*.json` and the validator ensures they are consistent with each other and with the architectural rules. This is a direct extension of what `crucible-compliance` already does, just without requiring a separate compliance framework definition.

**Maintenance cost:** Low -- 1 file changes (validator.rs). Zero new modules, zero new structs.

**10x scale path:** Add -- Add more annotation consistency rules as custom rules in `rules.json`. The existing `CustomRule` type already supports target patterns and severity.

**Fastest validation:** Add `check_annotation_consistency` to `Validator::validate()` after the existing 7 checks. Test it produces `ValidationIssue` for a module with `@phi` property but no `@requires-auth` method annotation. Estimated: ~3 hours total.

**Biggest risk:** This does NOT bridge the code-to-architecture gap at all. It only validates architecture JSON against itself. Developers still manually maintain annotations in JSON. But it delivers immediate value with near-zero risk.

**Tradeoffs:** Gain: absolute minimum diff (one function added to validator.rs), zero new concepts, impossible to break anything. Give up: does not solve the core problem of verifying source code matches architecture. It is a stepping stone, not a solution.

**Code sketch:**

```rust
// In validator.rs -- add to validate() after check_event_trait_structure
fn check_annotation_consistency(&self) -> Option<Vec<ValidationIssue>> {
    let mut issues = Vec::new();

    for module in &self.project.modules {
        for (export_name, export) in &module.exports {
            let property_annotations: Vec<&String> = export.properties
                .iter()
                .flat_map(|props| props.values())
                .flat_map(|p| &p.annotations)
                .collect();

            let has_phi = property_annotations.iter().any(|a| *a == "@phi");

            if has_phi {
                // Check methods that access this export have @requires-auth
                if let Some(methods) = &export.methods {
                    for (method_name, method) in methods {
                        if !method.annotations.contains(&"@requires-auth".to_string()) {
                            issues.push(ValidationIssue::new(
                                "annotation-consistency".to_string(),
                                Severity::Warning,
                                format!(
                                    "Method '{}' on export with @phi properties should have @requires-auth",
                                    method_name
                                ),
                                Some(format!("{}.{}.{}", module.module, export_name, method_name)),
                            ));
                        }
                    }
                }
            }
        }
    }

    if issues.is_empty() { None } else { Some(issues) }
}
```

---

### Approach 3: Extend DiscoveredModule for Annotation Diffing (Layered)

**Philosophy:** Combine Approach 1 and 2 as incremental steps. Ship Approach 2 first (pure JSON validation, 1 file change, 3 hours). Ship Approach 1 second (parser + sync, 3 file changes, 6 hours). Each step is independently valuable and independently testable.

**How it works:** Phase 1 adds `check_annotation_consistency` to `validator.rs` -- validates that JSON annotations are internally coherent. Phase 2 adds annotation extraction to `RustParser` and annotation mismatch detection to `SyncManager`. Phase 3 (future) adds `TypeScriptParser` with the same `DiscoveredModule` interface. Each phase is a small PR that passes existing tests and adds new ones.

**Maintenance cost:** Low -- Phase 1: 1 file. Phase 2: 3 files. No new modules created at any phase.

**10x scale path:** Add -- Each phase adds capability without rewriting anything from earlier phases. The `DiscoveredModule` struct is the stable interface between parsers and sync.

**Fastest validation:** Phase 1 can be validated in ~3 hours. Phase 2 can be validated in ~6 hours. Total to full feature: ~9 hours across 2 PRs.

**Biggest risk:** The phased approach could stall after Phase 1, leaving the code-to-architecture gap unsolved. But Phase 1 still delivers independent value, so this is acceptable.

**Tradeoffs:** Gain: smallest possible first PR, incremental de-risking, each PR is independently reviewable and revertible. Give up: slightly more total effort than doing everything at once (9 hours vs. 6 hours), but much lower risk per change.

**Code sketch:** Same as Approaches 1 and 2 combined. Phase 1 = Approach 2 code. Phase 2 = Approach 1 code.

---

## My Recommendation

**Approach 3 (Layered)** is the conservative choice. Ship the annotation consistency check first (zero risk, immediate value, 1 file change). Then ship the comment-based annotation parser second (low risk, bridges the code-to-architecture gap, 3 file changes). Both use only patterns already in this codebase:

- `Option<Vec<ValidationIssue>>` return pattern from validator.rs
- `Vec<String>` annotation storage from types.rs
- Line-by-line `strip_prefix` parsing from rust_parser.rs
- `HashMap<String, Vec<String>>` diff comparison from sync.rs
- `@prefix` annotation naming from compliance framework

No new dependencies. No new modules. No new abstractions. The most boring possible implementation of an actually useful feature.
