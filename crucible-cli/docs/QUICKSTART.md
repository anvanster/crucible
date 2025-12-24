# 🚀 5-Minute Quickstart

Get started with Crucible in 5 minutes. Zero to validated architecture.

---

## Step 1: Install (30 seconds)

```bash
cargo install crucible-cli
crucible --version  # Verify installation
```

**Using cargo-binstall?**
```bash
cargo binstall crucible-cli
```

---

## Step 2: Create Your Project (1 minute)

**New project:**
```bash
crucible init --name my-app
cd my-app
```

**Existing project:**
```bash
cd my-existing-project
crucible init --here
```

You now have:
- `.crucible/manifest.json` - Project configuration
- `.crucible/rules.json` - Architecture rules
- `.crucible/modules/` - 3 example modules (user, user-service, user-controller)
- `.claude/commands/` - 8 Claude Code slash commands

---

## Step 3: Understand the Example (1 minute)

Your project has a 3-layer architecture:

```
┌─────────────────────────────────────┐
│  Presentation Layer                 │  user-controller.json
│  (HTTP endpoints, UI components)    │  - HTTP request/response
│                                     │  - Calls user-service
└─────────────────┬───────────────────┘
                  │ depends on
┌─────────────────▼───────────────────┐
│  Application Layer                  │  user-service.json
│  (Business logic, use cases)        │  - CRUD operations
│                                     │  - Calls domain functions
└─────────────────┬───────────────────┘
                  │ depends on
┌─────────────────▼───────────────────┐
│  Domain Layer                       │  user.json
│  (Core entities, business rules)    │  - User entity
│                                     │  - Email validation
└─────────────────────────────────────┘
```

**Dependency Flow:** Presentation → Application → Domain (one direction only!)

---

## Step 4: Validate (30 seconds)

```bash
crucible validate
```

**Expected output:**
```
Validating  architecture...
  3 modules found
Architecture is valid!
```

✅ **Success!** Your architecture is defined and validated.

---

## Step 5: Customize (2 minutes)

### Create Your First Module

Copy an example:
```bash
cp .crucible/modules/user.json .crucible/modules/product.json
```

Edit `product.json`:
```json
{
  "module": "product",
  "version": "1.0.0",
  "layer": "domain",
  "description": "Product domain entity",
  "exports": {
    "Product": {
      "type": "interface",
      "properties": {
        "id": {"type": "string", "required": true},
        "name": {"type": "string", "required": true},
        "price": {"type": "number", "required": true}
      }
    }
  },
  "dependencies": {}
}
```

Add to `.crucible/manifest.json`:
```json
{
  "modules": ["user", "user-service", "user-controller", "product"]
}
```

Validate again:
```bash
crucible validate
```

---

## 🎉 You're Done!

**In 5 minutes, you:**
- ✅ Installed Crucible
- ✅ Created a validated architecture
- ✅ Understood the layer structure
- ✅ Created your first custom module

---

## What's Next?

### Quick Actions (5-10 minutes each)

**Generate TypeScript code:**
```bash
crucible generate --lang typescript --output ./src/generated
```

**Create a service:**
```bash
cp .crucible/modules/user-service.json .crucible/modules/product-service.json
# Edit to add methods like createProduct, getProduct, etc.
# Update dependencies to include "product": "Product"
crucible validate
```

**Create domain events:**
```json
{
  "ProductCreated": {
    "type": "event",
    "payload": {
      "productId": {"type": "string", "required": true},
      "name": {"type": "string", "required": true},
      "createdAt": {"type": "Date", "required": true}
    }
  }
}
```

**Create a trait (behavioral contract):**
```json
{
  "Repository": {
    "type": "trait",
    "methods": {
      "findById": {
        "inputs": [{"name": "id", "type": "string"}],
        "returns": {"type": "object | null"},
        "async": true
      },
      "save": {
        "inputs": [{"name": "entity", "type": "object"}],
        "returns": {"type": "object"},
        "async": true
      }
    }
  }
}
```

**View dependency graph:**
```bash
crucible graph
```

**Use with Claude Code:**
```bash
# Type "/" in Claude Code and look for crucible: commands
/crucible:validate
/crucible:architecture
/crucible:module
```

### Deep Dives (20-30 minutes each)

1. **Master the Schema** → [Schema Reference](./schema-reference.md)
   - Learn all export types (class, function, interface, type, enum, event, trait)
   - Understand method definitions, async support, and type syntax
   - See complete examples including domain events and traits

2. **Avoid Common Mistakes** → [Common Mistakes](./common-mistakes.md)
   - 12 most common errors with fixes
   - Batch fix scripts for migration
   - Error message decoder

3. **Explore Real Example** → [Example Project](./examples/full-stack-app/)
   - 33-module full-stack application
   - See complex patterns in practice
   - Copy patterns for your project

4. **Learn Type System** → [Type System](./type-system.md)
   - Arrays, generics, unions, nullable types
   - Language mappings (TypeScript, Rust, Python, Go)
   - Best practices

5. **Master CLI** → [CLI Reference](./cli-reference.md)
   - All commands with options
   - CI/CD integration
   - Pre-commit hooks

---

## Common First Questions

**Q: Can I use this with an existing codebase?**
A: Yes! Use `crucible init --here` in your project directory. Then create module definitions that match your code structure.

**Q: Do I need to define every file?**
A: No! Define modules (groups of related functionality), not individual files. One module can represent many files.

**Q: What if validation fails?**
A: Check the error message. It will tell you exactly what's wrong and link to documentation. See [Common Mistakes](./common-mistakes.md) for solutions.

**Q: Can I have circular dependencies?**
A: No, Crucible prevents circular dependencies. This is a feature to maintain clean architecture!

**Q: How do I handle multiple exports from one module?**
A: Use comma-separated exports: `"dependencies": {"user-module": "User,CreateUserDTO,UpdateUserDTO"}`

**Q: Can presentation depend on domain directly?**
A: It depends on your rules.json. The default allows it, but you can make it stricter if needed.

---

## Getting Help

**Documentation:**
- [Complete Documentation Index](./README.md)
- [Common Mistakes](./common-mistakes.md) - Start here if you have errors!
- [Schema Reference](./schema-reference.md) - Complete JSON format guide

**Community:**
- [GitHub Issues](https://github.com/anvanster/crucible/issues)
- [GitHub Discussions](https://github.com/anvanster/crucible/discussions)

**Pro Tip:** 90% of questions are answered in [Common Mistakes](./common-mistakes.md). Check there first!

---

## Summary Card

```
┌─────────────────────────────────────────────────┐
│  Crucible 5-Minute Quickstart                   │
├─────────────────────────────────────────────────┤
│  1. Install:    cargo install crucible-cli      │
│  2. Init:       crucible init --name my-app     │
│  3. Explore:    Review example modules           │
│  4. Validate:   crucible validate               │
│  5. Customize:  Copy & edit modules             │
├─────────────────────────────────────────────────┤
│  Next Steps:                                     │
│  • Generate code: crucible generate              │
│  • Read schema reference                        │
│  • Study 33-module example                      │
│  • Set up Claude Code integration               │
└─────────────────────────────────────────────────┘
```

**Time to first validated architecture: 5 minutes**

**Time to first custom module: 7 minutes**

**Time to production-ready architecture: 1-2 hours** (with docs)

---

Happy architecting! 🎯
