# Project Architecture Guidelines

This project uses **Crucible** for formal architecture management. Claude should read and respect these architectural definitions when generating or modifying code.

## 🏗️ Architecture Overview

This project follows a **layered** pattern with clear separation of concerns.

## 📦 Module Structure

The application is divided into the following modules:

### types Module (`types`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: Manifest, Project, Export, Module

### error Module (`error`)
- **Layer**: core
- **Can depend on**: 
- **Key exports**: CrucibleError, Result

### parser Module (`parser`)
- **Layer**: core
- **Can depend on**: error, types
- **Key exports**: Parser

### validator Module (`validator`)
- **Layer**: core
- **Can depend on**: graph, error, types
- **Key exports**: Validator, ValidationResult

### graph Module (`graph`)
- **Layer**: core
- **Can depend on**: types
- **Key exports**: build_dependency_graph, detect_cycles

### generator Module (`generator`)
- **Layer**: application
- **Can depend on**: error, types
- **Key exports**: Language, Generator

### cli Module (`cli`)
- **Layer**: presentation
- **Can depend on**: validator, parser, types, error, generator
- **Key exports**: run

## ✅ Before Writing Code

**Always check these architectural constraints:**

1. **Layer Dependencies**: Ensure you're not violating layer boundaries
2. **Module Dependencies**: Check if the module you're modifying can depend on the module you're importing
   - Review `.crucible/modules/<module>.json` for allowed dependencies
3. **Interface Contracts**: When calling functions from other modules
   - Verify the function exists in the module's exports
   - Match the exact signature defined in the architecture
4. **Naming Conventions**: Follow established patterns

## 🔄 After Writing Code

**Update the architecture to maintain sync:**

1. **New Exports**: If you added public functions/classes
   ```bash
   crucible validate
   ```

2. **New Dependencies**: If you imported from a new module
   ```bash
   crucible validate <module-name>
   ```

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
