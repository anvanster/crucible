# Project Architecture Guidelines

This project uses **Crucible** for formal architecture management. Claude should read and respect these architectural definitions when generating or modifying code.

## 🏗️ Architecture Overview

This project follows a **layered** pattern with clear separation of concerns.

## 📦 Module Structure

The application is divided into the following modules:

### primitives Module (`primitives`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: Path, HashMap<String, String>, Result<Module>, Option<IntegrationMode>, Vec<String>, u64, Result<Vec<Module>>, Result<IntegrationConfig>, HashMap<String, Method>, Option<Rules>, Result<Project>, void, Vec<ValidationIssue>, Vec<Dependency>, String, HashMap<String, Export>, Option<Module>, Option<ValidationLevel>, Result<void>, Option<ArchitecturePattern>, Option<Metadata>, Result<Manifest>, Result<Vec<String>>, Option<String>, HashSet<String>, Option<Vec<ValidationIssue>>, Vec<Parameter>, HashMap<String, ValidationLevel>, PathBuf, Result<Rules>, boolean, Option<Project>, Result<GenerationResult>, HashMap<String, Property>, Option<GlobalConfig>, Vec<Module>, usize, HashMap<String, boolean>, Option<Path>

### error Module (`error`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: Result, CrucibleError

### types Module (`types`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: ReturnType, Architecture, Severity, Method, Rules, Layer, CustomRule, Dependency, Metadata, ProjectConfig, Project, Property, Manifest, ArchitecturePattern, Module, Export, Language, ExportType, Rule, Parameter

### cache Module (`cache`)
- **Layer**: infrastructure
- **Can depend on**: primitives, types, error
- **Key exports**: ArchitectureCache, CacheStats

### performance Module (`performance`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: GlobalConfig, Vec<string>, HashMap<string, boolean>, PerformanceConfig, ChangeTracker, ArchitectureCache, CacheStats

### parser Module (`parser`)
- **Layer**: infrastructure
- **Can depend on**: primitives, types, performance
- **Key exports**: Parser

### validator Module (`validator`)
- **Layer**: infrastructure
- **Can depend on**: primitives, performance, types
- **Key exports**: ValidationResult, ValidationIssue, HashMap<string, boolean>, Validator

### generator Module (`generator`)
- **Layer**: infrastructure
- **Can depend on**: primitives, types
- **Key exports**: Generator, GenerationResult

### config Module (`config`)
- **Layer**: infrastructure
- **Can depend on**: types, performance, primitives
- **Key exports**: IntegrationMode, ValidationLevel, IntegrationConfig, ValidationConfig

### context Module (`context`)
- **Layer**: application
- **Can depend on**: error, types, config
- **Key exports**: ContextSummary, ContextGenerator

## ⚠️ CRITICAL: Architecture-First Development

**This project uses architecture-first development. You MUST follow this workflow:**

### 🔴 STOP: Before Writing ANY Code

**When adding features, changing APIs, or modifying module interfaces:**

1. **UPDATE ARCHITECTURE FIRST**
   - Edit `.crucible/modules/<module>.json` to add new methods/exports
   - Create `.crucible/modules/<new-module>.json` for new modules
   - Update dependencies, method signatures, and types in architecture
   - Add new modules to `.crucible/manifest.json`

2. **VALIDATE ARCHITECTURE**
   ```bash
   crucible validate
   ```
   - Fix ALL violations before proceeding
   - Resolve layer boundaries, circular dependencies, missing types
   - Re-validate until zero errors

3. **ONLY AFTER VALIDATION PASSES: Write Code**
   - Implement code matching the validated architecture
   - Follow exact signatures from architecture definitions
   - Use only declared dependencies

4. **VERIFY IMPLEMENTATION**
   ```bash
   cargo build  # or npm build, etc.
   cargo test   # verify tests pass
   ```

### ❌ Anti-Pattern (Code-First)

```
1. Write code → 2. Compilation errors → 3. Fix code → 
4. Architecture violations → 5. Update architecture → 6. Fix code again
Result: 7-10 iterations, 16,500 tokens wasted
```

### ✅ Correct Pattern (Architecture-First)

```
1. Update architecture → 2. Validate architecture → 3. Fix violations → 
4. Write code → 5. Build succeeds → 6. Tests pass
Result: 1-2 iterations, 4,500 tokens, zero violations
```

### 📋 Pre-Implementation Checklist

Before writing code, verify:

- [ ] Architecture definition exists in `.crucible/modules/`
- [ ] New methods/types added to module's `exports`
- [ ] Dependencies declared in `dependencies` section
- [ ] `crucible validate` shows ZERO errors
- [ ] Layer boundaries respected (core → infrastructure → application → presentation)
- [ ] No circular dependencies detected

## 🚫 Architectural Rules

The following rules are enforced:

1. **No Circular Dependencies**: Modules cannot depend on each other in cycles
2. **Layer Boundaries**: Lower layers cannot depend on higher layers
3. **Explicit Dependencies**: All external module usage must be declared
4. **Type Safety**: All referenced types must exist
5. **Export Validation**: Only exported functions can be called externally

## 💡 Quick Commands

```bash
# Validate current architecture
crucible validate

# Check specific module
crucible validate <module-name>

# Sync architecture with code changes
crucible claude sync --from-code
```

---

**Remember**: The architecture is the source of truth. When in doubt, check the `.crucible/` definitions before making changes.
