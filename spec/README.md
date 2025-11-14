# Crucible

**An open standard for AI-native application architecture**

Crucible provides a structured, machine-readable format for defining application architecture that AI coding assistants can use to maintain consistency, validate changes, and generate code.

## The Problem

AI coding assistants are powerful but struggle with:
- **Context loss** across sessions
- **Inconsistency** across multiple files
- **Architectural drift** over time
- **Validation** before code is written

Current solutions (PRDs, comments, docs) are text-based and lose structure when converted to code and back.

## The Solution

Crucible is a formal specification language for application architecture:

```json
{
  "module": "auth",
  "exports": {
    "AuthService": {
      "type": "class",
      "methods": {
        "login": {
          "inputs": [
            {"name": "email", "type": "string"},
            {"name": "password", "type": "string"}
          ],
          "returns": {"type": "Promise", "inner": "User"},
          "calls": ["database.UserRepository.findByEmail"],
          "effects": ["database.read"]
        }
      }
    }
  }
}
```

## Key Features

### ✅ AI-Native Design
Optimized for AI consumption first, human readability second. AI assistants read architecture files to understand your system.

### ✅ Validation Before Code
Catch architectural issues before writing a single line:
- Circular dependencies
- Type mismatches
- Layer violations
- Missing declarations

### ✅ Language-Agnostic
Works with TypeScript, Rust, Python, Go, Java. One architecture, multiple implementations.

### ✅ Incremental Adoption
Start with high-level modules, add details as needed. Works alongside existing code.

### ✅ Open Standard
No vendor lock-in. Any tool can implement Crucible support.

## Quick Example

**Define architecture:**
```bash
.crucible/
├── manifest.json       # Project config
├── modules/
│   ├── auth.json      # Auth module
│   └── database.json  # Database module
└── rules.json         # Validation rules
```

**Validate:**
```bash
$ crucible validate

✓ No circular dependencies
✓ All types exist
✓ All calls reference exported functions
✗ Error: auth.AuthService.login calls database.UserRepository.findByEmail
  but database is not declared as a dependency

Architecture is invalid!
```

**Fix and regenerate:**
```bash
# Add dependency
# Run validation again
$ crucible validate

✓ Architecture is valid!

# Generate interfaces
$ crucible generate --lang=typescript
Generated: src/generated/auth.ts
```

## Use Cases

### AI-Assisted Development
- Claude Code, Copilot, Cursor use architecture as context
- Maintain consistency across AI-generated code
- Catch mistakes before they happen

### Architecture Governance
- Enforce architectural patterns (layered, hexagonal, microservices)
- Validate changes in CI/CD
- Document architectural decisions

### Code Generation
- Generate interfaces from architecture
- Create boilerplate conforming to spec
- Maintain type safety

### Refactoring
- Visualize dependency graphs
- Plan refactoring safely
- Validate changes before implementation

## How It Works

```
1. Define Architecture
   ↓
   .crucible/ files (JSON)
   
2. AI Reads Architecture
   ↓
   Claude/Copilot understand your system
   
3. Validate Changes
   ↓
   crucible validate (catches issues)
   
4. Generate Code
   ↓
   Interfaces, types, boilerplate
   
5. Implement
   ↓
   AI-assisted or manual
```

## Comparison

| Approach | AI Context | Validatable | Language-Agnostic | Executable |
|----------|------------|-------------|-------------------|------------|
| **Comments** | Poor | No | Yes | No |
| **PRDs** | Medium | No | Yes | No |
| **OpenAPI** | Good | Yes | Yes | APIs only |
| **TypeScript .d.ts** | Good | Yes | No | Types only |
| **Crucible** | **Excellent** | **Yes** | **Yes** | **Full App** |

## Project Structure

```
crucible/
├── spec/                   # Specification documents
│   ├── SPEC.md            # Full specification
│   ├── schema.json        # JSON Schema definitions
│   └── examples/          # Example architectures
│
├── crucible-core/              # Rust implementation (planned)
│   ├── parser/            # Parse .crucible/
│   ├── validator/         # Validation engine
│   └── graph/             # Dependency analysis
│
├── crucible-cli/               # Command-line tool (planned)
│
├── crucible-codegen/           # Code generators (planned)
│   ├── typescript/
│   ├── rust/
│   └── python/
│
└── integrations/          # Tool integrations (planned)
    ├── claude-code/
    ├── vscode/
    └── github-actions/
```

## Getting Started

### For Users

**Option 1: Manual Setup (Now)**

1. Create `.crucible/` directory
2. Copy example files from `spec/examples/`
3. Customize for your project
4. Share with AI assistants

**Option 2: CLI Tool (Coming Soon)**

```bash
cargo install crucible-cli
crucible init
crucible validate
```

See [GETTING-STARTED.md](GETTING-STARTED.md) for detailed guide.

### For Tool Builders

Implement Crucible support in your AI coding assistant:

1. Parse `.crucible/` directory (see schema.json)
2. Use architecture as context for code generation
3. Validate changes against architecture
4. Update architecture files when changing structure

Reference implementation coming soon in Rust.

### For Contributors

We need help with:
- **Rust implementation** - Parser, validator, CLI
- **Code generators** - TypeScript, Python, Rust, Go
- **Integrations** - Claude Code, Copilot, Cursor
- **Documentation** - Examples, tutorials, guides
- **Feedback** - Use cases, pain points, proposals

## Roadmap

### Phase 1: Specification (Current)
- ✅ Core schema design
- ✅ JSON Schema definitions
- ✅ Example architectures
- ✅ Documentation

### Phase 2: Reference Implementation (Next 2 months)
- [ ] Rust parser
- [ ] Core validation rules
- [ ] CLI tool (validate, graph, analyze)
- [ ] TypeScript code generator

### Phase 3: Integrations (Months 3-6)
- [ ] Claude Code integration
- [ ] VS Code extension
- [ ] GitHub Actions
- [ ] CI/CD tooling

### Phase 4: Ecosystem (Months 6-12)
- [ ] Community code generators (Python, Rust, Go)
- [ ] Advanced analysis (concurrency, performance)
- [ ] Visualization tools
- [ ] Enterprise features

## Examples

### Todo App (Simple CRUD)
```
modules:
  - auth    (authentication)
  - todo    (todo management)
  - user    (user management)
  - database (data persistence)
  - api     (HTTP endpoints)

pattern: layered
validation: strict
```

See `spec/examples/todo-app/` for complete example.

### E-Commerce (Multi-Module)
```
modules:
  - catalog  (product catalog)
  - cart     (shopping cart)
  - order    (order management)
  - payment  (payment processing)
  - user     (user accounts)
  - inventory (stock management)

pattern: hexagonal
validation: strict
```

Coming soon in `spec/examples/e-commerce/`.

## Community

- **Discussions**: GitHub Discussions (coming soon)
- **Issues**: GitHub Issues (coming soon)
- **Contributing**: See CONTRIBUTING.md (coming soon)

## Philosophy

### AI-First, Not AI-Only
While optimized for AI consumption, Crucible remains human-readable and editable. It's a collaboration tool.

### Minimal and Extensible
Start with essentials (modules, interfaces, dependencies). Extend as patterns emerge.

### Validation Over Documentation
Architecture isn't just docs—it's executable specification that catches errors.

### Open Standard
No vendor lock-in. Any tool can implement. Community-driven evolution.

### Pragmatic Adoption
Works alongside existing code. Incremental migration. No big bang required.

## Frequently Asked Questions

**Q: Do I have to use AI coding assistants to use Crucible?**
A: No. Crucible is valuable for architecture documentation, validation, and code generation even without AI.

**Q: Can I use this with my existing codebase?**
A: Yes. Start by documenting your current architecture, then validate new changes against it.

**Q: How is this different from OpenAPI?**
A: OpenAPI defines HTTP APIs. Crucible defines entire application architectures including internal modules, dependencies, and business logic.

**Q: What about database schemas?**
A: Use existing tools (Prisma, SQLAlchemy). Crucible focuses on application architecture, not data modeling.

**Q: Can I define my own validation rules?**
A: Yes. See the custom_rules section in rules.json.

**Q: Is this only for TypeScript?**
A: No. Crucible is language-agnostic. We'll support TypeScript, Rust, Python, Go, and Java.

**Q: How do I keep architecture and code in sync?**
A: Validate in CI/CD. AI assistants can also update architecture as they change code.

**Q: Can this detect architectural violations?**
A: Yes. That's a core feature—detect layer violations, circular dependencies, etc.

## Technical Details

**Format**: JSON (human-readable, widely supported)
**Validation**: JSON Schema + custom rules engine
**Versioning**: Semantic versioning (semver)
**Type System**: Simplified, maps to common languages
**Effect System**: Declares side effects (DB, network, file I/O)

See [SPEC.md](spec/SPEC.md) for complete technical specification.

## Status

**Current Version**: 0.1.0 (Specification only)
**Stage**: Early specification and feedback
**License**: 
- **Specification**: [CC0 1.0 Universal](LICENSE-SPEC) (Public Domain) - Use freely, no restrictions
- **Implementation Code** (future): [Apache 2.0](LICENSE-CODE) - Permissive with patent grant
- See [LICENSING.md](LICENSING.md) for complete details

## Acknowledgments

Inspired by:
- OpenAPI/Swagger (API specifications)
- TypeScript (type definitions)
- Terraform (infrastructure as code)
- Protocol Buffers (interface definitions)

Special thanks to the AI-assisted development community for highlighting the need for better architectural context.

## Contact

- **Repository**: Coming soon
- **Specification**: [SPEC.md](spec/SPEC.md)
- **Examples**: [examples/](spec/examples/)
- **Questions**: GitHub Discussions (coming soon)

---

**Built for the AI era. Open for everyone.**

⭐ Star this repo if you believe architecture should be formal, validatable, and AI-native.
