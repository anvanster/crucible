# Agent Bravo: Unconventional Approaches to Code Decorators

## Key Observation Before Proposals

The conventional approach everyone will propose is: "parse `@crucible` comments from source code, match them to `.crucible/` JSON definitions, report mismatches." I deliberately reject this as a starting point because it has a fundamental design flaw: **it creates two sources of truth that must be kept in sync, which is the exact problem Crucible was built to solve in the first place.**

Adding "code decorators" that must be manually synchronized with architecture JSON files is turtles all the way down. The architecture JSON already *is* the decorator system -- it annotates code with architectural intent. The real question is: how do we verify that code *conforms to* those declarations without requiring developers to manually parrot the architecture back into their source files?

---

### Approach 1: Structural Fingerprinting (from Distributed Systems / Content-Addressable Storage)

**Philosophy:** Instead of asking developers to annotate code, compute a structural fingerprint of code elements and compare it against the architecture definition -- like a content hash verifies data integrity without embedding metadata in the data itself.

**How it works:** Extend `RustParser` (and add a `TypeScriptParser`) to extract not just names but *structural signatures*: function arity, parameter types (from type annotations), return types, public method sets on structs/classes, and `use`/`import` edges. Compute a `StructuralFingerprint` for each discovered code element. The `.crucible/` JSON definitions already contain this information (parameter types, return types, method names, dependencies). The validator compares fingerprints: if the architecture says `PatientService.findById` takes `(string, string) -> Promise<Patient>` with effects `[database.read, audit.log]`, but the code's fingerprint shows `findById(id: String) -> Result<Patient>` with no audit dependency, that is a structural mismatch. No annotations in source code needed.

**Maintenance cost:** Low -- The parser changes are incremental (extend `extract_exports` to also extract signatures), and the comparison logic is a new validation rule added to `Validator.validate()`. Someone reads: `rust_parser.rs`, `validator.rs`, and the new `structural_match.rs`. Three files.

**10x scale path:** Add -- New parsers for new languages produce the same `StructuralFingerprint` type. The comparison logic is language-agnostic. Adding Go means writing `GoParser` that produces fingerprints, nothing else changes.

**Fastest validation:** Build signature extraction for Rust structs/fns (extend `extract_exports` to capture `fn name(param: Type) -> ReturnType`) and a `check_structural_match` validation rule. **Estimated effort: 2-3 days.**

**Biggest risk:** Regex-based parsing cannot reliably extract type signatures from complex Rust code (generics, lifetimes, trait bounds). This works for 80% of real code but has a long tail of edge cases.

**Tradeoffs:** You gain zero-annotation verification (developers never touch architecture metadata). You lose the ability to express *intent that is invisible in structure* -- things like `@phi` or `@requires-auth` have no structural manifestation in code. This approach verifies shape, not semantics.

**Code sketch:**
```rust
/// Structural fingerprint of a code element
pub struct StructuralFingerprint {
    pub name: String,
    pub kind: ExportType, // Class/Function/Enum/etc
    pub methods: HashMap<String, MethodSignature>,
    pub properties: HashMap<String, String>, // name -> type
    pub dependencies: Vec<String>, // imported modules
}

pub struct MethodSignature {
    pub params: Vec<(String, String)>, // (name, type)
    pub return_type: String,
    pub is_async: bool,
}

// New validation rule in validator.rs
fn check_structural_match(&self, discovered: &[DiscoveredModule]) -> Option<Vec<ValidationIssue>> {
    // For each architecture module, find corresponding discovered module
    // Compare fingerprints, report drift
}
```

---

### Approach 2: Architecture as Test Oracle (from Property-Based Testing / QuickCheck)

**Philosophy:** Do not annotate code at all. Instead, treat the `.crucible/` architecture definitions as *test oracles* and generate verification tests that run as part of `cargo test` / `npm test`, the way property-based testing generates assertions from invariants.

**How it works:** A `crucible generate-tests` command reads `.crucible/` definitions and emits test files that verify architectural properties using the language's own reflection/import system. For Rust: generate `#[test]` functions that attempt `use crate::module::ExportName` to verify exports exist, check that public method signatures match (using `std::any::type_name` or compile-time assertions), and verify dependency boundaries by checking that certain `use` paths do NOT compile (using `trybuild` or `compile_fail` doc tests). For TypeScript: generate `.test.ts` files that import types and use `satisfies` or `extends` checks. The key insight from property-based testing: the architecture definition IS the specification; we generate the test, not the annotation.

**Maintenance cost:** Medium -- Generated test files need to be re-generated when architecture changes. But the generator is a single module (`test_generator.rs`), and the generated tests are standard language tests that developers already know how to read. Two files to understand: the generator and an example generated test.

**10x scale path:** Add -- Each language needs a test template (Handlebars templates already exist in the codebase via `crucible-core/src/claude/templates.rs`). Adding a language means adding a template, not changing the generator logic.

**Fastest validation:** Generate simple import/existence tests for Rust exports. `crucible generate-tests --target tests/architecture_tests.rs`. **Estimated effort: 1-2 days for a basic proof of concept** (just export existence checks).

**Biggest risk:** Generated tests verify existence and shape but cannot verify behavioral properties (effects, annotations). Also, generated test files can become stale if someone forgets to re-run the generator. This is mitigatable with a CI check.

**Tradeoffs:** You gain tests that run in the language's native toolchain (cargo test, npm test) -- no new tools to learn, no new CI steps beyond what already exists. You lose real-time validation (must re-generate and re-run). Also, compile-time boundary checks are powerful in Rust but weak in TypeScript.

**Code sketch:**
```rust
// test_generator.rs
pub struct ArchitectureTestGenerator {
    project: Project,
}

impl ArchitectureTestGenerator {
    pub fn generate_rust_tests(&self) -> String {
        let mut output = String::from("//! Auto-generated architecture verification tests\n\n");
        for module in &self.project.modules {
            for (export_name, export) in &module.exports {
                // Generate existence test
                output.push_str(&format!(
                    "#[test]\nfn verify_{module}_{export}_exists() {{\n    \
                     use crate::{module}::{export_name};\n}}\n\n",
                    module = module.module, export = export_name
                ));

                // Generate method signature tests
                if let Some(methods) = &export.methods {
                    for (method_name, method) in methods {
                        // Generate compile-time assertion that method exists with expected signature
                    }
                }
            }
        }
        output
    }
}
```

---

### Approach 3: Effect Provenance Tracking (from Database Lineage / Supply Chain Security)

**Philosophy:** The hardest thing to verify is not structure (exports, types, signatures) but *behavior* (effects, data flow annotations like `@phi`, access patterns). Borrow from data lineage tracking in databases: instead of annotating individual code elements, track the *provenance chain* of how sensitive data flows through the system and verify it against declared effects.

**How it works:** This inverts the decorator model. Instead of developers tagging code with `@phi`, the architecture definitions already declare which properties carry `@phi` annotations. Crucible builds a *data flow graph* from the architecture: if `Patient.ssn` has `@phi`, and `PatientService.findById` returns `Patient`, then `findById` implicitly accesses PHI. The validator can then check: does `findById` declare `audit.log` in its effects? Does it have `@requires-auth`? This is already how `crucible-compliance/src/validator.rs` works (see `collect_property_annotations` which gathers annotations from properties to validate methods). The extension is: do this *across module boundaries* using the dependency graph. If module A depends on module B, and module B exports a type with `@phi` properties, then any method in module A that accepts that type is implicitly handling PHI and must comply with the corresponding rules.

**Maintenance cost:** Low -- This is an extension of the existing compliance validator pattern, not a new system. The `collect_property_annotations` function in `crucible-compliance/src/validator.rs` already does single-module provenance. Extending it to cross-module provenance using the dependency graph in `crucible-core/src/graph.rs` is the main work. Two files change: `compliance/validator.rs` and `core/graph.rs`.

**10x scale path:** Add -- The provenance graph is built from architecture JSON, not source code. Adding languages does not change the analysis at all. Adding new compliance frameworks means adding new rules, not new provenance logic.

**Fastest validation:** Extend `ComplianceValidator.validate_method` to resolve types across module boundaries using the dependency graph, propagating annotations transitively. **Estimated effort: 2-3 days.**

**Biggest risk:** Transitive annotation propagation can produce false positives. If module A uses a type from module B that has one `@phi` field among 20 fields, does every method in A need audit logging? The answer depends on whether the method actually accesses the PHI field, which architecture definitions do not capture at that granularity.

**Tradeoffs:** You gain automatic compliance checking without any source code annotations -- the architecture definitions are sufficient. You lose precision: the analysis is conservative (assumes any method receiving a PHI-containing type might access PHI fields). This is the right tradeoff for compliance (false positives are better than false negatives in HIPAA).

**Code sketch:**
```rust
/// Transitive annotation resolver using the dependency graph
pub struct AnnotationProvenance {
    /// Map of type_name -> annotations propagated from its properties
    type_annotations: HashMap<String, Vec<String>>,
}

impl AnnotationProvenance {
    pub fn build_from_project(project: &Project) -> Self {
        let mut type_annotations = HashMap::new();

        // Phase 1: Collect direct annotations from all properties
        for module in &project.modules {
            for (export_name, export) in &module.exports {
                let mut annotations = Vec::new();
                if let Some(props) = &export.properties {
                    for prop in props.values() {
                        annotations.extend(prop.annotations.clone());
                    }
                }
                if let Some(payload) = &export.payload {
                    for prop in payload.values() {
                        annotations.extend(prop.annotations.clone());
                    }
                }
                annotations.sort();
                annotations.dedup();
                type_annotations.insert(
                    format!("{}.{}", module.module, export_name),
                    annotations,
                );
            }
        }

        // Phase 2: Propagate through method parameters
        // If method takes Patient (which has @phi), the method context inherits @phi

        Self { type_annotations }
    }

    /// Get all annotations that a method is exposed to via its parameter types
    pub fn method_exposure(&self, method: &Method) -> Vec<String> {
        method.inputs.iter()
            .flat_map(|param| {
                self.type_annotations
                    .get(&param.param_type)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect()
    }
}
```

---

## Challenge to Assumptions

**Assumption I challenge: "Developers need to add decorators/annotations to their source code."**

This assumption comes from the Java/Python decorator tradition where annotations are runtime-meaningful (`@Override`, `@Inject`, `@Transactional`). But Crucible is a *static architecture verifier*, not a runtime framework. The architecture JSON files already contain all the metadata. Adding source-code annotations creates a synchronization problem (JSON says X, code says Y -- who wins?) that is strictly harder than the original problem (JSON says X, code does Y -- does it?).

The three approaches above all avoid source-code annotations entirely:
1. **Structural Fingerprinting** verifies shape without annotations
2. **Test Oracle** generates verification from the JSON spec
3. **Effect Provenance** derives behavioral properties transitively from the JSON spec

If Crucible must have source-code decorators, the strongest argument is developer experience: a developer reading `patient_service.rs` should see that it handles PHI without opening a JSON file. But that is a documentation/DX problem, not a verification problem. A better solution for DX is an IDE overlay or `crucible annotate` that prints architecture metadata alongside code, rather than embedding it in comments that must be kept in sync.

**The domain I draw from: supply chain security (SBOM/SLSA).** In software supply chain security, you do not ask every dependency to self-report its provenance. Instead, you build provenance chains from external evidence (build logs, signatures, dependency graphs). Crucible should work the same way: the `.crucible/` definitions are the "supply chain manifest," and verification means checking that the actual artifacts (source code) match the manifest -- not asking the artifacts to carry their own labels.
