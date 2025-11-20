# Full-Stack Application Example

Complete real-world example of a Crucible architecture with 34 modules across 4 layers.

## Overview

This example demonstrates a full-stack TypeScript application using a layered architecture pattern. The project is a content management system with analysis capabilities.

## Architecture

```
Presentation Layer (UI Components)
    ↓
Application Layer (Services & Use Cases)
    ↓
Infrastructure Layer (External Systems)
    ↓
Domain Layer (Business Entities)
```

### Layer Dependency Rules

- **Presentation** can depend on: Presentation, Application, Infrastructure, Domain
- **Application** can depend on: Application, Infrastructure, Domain
- **Infrastructure** can depend on: Infrastructure, Domain
- **Domain** can depend on: Domain (only other domain entities)

## Module Count by Layer

- **Domain**: 8 modules (entities and value objects)
- **Infrastructure**: 6 modules (repositories, external clients, utilities)
- **Application**: 6 modules (services and use cases)
- **Presentation**: 4 modules (UI components)
- **CLI**: 10 modules (command-line interface)

**Total**: 34 modules

## Module Categories

### Domain Layer (`domain`)

Core business entities and value objects:

- `project-config` - Main configuration entity
- `spec-info` - Specification metadata
- `chapter` - Content chapter entity
- `character` - Character entity
- `plot-thread` - Story arc entity
- `timeline-event` - Event tracking
- `task` - Task entity
- `consistency-issue` - Issue tracking

### Infrastructure Layer (`infrastructure`)

External system integrations:

- `claude-client` - AI service client
- `prompt-manager` - Template management
- `git-repository` - Version control interface
- `file-storage` - File system operations
- `template-engine` - Template processing
- `consistency-checker` - Validation engine

### Application Layer (`application`)

Business logic and use cases:

- `project-service` - Project management
- `spec-service` - Specification handling
- `constitution-service` - Configuration management
- `plan-service` - Planning operations
- `task-service` - Task management
- `analysis-service` - Analysis orchestration

### Presentation Layer (`presentation`)

User interface components:

- `project-browser-ui` - Project browsing component
- `constitution-wizard-ui` - Configuration wizard
- `spec-editor-ui` - Specification editor
- `plan-view-ui` - Planning interface

### CLI Layer (`application`)

Command-line interface:

- `init-command` - Project initialization
- `constitution-command` - Configuration management
- `specify-command` - Specification creation
- `clarify-command` - Clarification workflows
- `plan-command` - Planning operations
- `tasks-command` - Task management
- `check-command` - Consistency checking
- `analyze-command` - Analysis operations
- `pacing-analyzer` - Pacing analysis utility

## Key Patterns Demonstrated

### 1. Domain Entity

See: `modules/chapter.json`

Simple domain entity with properties:
```json
{
  "module": "chapter",
  "layer": "domain",
  "exports": {
    "Chapter": {
      "type": "interface",
      "properties": {...}
    }
  }
}
```

### 2. Application Service

See: `modules/analysis-service.json`

Service with methods and complex dependencies:
```json
{
  "module": "analysis-service",
  "layer": "application",
  "exports": {
    "AnalysisService": {
      "type": "class",
      "methods": {...}
    }
  },
  "dependencies": {
    "consistency-checker": "ConsistencyChecker",
    "chapter": "Chapter"
  }
}
```

### 3. Infrastructure Component

See: `modules/claude-client.json`

External service client:
```json
{
  "module": "claude-client",
  "layer": "infrastructure",
  "exports": {
    "ClaudeClient": {
      "type": "class",
      "methods": {...}
    }
  }
}
```

### 4. React Component

See: `modules/constitution-wizard-ui.json`

UI component with props:
```json
{
  "module": "constitution-wizard-ui",
  "layer": "presentation",
  "exports": {
    "ConstitutionWizard": {
      "type": "function",
      "inputs": [...],
      "returns": {"type": "JSX.Element"}
    }
  }
}
```

### 5. Multiple Exports

See: `modules/consistency-issue.json`

Module exporting both interface and enum:
```json
{
  "exports": {
    "ConsistencyIssue": {"type": "interface"},
    "IssueSeverity": {"type": "enum"}
  }
}
```

### 6. Complex Dependencies

See: `modules/analysis-service.json`

Multiple exports from same module:
```json
{
  "dependencies": {
    "consistency-issue": "ConsistencyIssue,ConsistencyReport",
    "character": "Character,CharacterAppearance"
  }
}
```

## Validation

Run validation to verify the architecture:

```bash
crucible validate --path .crucible
```

Expected output:
```
Validating architecture...
  34 modules found
Architecture is valid!
```

## Structure

```
.crucible/
├── manifest.json           # Project manifest (34 modules)
├── rules.json             # Architecture rules (4-layer)
└── modules/               # Module definitions
    ├── project-config.json
    ├── chapter.json
    ├── analysis-service.json
    ├── claude-client.json
    ├── constitution-wizard-ui.json
    └── ... (29 more modules)
```

## Learning Path

1. **Start Simple** - Review `chapter.json` (simple domain entity)
2. **Add Behavior** - Check `project-service.json` (service with methods)
3. **Complex Dependencies** - Study `analysis-service.json` (multiple dependencies)
4. **UI Components** - Examine `constitution-wizard-ui.json` (React patterns)
5. **Full Architecture** - Review `manifest.json` and `rules.json`

## Common Patterns

### Domain Entity Pattern

```json
{
  "module": "entity-name",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "EntityName": {
      "type": "interface",
      "properties": {
        "id": {"type": "string"},
        "name": {"type": "string"}
      }
    }
  },
  "dependencies": {}
}
```

### Service Pattern

```json
{
  "module": "entity-service",
  "version": "1.0.0",
  "layer": "application",
  "exports": {
    "EntityService": {
      "type": "class",
      "methods": {
        "create": {
          "inputs": [{"name": "data", "type": "CreateDTO"}],
          "returns": {"type": "Promise<Entity>"}
        }
      }
    }
  },
  "dependencies": {
    "entity": "Entity"
  }
}
```

### Repository Pattern

```json
{
  "module": "entity-repository",
  "version": "1.0.0",
  "layer": "infrastructure",
  "exports": {
    "EntityRepository": {
      "type": "class",
      "methods": {
        "save": {
          "inputs": [{"name": "entity", "type": "Entity"}],
          "returns": {"type": "Promise<void>"}
        }
      }
    }
  },
  "dependencies": {
    "entity": "Entity"
  }
}
```

## See Also

- [Schema Reference](../../schema-reference.md)
- [Common Mistakes](../../common-mistakes.md)
- [Type System](../../type-system.md)
