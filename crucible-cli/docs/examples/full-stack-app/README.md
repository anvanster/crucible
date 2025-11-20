# Full-Stack Application Example

**33-module production-quality architecture** demonstrating Crucible best practices.

This is a real-world application architecture (Loom - an AI-assisted writing tool) adapted as a comprehensive Crucible example.

---

## 📊 Architecture Overview

### 4-Layer Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│  Presentation Layer (4 modules)                                   │
│  React components, UI wizards, views                              │
│                                                                   │
│  • project-browser-ui      • constitution-wizard-ui              │
│  • spec-editor-ui          • plan-view-ui                        │
└───────────────────────────┬──────────────────────────────────────┘
                            │ depends on
┌───────────────────────────▼──────────────────────────────────────┐
│  Application Layer (12 modules)                                   │
│  Commands (CLI), Services (business logic)                        │
│                                                                   │
│  Commands:                    Services:                           │
│  • init-command               • project-service                   │
│  • constitution-command       • spec-service                      │
│  • specify-command            • constitution-service              │
│  • clarify-command            • plan-service                      │
│  • plan-command               • task-service                      │
│  • tasks-command              • analysis-service                  │
│  • check-command                                                  │
│  • analyze-command                                                │
└───────────────────────────┬──────────────────────────────────────┘
                            │ depends on
┌───────────────────────────▼──────────────────────────────────────┐
│  Infrastructure Layer (8 modules)                                 │
│  External integrations, algorithms, storage                       │
│                                                                   │
│  • claude-client           • prompt-manager                       │
│  • git-repository          • file-storage                         │
│  • template-engine         • consistency-checker                  │
│  • pacing-analyzer                                                │
└───────────────────────────┬──────────────────────────────────────┘
                            │ depends on
┌───────────────────────────▼──────────────────────────────────────┐
│  Domain Layer (8 modules)                                         │
│  Core entities, business rules, no external dependencies          │
│                                                                   │
│  • project-config          • spec-info                            │
│  • chapter                 • character                            │
│  • plot-thread             • timeline-event                       │
│  • task                    • consistency-issue                    │
└──────────────────────────────────────────────────────────────────┘
```

**Total:** 33 modules | **Language:** TypeScript | **Pattern:** Relaxed Layered Architecture

### Detailed Dependency Flow Example

Here's how dependencies flow through the layers in a typical operation:

```
User Action: "Create a new chapter specification"

┌─────────────────────────────────────────────────────────────────┐
│ PRESENTATION                                                     │
│ spec-editor-ui.tsx                                              │
│   └─ User clicks "Save"                                         │
│      └─ Calls SpecService.createSpec(data)                      │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│ APPLICATION                                                      │
│ spec-service.ts                                                 │
│   └─ createSpec(data: CreateSpecDTO): Promise<SpecInfo>        │
│      ├─ Validates using SpecInfo domain rules                   │
│      ├─ Calls FileStorage.write()                               │
│      └─ Calls GitRepository.commit()                            │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│ INFRASTRUCTURE                                                   │
│ file-storage.ts           git-repository.ts                     │
│   └─ write(path, data)      └─ commit(message)                  │
│      └─ fs.writeFile()         └─ git commit                    │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│ DOMAIN                                                           │
│ spec-info.ts                                                    │
│   └─ interface SpecInfo {                                       │
│        id: string;          ◄────── Used throughout            │
│        type: SpecType;              all layers                  │
│        content: string;                                         │
│      }                                                           │
└─────────────────────────────────────────────────────────────────┘

Key Architectural Principles:
  ✓ Domain layer has NO dependencies (pure business logic)
  ✓ Infrastructure depends ONLY on Domain
  ✓ Application coordinates Infrastructure and Domain
  ✓ Presentation depends on Application (and can skip to Domain)
  ✓ Relaxed layering allows services to call other services
```

### Allowed vs Forbidden Dependencies

```
┌──────────────────────────┐  ┌──────────────────────────┐
│   ALLOWED ✓              │  │   FORBIDDEN ✗            │
├──────────────────────────┤  ├──────────────────────────┤
│                          │  │                          │
│  Presentation            │  │  Domain                  │
│      ↓                   │  │      ↓                   │
│  Application             │  │  Infrastructure  ✗✗✗    │
│      ↓                   │  │                          │
│  Infrastructure          │  │  Domain                  │
│      ↓                   │  │      ↓                   │
│  Domain                  │  │  Application     ✗✗✗    │
│                          │  │                          │
│  Application             │  │  Infrastructure          │
│      ↓                   │  │      ↓                   │
│  Application  (same)     │  │  Presentation    ✗✗✗    │
│  (Relaxed Layering)      │  │                          │
│                          │  │                          │
└──────────────────────────┘  └──────────────────────────┘

Relaxed Layering Rule:
  Each layer can depend on:
    - Itself (intra-layer dependencies)
    - Any layer below it

  But NEVER on layers above it!
```

---

## 🎯 What This Example Demonstrates

### Core Patterns

✅ **4-Layer Architecture** - Domain → Infrastructure → Application → Presentation
✅ **Relaxed Layering** - Allows intra-layer dependencies (e.g., services can call other services)
✅ **Domain-Driven Design** - Rich domain entities with value objects
✅ **Command Pattern** - CLI commands in application layer
✅ **Service Pattern** - Business logic encapsulation
✅ **Repository Pattern** - Data access abstraction
✅ **React Components as Functions** - Modern React patterns
✅ **Complex Dependencies** - Multiple exports from same module

### Advanced Features

✅ **Generic Types** - `Promise<T>`, `Array<T>`, `Map<K,V>`
✅ **Union Types** - `User | null`, `Success | Error`
✅ **Array Types** - `Chapter[]`, `string[]`, `number[]`
✅ **Nullable Types** - Optional properties with `| null`
✅ **Cross-Module Dependencies** - Services depend on repositories and domain entities
✅ **Method Calls** - Tracked across modules for validation

---

## 📁 Project Structure

```
.crucible/
├── manifest.json              # 33 modules, strict validation
├── rules.json                 # 4-layer relaxed architecture
└── modules/
    ├── # Domain Layer (8 modules)
    ├── project-config.json    # Core project configuration entity
    ├── spec-info.json         # Specification metadata
    ├── chapter.json           # Chapter entity with metadata
    ├── character.json         # Character with appearances
    ├── plot-thread.json       # Plot thread tracking
    ├── timeline-event.json    # Timeline events
    ├── task.json              # Task entity
    ├── consistency-issue.json # Consistency tracking
    │
    ├── # Infrastructure Layer (8 modules)
    ├── claude-client.json     # AI integration (Claude API)
    ├── prompt-manager.json    # Prompt template management
    ├── git-repository.json    # Git operations
    ├── file-storage.json      # File system operations
    ├── template-engine.json   # Template processing
    ├── consistency-checker.json # Consistency validation
    ├── pacing-analyzer.json   # Story pacing analysis
    │
    ├── # Application Layer (12 modules)
    ├── init-command.json      # Initialize project
    ├── constitution-command.json # Define project rules
    ├── specify-command.json   # Create specifications
    ├── clarify-command.json   # Clarify specifications
    ├── plan-command.json      # Generate writing plan
    ├── tasks-command.json     # List tasks
    ├── check-command.json     # Check consistency
    ├── analyze-command.json   # Analyze story
    ├── project-service.json   # Project management logic
    ├── spec-service.json      # Specification logic
    ├── constitution-service.json # Constitution logic
    ├── plan-service.json      # Planning logic
    ├── task-service.json      # Task management logic
    ├── analysis-service.json  # Analysis logic
    │
    └── # Presentation Layer (4 modules)
        ├── project-browser-ui.json    # Project browsing interface
        ├── constitution-wizard-ui.json # Constitution setup wizard
        ├── spec-editor-ui.json        # Specification editor
        └── plan-view-ui.json          # Plan visualization
```

---

## 🗺️ How to Explore This Example

### Quick Tour (10 minutes)

Follow this sequence to understand the architecture:

#### 1. **Start with Domain** (2 min)

```bash
# Read the core entity
cat .crucible/modules/project-config.json
```

**What to notice:**
- Simple interface with properties
- No dependencies (domain is independent)
- Enum types for ProjectType
- Nested objects (ProjectInfo, ProjectSettings)

#### 2. **See Infrastructure** (2 min)

```bash
# Check external integration
cat .crucible/modules/claude-client.json
```

**What to notice:**
- Class with methods
- Dependencies on domain entities
- Promise return types
- Method calls to external service

#### 3. **Review Application Services** (3 min)

```bash
# Complex service example
cat .crucible/modules/analysis-service.json
```

**What to notice:**
- Multiple interface exports (PacingReport, CharacterAnalysis)
- Complex dependencies (6 different modules!)
- Multiple exports from same module: `"character": "Character,CharacterAppearance"`
- Array return types: `Chapter[]`, `number[]`
- Calls to infrastructure and domain

#### 4. **Examine UI Components** (3 min)

```bash
# React component example
cat .crucible/modules/constitution-wizard-ui.json
```

**What to notice:**
- React.FC<Props> pattern
- Props interface
- Dependencies on application services
- React-specific types (JSX.Element, React.ReactNode)

### Deep Dive (30 minutes)

Choose a vertical slice and follow the complete flow:

#### Example: "Create a New Specification" Flow

**User Action → UI → Service → Repository → Domain**

1. **UI Layer**: `spec-editor-ui.json`
   - User interacts with editor
   - Calls `SpecService.createSpec()`

2. **Application Layer**: `spec-service.json`
   - Validates input
   - Coordinates with domain
   - Calls `FileStorage.write()` and `GitRepository.commit()`

3. **Infrastructure Layer**: `file-storage.json`, `git-repository.json`
   - Persists specification to file
   - Commits to git

4. **Domain Layer**: `spec-info.json`
   - Core SpecInfo entity
   - Business rules

**Trace this flow:**
```bash
# Follow the dependency chain
grep -l "spec-service" .crucible/modules/*.json
grep -l "file-storage" .crucible/modules/*.json
grep -l "spec-info" .crucible/modules/*.json
```

---

## 📖 Learning Objectives

### For Beginners

**Start with these files to learn:**

1. **Domain entities**: `chapter.json`, `character.json`
   - Learn interface definitions
   - See property types and required fields
   - Understand enums

2. **Simple services**: `task-service.json`
   - Learn class exports
   - See method definitions
   - Understand inputs and returns

3. **Dependencies**: `analysis-service.json`
   - Learn how to declare dependencies
   - See multiple exports from same module
   - Understand cross-layer dependencies

### For Intermediate Users

**Study these patterns:**

1. **Complex type patterns**: `analysis-service.json`
   - Generic types: `Promise<T>`
   - Array types: `Chapter[]`, `ConsistencyIssue[]`
   - Union types: `Success | Error`
   - Multiple interface exports

2. **Service coordination**: `plan-service.json`
   - Multiple method calls
   - Dependency orchestration
   - Error handling patterns

3. **React patterns**: `constitution-wizard-ui.json`
   - Component props
   - State management types
   - Event handlers

### For Advanced Users

**Explore these designs:**

1. **Architectural patterns**:
   - Command pattern in CLI modules
   - Repository pattern in infrastructure
   - Service pattern in application layer
   - Component pattern in presentation

2. **Relaxed layering** (`rules.json`):
   ```json
   {"name": "application", "can_depend_on": ["application", "infrastructure", "domain"]}
   ```
   This allows services to call other services (intra-layer dependencies).

3. **Complex dependency graphs**:
   - See how `analysis-service` coordinates 6 different modules
   - Understand the trade-offs

---

## 🔍 Validation

This example validates successfully:

```bash
crucible validate --path .crucible
```

**Expected output:**
```
Validating  architecture...
  33 modules found
Architecture is valid!
```

**What's being validated:**
- ✅ All types exist and are imported correctly
- ✅ Layer boundaries are respected (no domain → application dependencies)
- ✅ No circular dependencies
- ✅ Method calls target existing exports
- ✅ Dependencies are declared for all used modules

---

## 🎨 Design Decisions

### Why Relaxed Layering?

Traditional strict layering would prevent services from calling other services:
```
application layer can only depend on [infrastructure, domain]
```

This example uses relaxed layering:
```
application layer can depend on [application, infrastructure, domain]
```

**Benefits:**
- Services can coordinate (e.g., `plan-service` calls `spec-service`)
- More natural for complex business logic
- Reduces need for orchestrator services

**Trade-off:**
- Must be careful to avoid circular dependencies (Crucible still prevents these!)

### Why So Many Modules?

This represents a real production application with:
- **8 CLI commands** - Each command is a module
- **6 services** - Each service encapsulates business logic
- **8 domain entities** - Core business concepts
- **8 infrastructure components** - External integrations
- **4 UI components** - React interfaces

In practice, you might start with 5-10 modules and grow organically.

### React Components as Functions

Modern React uses function components:
```json
"ConstitutionWizardUI": {
  "type": "function",
  "inputs": [{"name": "props", "type": "ConstitutionWizardProps"}],
  "returns": {"type": "JSX.Element"}
}
```

Not class components.

---

## 💡 Common Patterns to Copy

### Pattern 1: Domain Entity

```json
{
  "module": "my-entity",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "MyEntity": {
      "type": "interface",
      "properties": {
        "id": {"type": "string", "required": true},
        "name": {"type": "string", "required": true}
      }
    }
  },
  "dependencies": {}
}
```

### Pattern 2: Service with Repository

```json
{
  "module": "my-service",
  "version": "1.0.0",
  "layer": "application",
  "exports": {
    "MyService": {
      "type": "class",
      "methods": {
        "create": {
          "inputs": [{"name": "data", "type": "CreateDTO"}],
          "returns": {"type": "Promise<MyEntity>"},
          "calls": ["my-repository.MyRepository.save"]
        }
      }
    }
  },
  "dependencies": {
    "my-entity": "MyEntity",
    "my-repository": "MyRepository"
  }
}
```

### Pattern 3: Multiple Exports from Dependency

```json
{
  "dependencies": {
    "character": "Character,CharacterAppearance,CharacterRelationship"
  }
}
```

### Pattern 4: React Component

```json
{
  "module": "my-component-ui",
  "version": "1.0.0",
  "layer": "presentation",
  "exports": {
    "MyComponentUI": {
      "type": "function",
      "inputs": [{"name": "props", "type": "MyComponentProps"}],
      "returns": {"type": "JSX.Element"}
    },
    "MyComponentProps": {
      "type": "interface",
      "properties": {
        "title": {"type": "string", "required": true},
        "onSave": {"type": "(data: MyData) => void", "required": true}
      }
    }
  },
  "dependencies": {
    "my-service": "MyService"
  }
}
```

---

## 📚 Related Documentation

- **[5-Minute Quickstart](../../QUICKSTART.md)** - Get started fast
- **[Schema Reference](../../schema-reference.md)** - Complete JSON format guide
- **[Type System](../../type-system.md)** - All type patterns
- **[Common Mistakes](../../common-mistakes.md)** - Error solutions
- **[CLI Reference](../../cli-reference.md)** - Command documentation

---

## 🎯 Next Steps

After exploring this example:

1. **Copy patterns** you like into your own project
2. **Simplify for your needs** - You probably don't need 33 modules!
3. **Validate frequently** - `crucible validate` as you build
4. **Start small** - Begin with 5-10 modules, grow organically
5. **Use Claude Code** - Try `/crucible:architecture` to design new features

---

## 📝 Notes

- This example uses **TypeScript** but patterns apply to **Rust, Python, Go, Java**
- The **Loom** project is a real AI-assisted writing tool
- Architecture demonstrates **production-quality** patterns
- All 33 modules **validate successfully**
- Designed for **learning and reference**, not necessarily the perfect architecture for every project

---

**Questions?** See [Common Mistakes](../../common-mistakes.md) or [Schema Reference](../../schema-reference.md).

**Ready to build?** Start with the [5-Minute Quickstart](../../QUICKSTART.md)!
