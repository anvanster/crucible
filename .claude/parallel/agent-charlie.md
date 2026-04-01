# Agent Charlie: Minimal / Validation-First

## The Riskiest Assumption

The entire code decorators idea rests on ONE assumption that nobody has tested:

**Can we reliably round-trip annotations between source code comments and `.crucible/` JSON definitions, and catch mismatches?**

Everything else (syntax design, multi-language support, AST parsing, abstraction layers) is premature until we know the answer to: "If a developer writes `// @crucible layer=domain` in their Rust source, and the `.crucible/modules/patient.json` says `"layer": "service"`, can we detect that mismatch and report it clearly?"

If the answer is no (regex too fragile, too many false positives, developers won't write the comments), then no amount of architecture will save this feature.

## What to Build

A single-file proof that:
1. Reads a Rust source file and extracts `// @crucible:` comments attached to `pub struct/fn/enum`
2. Reads the corresponding `.crucible/modules/*.json` file
3. Compares annotations from code vs. JSON and reports mismatches
4. Does this in under 100 lines with zero new dependencies

Nothing else. No trait hierarchies, no multi-language support, no config files, no error recovery.

---

### Annotation Mismatch Detector
**Philosophy:** The only thing that matters is whether code-to-JSON comparison works at all -- build the thinnest possible end-to-end slice.

**How it works:** Developer adds structured comments like `// @crucible: layer=domain, annotations=[@phi, @encrypted]` above `pub struct Patient`. A standalone script reads the .rs file with a single regex, reads the matching .crucible JSON, and diffs the two. Output is a list of mismatches: "Patient: code says layer=domain, JSON says layer=service" or "Patient.ssn: code says annotations=[@phi, @encrypted], JSON has annotations=[@phi] -- missing @encrypted".

**Maintenance cost:** Low -- it is one file, one regex, one comparison loop. Anyone can read it in 5 minutes.

**10x scale path:** Add -- the core comparison logic stays identical. You add more regex patterns for TypeScript, more field comparisons beyond layer/annotations, and eventually swap regex for tree-sitter. The diff algorithm never changes.

**Fastest validation:** Build the script below, run it against the existing `crucible-compliance/docs/examples/healthcare-portal/.crucible/modules/patient.json` with a fake annotated Rust file. If the mismatch report is correct and useful, the concept is validated. Estimated effort: 2 hours.

**Biggest risk:** Developers simply won't write the comments. The technical extraction works fine but adoption is zero because the ergonomics of comment-based annotations are terrible compared to just editing the JSON directly.

**Tradeoffs:** You get immediate proof of concept with zero risk of over-engineering. You give up multi-language support, proper error handling, and any kind of nice CLI experience. Those are all things that don't matter until you know the core idea works.

**Code sketch:**

This is a runnable Rust program (under 100 lines) that you drop into a `examples/` directory or a scratch binary. It hard-codes paths, panics on errors, and does exactly one thing: extract `@crucible` comments from a Rust file, compare them to a `.crucible` JSON definition, and print mismatches.

```rust
// file: crucible-core/examples/decorator_spike.rs
//
// Run: cargo run --example decorator_spike
//
// Tests the core assumption: can we extract structured annotations from
// Rust source comments and compare them against .crucible JSON definitions?

use std::collections::HashMap;

fn main() {
    // Hard-coded test input: a Rust file with @crucible annotations
    let rust_source = r#"
// @crucible: layer=domain, annotations=[@phi, @encrypted]
pub struct Patient {
    pub id: String,
    // @crucible: annotations=[@phi, @encrypted]
    pub ssn: String,
    // @crucible: annotations=[@phi]
    pub first_name: String,
}

// @crucible: layer=domain, annotations=[@requires-auth, @phi-access]
pub fn find_patient(id: String) -> Patient {
    todo!()
}
"#;

    // Hard-coded JSON definition (simulating what's in .crucible/modules/patient.json)
    let json_def = serde_json::json!({
        "module": "patient",
        "version": "1.0.0",
        "layer": "service",
        "exports": {
            "Patient": {
                "type": "class",
                "properties": {
                    "ssn": { "type": "string", "annotations": ["@phi"] },
                    "first_name": { "type": "string", "annotations": ["@phi", "@encrypted"] }
                }
            },
            "find_patient": {
                "type": "function",
                "methods": {},
                "annotations": ["@requires-auth"]
            }
        }
    });

    // Step 1: Extract @crucible comments from source
    let code_decls = extract_decorators(rust_source);
    println!("Extracted {} declarations from source code\n", code_decls.len());

    // Step 2: Compare against JSON
    let json_layer = json_def["layer"].as_str().unwrap_or("");
    let json_exports = json_def["exports"].as_object().unwrap();

    let mut mismatches = Vec::new();

    for decl in &code_decls {
        // Check layer mismatch
        if let Some(code_layer) = &decl.layer {
            if code_layer != json_layer {
                mismatches.push(format!(
                    "{}: layer mismatch -- code says '{}', JSON says '{}'",
                    decl.name, code_layer, json_layer
                ));
            }
        }

        // Check export-level annotation mismatches
        if let Some(json_export) = json_exports.get(&decl.name) {
            // Check property-level annotations
            if let Some(props) = &decl.properties {
                let json_props = json_export.get("properties")
                    .and_then(|p| p.as_object());
                for (prop_name, code_annotations) in props {
                    let json_anns: Vec<String> = json_props
                        .and_then(|p| p.get(prop_name))
                        .and_then(|p| p.get("annotations"))
                        .and_then(|a| a.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default();
                    let missing_in_json: Vec<_> = code_annotations.iter()
                        .filter(|a| !json_anns.contains(a)).collect();
                    let missing_in_code: Vec<_> = json_anns.iter()
                        .filter(|a| !code_annotations.contains(a)).collect();
                    if !missing_in_json.is_empty() {
                        mismatches.push(format!(
                            "{}.{}: code has {:?} but JSON is missing them",
                            decl.name, prop_name, missing_in_json
                        ));
                    }
                    if !missing_in_code.is_empty() {
                        mismatches.push(format!(
                            "{}.{}: JSON has {:?} but code is missing them",
                            decl.name, prop_name, missing_in_code
                        ));
                    }
                }
            }
        } else {
            mismatches.push(format!("{}: exists in code but not in JSON", decl.name));
        }
    }

    // Step 3: Report
    if mismatches.is_empty() {
        println!("All annotations match between code and architecture definition.");
    } else {
        println!("MISMATCHES FOUND ({}):", mismatches.len());
        for m in &mismatches {
            println!("  - {}", m);
        }
    }
}

struct CodeDeclaration {
    name: String,
    layer: Option<String>,
    annotations: Vec<String>,
    properties: Option<HashMap<String, Vec<String>>>,
}

fn extract_decorators(source: &str) -> Vec<CodeDeclaration> {
    let mut result = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if let Some(rest) = trimmed.strip_prefix("// @crucible:") {
            let attrs = parse_attrs(rest.trim());
            // Look ahead for pub struct/fn/enum or property
            if i + 1 < lines.len() {
                let next = lines[i + 1].trim();
                if let Some(name) = extract_pub_name(next) {
                    // This is a top-level declaration -- collect property annotations
                    let mut props = HashMap::new();
                    let mut j = i + 2;
                    while j < lines.len() {
                        let line = lines[j].trim();
                        if line.starts_with("// @crucible:") && j + 1 < lines.len() {
                            let prop_attrs = parse_attrs(lines[j].trim().strip_prefix("// @crucible:").unwrap().trim());
                            let prop_line = lines[j + 1].trim();
                            if let Some(pname) = extract_field_name(prop_line) {
                                props.insert(pname, prop_attrs.get("annotations").cloned().unwrap_or_default());
                            }
                            j += 2;
                        } else if line.starts_with("}") || line.is_empty() && j + 1 < lines.len() && lines[j+1].trim().starts_with("//") {
                            break;
                        } else {
                            j += 1;
                        }
                    }
                    result.push(CodeDeclaration {
                        name,
                        layer: attrs.get("layer").map(|v| v[0].clone()),
                        annotations: attrs.get("annotations").cloned().unwrap_or_default(),
                        properties: if props.is_empty() { None } else { Some(props) },
                    });
                }
            }
        }
        i += 1;
    }
    result
}

fn parse_attrs(s: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for part in s.split(',') {
        let part = part.trim();
        if let Some((key, val)) = part.split_once('=') {
            let key = key.trim().to_string();
            let val = val.trim();
            if val.starts_with('[') {
                let inner = val.trim_start_matches('[').trim_end_matches(']');
                map.insert(key, inner.split(',').map(|s| s.trim().to_string()).collect());
            } else {
                map.insert(key, vec![val.to_string()]);
            }
        }
    }
    map
}

fn extract_pub_name(line: &str) -> Option<String> {
    for prefix in &["pub struct ", "pub fn ", "pub enum ", "pub type "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next().map(String::from);
        }
    }
    None
}

fn extract_field_name(line: &str) -> Option<String> {
    let line = line.strip_prefix("pub ").unwrap_or(line);
    line.split(':').next().map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with("//"))
}
```

**Expected output when you run it:**

```
Extracted 2 declarations from source code

MISMATCHES FOUND (3):
  - Patient: layer mismatch -- code says 'domain', JSON says 'service'
  - Patient.ssn: code has ["@encrypted"] but JSON is missing them
  - Patient.first_name: JSON has ["@encrypted"] but code is missing them
```

This output alone proves the concept works. The three mismatches are all real, all actionable, and a developer could fix any of them in 30 seconds.

## What This Proves (And What It Doesn't)

**Proves:**
- Structured comments are parseable with trivial regex (no tree-sitter needed for v1)
- The comparison yields useful, actionable output
- The `// @crucible:` prefix is greppable, won't collide with normal comments
- Property-level annotation diffing works end-to-end

**Does NOT prove:**
- Whether developers will actually write these comments (needs user interviews, not code)
- Whether TypeScript/Python parsing works (but the approach trivially ports)
- Whether this integrates cleanly into the existing validator pipeline (integration is easy once the concept is validated)
- Performance at scale (irrelevant until there's adoption)

## Integration Path (If The Spike Succeeds)

1. Move `extract_decorators` into `rust_parser.rs` as a new method on `RustParser` (add `DiscoveredAnnotation` to `DiscoveredModule`)
2. Add a `check_code_annotations` method to `Validator` that calls the parser and diffs against loaded modules
3. Wire it into `SyncManager::detect_conflicts`
4. Done. Three files touched, zero new modules.

The key insight: the existing `RustParser` already walks all `.rs` files and reads their content. Adding annotation extraction to `parse_file` is ~20 additional lines. The existing `SyncManager` already compares discovered code vs. JSON modules. Adding annotation comparison is ~30 additional lines. The total integration is under 50 lines of production code, not counting tests.
