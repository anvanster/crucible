# Decision: Code Decorators for Architecture Verification
**Date:** 2026-02-21
**Status:** DECIDED

## Chosen Approach: Hybrid — Provenance Core + Optional Decorators

### The Decision

Build **cross-module effect provenance tracking** as the primary verification mechanism, with **source-code decorators as an optional DX layer** added later if users request it.

### Phased Execution

| Phase | What | Effort | Files Changed | Dependencies |
|-------|------|--------|---------------|-------------|
| 0 | Provenance spike (`cargo run --example provenance_spike`) | 2 hours | 1 new example file | None |
| 1 | Cross-module `AnnotationProvenance` in core validator | 1-2 days | 2 files (graph.rs, validator.rs) | None |
| 2 | Wire into compliance validator + SyncManager | 1 day | 2 files (compliance/validator.rs, sync.rs) | None |
| 3 | Optional: `// @crucible:` source-code decorators | 1 day | 3 files (rust_parser.rs, sync.rs, validator.rs) | None — only if users ask |

### Rationale

#### Why Provenance Over Decorators

Bravo identified the critical insight: **source-code decorators create a second source of truth that must be synchronized with the architecture JSON — which is the exact problem Crucible was built to solve.**

The `.crucible/` JSON definitions already contain all the annotation metadata (`@phi`, `@encrypted`, `@requires-auth` on properties and methods). Provenance tracking *derives* verification from this existing data by propagating annotations transitively across the dependency graph. If `Patient.ssn` has `@phi` and a method in `PatientService` accepts `Patient`, that method is implicitly handling PHI.

This is strictly more powerful than comment parsing:
- **Zero developer behavior change** — no comments to write or maintain
- **Cross-module analysis** — catches data flow violations that single-file parsing cannot
- **Already has precedent** — `crucible-compliance/src/validator.rs` already does single-module provenance via `collect_property_annotations`
- **Language-agnostic** — works from JSON, not source code

#### Why Keep Decorators as Phase 3 (Optional)

Source-code annotations solve a real DX problem: a developer reading `patient_service.rs` should see architectural metadata without opening a JSON file. But this is a *documentation* problem, not a *verification* problem. Build it only if users specifically ask for it.

#### Why Spike First (Phase 0)

Charlie's insight: the riskiest assumption hasn't been tested. For provenance, the risk is **false positive rate**. If every method that touches a type containing one `@phi` field gets flagged, the feature is unusable. The spike tests this specific risk with real data.

### What Was Sacrificed and Why

| Sacrificed | From | Why |
|-----------|------|-----|
| Annotation consistency within JSON (Phase 1) | Alpha | Already exists in crucible-compliance; duplicating it in crucible-core adds confusion |
| Structural fingerprinting | Bravo | Regex can't reliably extract type signatures from complex Rust (generics, lifetimes). 80% accuracy creates false safety |
| Test oracle generation | Bravo | Generated files go stale. Compile-time boundary checks are powerful in Rust but useless in TypeScript/Python |
| "Ship the boring thing first" | Alpha | The boring thing (comment parsing) actually creates a harder problem (dual source of truth). Sometimes boring is wrong |

### Evaluation Criteria Scores (Chosen Approach)

| Criteria | Score | Notes |
|----------|-------|-------|
| Maintenance cost | 2/5 | Extends existing compliance pattern, 2-3 files |
| Scale path (10x) | Add | Language-agnostic (works from JSON). New languages = new parsers for sync, provenance stays identical |
| Validation speed | Hours (spike) → Days (full) | Phase 0 validates in 2 hours |
| Integration effort | 2/5 | Extends existing `collect_property_annotations` + `petgraph` dependency graph |
| Risk level | 2/5 | Main risk is false positive rate, tested in Phase 0 |

### First Step

Build `crucible-core/examples/provenance_spike.rs`:
1. Load healthcare-portal example modules from JSON
2. Build `AnnotationProvenance` graph — propagate `@phi`/`@encrypted` from properties through method parameters across modules
3. Check every method exposed to sensitive annotations has required mitigations (`@requires-auth`, `audit.log` effect)
4. Print actionable warnings

**Expected output:**
```
Building provenance graph from 3 modules...

PatientService.findById: exposed to [@phi] via Patient return type
  WARNING: missing @requires-auth annotation
  WARNING: missing audit.log effect

PatientController.getPatient: exposed to [@phi] via PatientService.findById call chain
  WARNING: missing @requires-auth annotation

2 methods with provenance warnings, 0 false positives on non-sensitive methods.
```

**Success criteria:** The spike produces useful warnings on the healthcare example with ≤10% false positive rate (non-sensitive methods incorrectly flagged).

### Agents Referenced

- **Alpha** (agent-alpha.md): Provided the phased implementation strategy and the integration path for optional decorators
- **Bravo** (agent-bravo.md): Provided the winning approach (effect provenance) and the critical "second source of truth" insight
- **Charlie** (agent-charlie.md): Provided the spike-first methodology and identified the riskiest assumption
