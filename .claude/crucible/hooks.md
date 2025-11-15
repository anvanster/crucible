# Crucible Validation Hooks

These prompts help maintain architectural integrity during development.

## 🎯 Pre-Change Validation

Before implementing any feature or fix, validate your approach:

### Checklist for New Features

- [ ] Have I identified which module this belongs to?
- [ ] Is this module in the correct architectural layer?
- [ ] Are all required dependencies already declared?
- [ ] Will this create any circular dependencies?
- [ ] Do the types I need exist in accessible modules?
- [ ] Am I following the naming conventions?

### Checklist for Module Modifications

- [ ] Am I only using exported functions from other modules?
- [ ] Are my new exports properly documented?
- [ ] Do my function signatures match the architecture?
- [ ] Have I checked layer boundary constraints?
- [ ] Will my changes break existing contracts?

## 🔄 Post-Change Sync

After implementing changes, ensure architecture stays synchronized:

### Validation Commands

```bash
# After adding new code
crucible validate <module>

# After refactoring
crucible validate --all

# To sync architecture with code
crucible claude sync --from-code
```

## ⚠️ Common Violations to Avoid

### 1. Layer Violations
❌ **Wrong**: Domain layer importing from Presentation layer
✅ **Right**: Presentation layer importing from Domain layer

### 2. Circular Dependencies
❌ **Wrong**: Module A → Module B → Module A
✅ **Right**: Module A → Module B → Module C

### 3. Undeclared Dependencies
❌ **Wrong**: Using a module without declaring it in dependencies
✅ **Right**: First declare dependency, then use the module

