# Crucible Specification Package - File Index

This package contains all the specification documents and examples for Crucible v0.1.0 - an open standard for AI-native application architecture.

**Crucible** (noun): *A vessel for refining under intense heat; a severe test or trial.*

Just as a crucible refines raw materials, this specification refines architectural ideas into validated, AI-readable definitions.

## Core Documentation

### [README.md](computer:///mnt/user-data/outputs/README.md)
**Start here!** Overview of Crucible, key features, use cases, and project vision. This is the main entry point for understanding what Crucible is and why it exists.

### [SPEC.md](computer:///mnt/user-data/outputs/SPEC.md)
**Complete technical specification.** Detailed documentation of:
- File structure and schema definitions
- Module, export, and dependency systems
- Validation rules and type system
- Effect system and architectural patterns
- Tool integration guidelines

### [GETTING-STARTED.md](computer:///mnt/user-data/outputs/GETTING-STARTED.md)
**Quick start guide.** Step-by-step instructions for:
- Installing and initializing Crucible
- Creating your first module
- Running validation
- Generating code
- Integrating with AI assistants

### [PROJECT-STRUCTURE.md](computer:///mnt/user-data/outputs/PROJECT-STRUCTURE.md)
**Organization guide.** Shows how to structure Crucible files in your project:
- Complete directory layouts
- Module organization patterns
- Best practices

### [BRANDING.md](computer:///mnt/user-data/outputs/BRANDING.md)
**Branding and messaging guide.** Covers:
- Name origin and meaning
- Taglines and positioning
- Usage conventions
- Community language
- Competitive positioning

### [LICENSING.md](computer:///mnt/user-data/outputs/LICENSING.md)
**Licensing guide.** Explains:
- Dual licensing approach (CC0 + Apache 2.0)
- What each license covers
- FAQ about usage and contributions
- License headers and attribution
- Contributing guidelines

## License Files

### [LICENSE-SPEC](computer:///mnt/user-data/outputs/LICENSE-SPEC)
**CC0 1.0 Universal** (Public Domain) - Applies to all specification files, documentation, examples, and schema definitions.

### [LICENSE-CODE](computer:///mnt/user-data/outputs/LICENSE-CODE)
**Apache License 2.0** - Will apply to all implementation code (CLI tools, libraries, generators) when released.

### [LICENSE-QUICK-REF.md](computer:///mnt/user-data/outputs/LICENSE-QUICK-REF.md)
**Quick reference card** - Fast answers to common licensing questions without reading the full legal text.
- Migration path for existing projects
- CI/CD integration

## Schema Definition

### [schema.json](computer:///mnt/user-data/outputs/schema.json)
**JSON Schema validation file.** Use this for:
- IDE autocomplete and validation
- Automated schema validation
- Understanding exact format requirements
- Tool integration

## Example Architecture Files

These files show a complete todo-app architecture:

### [example-manifest.json](computer:///mnt/user-data/outputs/example-manifest.json)
Project manifest showing:
- Crucible version configuration
- Project metadata (name, language, architecture pattern)
- Module listing
- Validation settings

### [example-module-auth.json](computer:///mnt/user-data/outputs/example-module-auth.json)
Authentication module with:
- AuthService class with login/logout/register methods
- User authentication types
- Error definitions
- Dependencies on user, crypto, and session modules

### [example-module-todo.json](computer:///mnt/user-data/outputs/example-module-todo.json)
Todo management module with:
- TodoService for business logic
- TodoRepository for data access
- Todo data types and filters
- CRUD operations

### [example-module-api.json](computer:///mnt/user-data/outputs/example-module-api.json)
HTTP API layer with:
- TodoController for todo endpoints
- AuthController for authentication endpoints
- Request/Response types
- Presentation layer concerns

### [example-rules.json](computer:///mnt/user-data/outputs/example-rules.json)
Validation rules showing:
- Layered architecture definition
- Built-in validation rules
- Custom validation rules
- Naming conventions

## How to Use These Files

### For Understanding Crucible
1. Read [README.md](computer:///mnt/user-data/outputs/README.md) for overview
2. Review [GETTING-STARTED.md](computer:///mnt/user-data/outputs/GETTING-STARTED.md) for practical guide
3. Refer to [SPEC.md](computer:///mnt/user-data/outputs/SPEC.md) for technical details

### For Starting a New Project
1. Copy example files to your project
2. Customize `example-manifest.json` for your project
3. Modify module files or create new ones
4. Update `example-rules.json` for your architecture
5. Follow [PROJECT-STRUCTURE.md](computer:///mnt/user-data/outputs/PROJECT-STRUCTURE.md) for organization

### For Tool Development
1. Use [schema.json](computer:///mnt/user-data/outputs/schema.json) for validation
2. Reference [SPEC.md](computer:///mnt/user-data/outputs/SPEC.md) for implementation details
3. Study example files for reference implementations

### For Contributing
1. Read [SPEC.md](computer:///mnt/user-data/outputs/SPEC.md) to understand current design
2. Review examples to see patterns
3. Propose changes via GitHub issues (coming soon)

## Quick Start Checklist

- [ ] Read README.md to understand Crucible
- [ ] Review GETTING-STARTED.md for practical guide
- [ ] Copy example files to `.crucible/` in your project
- [ ] Customize manifest.json for your project
- [ ] Create or modify module definitions
- [ ] Set up rules.json with your architectural constraints
- [ ] Validate your architecture (when CLI is available)

## File Sizes

- SPEC.md: ~13KB (comprehensive technical documentation)
- README.md: ~10KB (project overview)
- GETTING-STARTED.md: ~8.5KB (tutorial)
- PROJECT-STRUCTURE.md: ~11KB (organization guide)
- schema.json: ~11KB (validation schema)
- example-manifest.json: ~400B (simple config)
- example-module-auth.json: ~3.5KB (auth module)
- example-module-todo.json: ~7.4KB (todo module)
- example-module-api.json: ~7.1KB (API layer)
- example-rules.json: ~2.5KB (validation rules)

**Total package size: ~74KB**

## Next Steps

1. **Set up your project**: Create `.crucible/` directory
2. **Start with examples**: Copy and customize example files
3. **Share feedback**: Help improve the specification
4. **Build tooling**: Implement parsers, validators, generators
5. **Integrate with AI**: Use with Claude Code, Copilot, Cursor

## Repository Structure (Recommended)

When setting up a repository for Crucible:

```
crucible/
├── spec/
│   ├── SPEC.md                      # This package
│   ├── GETTING-STARTED.md
│   ├── PROJECT-STRUCTURE.md
│   ├── schema.json
│   └── examples/
│       ├── todo-app/
│       │   ├── manifest.json
│       │   ├── modules/
│       │   │   ├── auth.json
│       │   │   ├── todo.json
│       │   │   └── api.json
│       │   └── rules.json
│       └── [more examples]
│
├── crucible-core/          # Rust implementation (future)
├── crucible-cli/           # CLI tool (future)
└── README.md          # Project README
```

## Version Information

- **Crucible Specification**: v0.1.0
- **Status**: Initial specification release
- **Date**: November 2025
- **License**: 
  - Specification: CC0 1.0 Universal (Public Domain)
  - Implementation Code: Apache 2.0 (when released)

## Support

- **Questions**: GitHub Discussions (coming soon)
- **Issues**: GitHub Issues (coming soon)
- **Contributing**: See SPEC.md section on contributing

---

**Built for the AI era. Open for everyone.**

Thank you for your interest in Crucible!
