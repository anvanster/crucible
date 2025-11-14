# Crucible Rebrand Summary

## What Changed

### Name
- **Old**: ADK (Architecture Definition Kit)
- **New**: Crucible

### Rationale
- Avoid conflicts with existing CLI tools named "adk"
- More memorable and distinctive name
- Metaphor aligns with project goals: refining and testing architecture under pressure

## Updated Components

### Directory Structure
- **Old**: `.architecture/`
- **New**: `.crucible/`

### Commands
- **Old**: `adk validate`, `adk init`, `adk generate`
- **New**: `crucible validate`, `crucible init`, `crucible generate`

### Package Names
- **Old**: `adk-cli`, `adk-core`, `adk-codegen`
- **New**: `crucible-cli`, `crucible-core`, `crucible-codegen`

### Repository
- **Old**: `github.com/adk-spec`
- **New**: `github.com/crucible-spec`

### Website/Domain
- **Old**: `adk.dev`
- **New**: `crucible.dev`

## Files in This Package

All files have been updated with the new branding:

1. ✅ **README.md** - Project overview
2. ✅ **SPEC.md** - Technical specification
3. ✅ **GETTING-STARTED.md** - Tutorial guide
4. ✅ **PROJECT-STRUCTURE.md** - Organization guide
5. ✅ **INDEX.md** - Navigation guide
6. ✅ **BRANDING.md** - Branding guidelines (NEW)
7. ✅ **schema.json** - JSON Schema definitions
8. ✅ **example-manifest.json** - Example project config
9. ✅ **example-module-auth.json** - Auth module example
10. ✅ **example-module-todo.json** - Todo module example
11. ✅ **example-module-api.json** - API module example
12. ✅ **example-rules.json** - Validation rules example

## What Stayed the Same

- File format (JSON)
- Schema structure
- Validation philosophy (strict)
- Code generation approach (minimal interfaces first)
- License approach (dual licensing with CC0 + Apache 2.0)
- Version (0.1.0)

## Licensing

Crucible uses dual licensing:
- **Specification files**: CC0 1.0 Universal (Public Domain)
- **Implementation code**: Apache 2.0 (when released)

This approach maximizes adoption of the standard while protecting contributors. See [LICENSING.md](LICENSING.md) for complete details.

## Quick Reference

### Project Structure
```
your-project/
├── .crucible/              # ← Changed from .architecture/
│   ├── manifest.json
│   ├── modules/
│   │   ├── auth.json
│   │   └── ...
│   └── rules.json
└── src/
    └── ...
```

### Basic Commands
```bash
# Install
cargo install crucible-cli         # ← Changed from adk-cli

# Initialize
crucible init                       # ← Changed from adk

# Validate
crucible validate

# Generate code
crucible generate --lang=typescript
```

### Import in Documentation
```markdown
# Old references
- ADK specification
- .architecture/ directory
- adk validate command
- github.com/adk-spec

# New references
- Crucible specification
- .crucible/ directory
- crucible validate command
- github.com/crucible-spec
```

## Migration for Early Adopters

If you've already started using ADK (unlikely since we just released the spec):

1. **Rename directory**: `.architecture/` → `.crucible/`
2. **Update commands**: Replace `adk` with `crucible` in scripts
3. **Update imports**: Change any references to adk packages
4. **Git commit**: `git mv .architecture .crucible`

## Communication Guidelines

### When Announcing
"We've renamed ADK to **Crucible** to avoid conflicts with existing tools and to better reflect the project's purpose: refining and testing architecture under pressure."

### When Explaining
"The name 'Crucible' comes from the vessel used to refine metals at high temperatures—just as this tool refines architectural ideas into validated specifications."

### Hashtags
- #Crucible (primary)
- #AIAssistedDev
- #ArchitectureAsCode

## Timeline

- **November 13, 2025**: Initial ADK specification created
- **November 14, 2025**: Renamed to Crucible
- **Next**: Public release with Crucible branding

## Next Steps

1. ✅ Update all documentation (DONE)
2. ✅ Update all examples (DONE)
3. ⬜ Create GitHub repository: `crucible-spec/crucible`
4. ⬜ Reserve domain: `crucible.dev`
5. ⬜ Update social media references
6. ⬜ Begin Rust implementation with new naming

## Notes

- All existing spec files updated in this package
- No breaking changes to the actual specification format
- Only naming and branding changed
- Schema semantics remain identical

## Questions?

See **BRANDING.md** for comprehensive branding guidelines, messaging, and usage conventions.

---

**Remember**: It's not just a rename—it's about refining architecture through validation, just like a crucible refines raw materials into something pure and strong.
