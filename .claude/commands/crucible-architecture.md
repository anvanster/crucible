---
name: crucible:architecture
description: Design architecture for a new feature using architecture-first TDD approach
---

You are helping the user design architecture for a new feature using Crucible's architecture-first TDD methodology.

## Context

Crucible follows an **architecture-first Test-Driven Development** approach:
1. Design architecture (module definitions)
2. Write failing tests based on architecture
3. Implement code to make tests pass
4. Validate architecture compliance

This command guides users through step 1: designing the architecture.

## Command Behavior

### Phase 1: Project Analysis & Detection

1. **Check existing project state**:
   - Verify `.crucible/` directory exists (if not, suggest running `/crucible:init` first)
   - Read `manifest.json` to discover existing modules
   - Parse `rules.json` to understand current architecture pattern (layers, dependencies)
   - Identify current layer structure (3-layer, 4-layer, custom)

2. **Analyze requirements**:
   - Parse feature description from user input
   - Read referenced files (PRDs, specs) if `@path` syntax used
   - Extract feature areas and functional requirements
   - Identify required modules and their relationships

### Phase 2: Architecture Design

3. **Design module architecture**:
   - Determine required modules and their purposes
   - Suggest module names (kebab-case convention)
   - Assign appropriate layers (domain, application, infrastructure, presentation)
   - Identify dependencies between new modules
   - Identify dependencies on existing modules
   - Define exports (types, functions, classes, interfaces)
   - Consider TypeScript type system features (generics, unions, arrays, nullable)

4. **Detect architecture conflicts**:
   - Check if new modules require layers not in `rules.json`
   - Example: Infrastructure layer needed but only 3-layer (domain → application → presentation) exists
   - Identify layer dependency conflicts
   - Check for circular dependency risks

### Phase 3: Interactive Confirmation & Updates

5. **Handle existing modules** (if manifest has modules):
   ```
   ⚠️  Found existing Crucible project with 3 modules:
      • user (domain)
      • user-service (application)
      • user-controller (presentation)

   Your new architecture will add 20 healthcare modules.

   Options:
   1. Merge - Keep existing modules and add new ones (23 total)
   2. Replace - Remove existing modules and start fresh (20 total)
   3. Cancel - Manual review required

   Choose [1-3]:
   ```

6. **Update rules.json if needed**:
   ```
   ⚠️  Architecture Pattern Conflict Detected!

   Your new architecture requires an "infrastructure" layer, but your
   current rules.json only supports 3 layers.

   Recommended change to rules.json:

   ADD layer:
   + {"name": "infrastructure", "can_depend_on": ["domain"]}

   UPDATE dependencies:
   ~ {"name": "application", "can_depend_on": ["infrastructure", "domain"]}
   ~ {"name": "presentation", "can_depend_on": ["application", "infrastructure", "domain"]}

   Apply this change? (Y/n): [Y]
   ```

7. **Update manifest.json automatically**:
   - Based on user's merge/replace choice
   - Add new module names to `modules` array
   - Preserve existing modules if merge selected
   - Show confirmation: `✓ Updated .crucible/manifest.json (N modules registered)`

### Phase 4: Module Generation

8. **Generate module definition files**:
   - Create `.crucible/modules/<module-name>.json` for each module
   - Follow Crucible JSON schema
   - Include all exports with proper TypeScript type signatures
   - Declare all dependencies (inter-module and to existing modules)
   - Add metadata (version, layer, language)
   - Show progress: `✓ Domain layer (5 modules)`, `✓ Application layer (8 modules)`, etc.

### Phase 5: Post-Generation Validation

9. **Run validation automatically**:
   - Execute `crucible validate` on all modules
   - Parse validation output for errors and warnings
   - Categorize violations by type (missing exports, type errors, layer violations, etc.)
   - Calculate violation summary statistics

10. **Present validation results**:
    ```
    ❌ Found 78 violations (75 errors, 3 warnings)

    Common issues:
    • 3 type errors: Type 'Blob' not found
    • 75 missing exports: Methods called but not defined in modules
    • 0 layer violations

    💡 Next steps:
    1. Fix type definitions (Blob → Buffer or add Blob type)
    2. Add missing method exports to infrastructure modules
    3. Run: /crucible:validate to check progress

    Would you like help fixing these issues? (Y/n):
    ```

11. **Offer guided fixes** (if user accepts):
    - Identify fixable violations automatically
    - Suggest specific changes to module definitions
    - Ask permission before applying fixes
    - Re-validate after fixes applied

### Phase 6: TDD Guidance

12. **Provide architecture-first TDD guidance**:
    - Suggest test file locations based on modules
    - List specific tests to write for each export
    - Provide example test structure with actual type signatures
    - Recommend test commands (npm test, cargo test, etc.)
    - Guide towards implementation workflow

## Automatic Project Updates

This command automatically handles:

✓ **Manifest updates**: Adds new modules to `.crucible/manifest.json`
✓ **Layer detection**: Identifies required architecture layers from module designs
✓ **Rules.json updates**: Adds missing layers to architecture rules when needed
✓ **Conflict resolution**: Prompts for handling existing modules (merge/replace)
✓ **Post-validation**: Runs validation and categorizes violations with actionable fixes
✓ **TDD guidance**: Provides test structure and implementation workflow

## Interactive Prompts

You will be asked to:
- **Choose merge strategy** when existing modules are found (merge, replace, or cancel)
- **Approve rules.json updates** when new layers are required (e.g., adding infrastructure layer)
- **Accept or modify** suggested module names, layers, dependencies, and exports
- **Review guided fixes** for validation violations (optional automation)

## Output Format

### Multi-Module Architecture Flow

When designing complex architecture with multiple modules:

```
🏗️ Designing architecture for: [Feature Name]

📋 Phase 1: Project Analysis
   ✓ Found existing Crucible project
   ✓ Current architecture: [N]-layer ([layer names and dependencies])
   ✓ Existing modules: [N] ([list module names with layers])
   ✓ Analyzed requirements from [source: user input, @file, PRD, etc.]

📋 Phase 2: Architecture Design
   ✓ Identified [N] feature areas
   ✓ Proposed [N] modules across [N] layers

   Layer distribution:
   • [Layer 1]: [N] modules ([list module names])
   • [Layer 2]: [N] modules ([list module names])
   • [Layer 3]: [N] modules ([list module names])
   ... (for each layer with modules)

[IF new layers needed that don't exist in rules.json]
⚠️  Architecture Pattern Conflict Detected!

   Your new architecture requires "[layer-name]" layer, but your
   current rules.json only supports [N] layers ([list current layers]).

   Recommended change to rules.json:

   ADD layer:
   + {"name": "[layer-name]", "can_depend_on": ["[allowed-dependencies]"]}

   UPDATE dependencies:
   ~ [For each affected layer, show dependency updates]

   Apply this change? (Y/n): [wait for input]

   [If Y] ✓ Updated .crucible/rules.json
   [If n] ⚠️  Proceeding without rules.json update - may cause validation errors

📋 Phase 3: Module Manifest Update

   Current manifest.json modules: [list current module names]
   New modules to add: [N] modules

   Options:
   1. Merge - Keep existing modules and add new ones ([total] total)
   2. Replace - Remove existing modules and start fresh ([N] modules)
   3. Cancel - Manual review required

   Choose [1-3]: [wait for input]

   [If 1 or 2] ✓ Updated .crucible/manifest.json ([N] modules registered)
   [If 3] ⚠️  Cancelled - no changes made to manifest

📋 Phase 4: Generate Module Definitions

   Generating [N] module definition files...

   [Group by layer and list modules]
   ✓ [Layer] layer ([N] modules)
     • [module-name].json
     ... (for each module in layer)

   💾 Created [N] files in .crucible/modules/

📋 Phase 5: Validation

   Running: crucible validate

   [IF validation passes]
   ✅ Validation passed: 0 violations
   ✓ No circular dependencies
   ✓ Layer boundaries respected
   ✓ All type references valid

   [IF validation fails]
   ❌ Found [N] violations ([N] errors, [N] warnings)

   [Categorize violations by type and show top issues]
   Common issues:
   • [N] type errors: [Brief description, e.g., "Type 'X' not found in [modules]"]
   • [N] missing exports: [Brief description, e.g., "Methods called but not defined"]
     [Show 3-5 specific examples with module names]
     ... ([N] more)
   • [N] layer violations: [Brief description if any]
   • [N] circular dependencies: [Brief description if any]

   💡 Next steps:
   1. [Specific fix for issue type 1, e.g., "Fix type definitions:"]
      - [Concrete action, e.g., "Change X → Y in [module] module"]
      - [Alternative action, e.g., "Or add X type to [location]"]
   2. [Specific fix for issue type 2, e.g., "Add missing method exports:"]
      - [Concrete action with module names and method signatures]
   3. Run: /crucible:validate to check progress

   Would you like help fixing these issues? (Y/n): [wait for input]

   [If Y] [Show guided fix workflow - see Guided Fixes section below]
   [If n] ℹ️  You can run /crucible:validate anytime to check progress

📝 Architecture-First TDD Guidance

   ✅ Architecture phase complete!

   Next: Write tests BEFORE implementing (RED → GREEN → REFACTOR)

   [Generate test structure based on modules created]
   Recommended test structure:

   tests/
   [For each layer with modules, create directory]
   ├── [layer-name]/
   │   ├── [module-1].test.[ext]  # Test [brief description]
   │   ├── [module-2].test.[ext]  # Test [brief description]
   │   └── ...
   └── ...

   Start with [lowest layer - typically domain] layer tests:

   [test command for language] tests/[layer]/[module].test.[ext]  # Should FAIL (not implemented yet)

   Then implement [layer] layer to make tests pass.
   Repeat for [next layers in dependency order].

   🎯 Architecture validation:
      Run /crucible:validate frequently to ensure compliance!
```

### Simple Single-Module Flow

When designing a simple architecture with one module:

```
🏗️ Designing architecture for: [Feature Name]

📋 Understanding requirements...

❓ What layer should this module belong to?
   1. domain (business logic, core types)
   2. application (use cases, services, orchestration)
   3. infrastructure (external systems, databases, APIs)
   [4. presentation (UI, controllers) - if exists in project]

   Based on "[feature description]", I suggest: [suggested-layer]
   Press Enter to accept, or type 1-[N]: [wait for input]

✓ Layer: [chosen-layer]

📝 Module name suggestion: [suggested-name]
   (from "[feature description]")
   Press Enter to accept, or type custom name: [wait for input]

✓ Module: [chosen-name]

🔍 Analyzing existing modules...
   Found [N] modules in project

🔗 Suggested dependencies:
   [For each suggested dependency]
   • [module-name] ([layer]) - for [reason/types it provides]

   Add more dependencies? (comma-separated, or Enter to continue): [wait for input]

✓ Dependencies: [list chosen dependencies]

📦 Defining exports...

For [feature description], I suggest these exports:

[For each export, with appropriate formatting]
1. [ExportName] ([export-type])
   [If class] Methods:
   - [methodName]([params with types]) → [return-type]
   [If type/interface] Properties:
   - [propertyName]: [type]

[Repeat for all exports]

Looks good? (Y/n/edit): [wait for input]

✓ Exports defined

💾 Generating module definition...
✓ Created: .crucible/modules/[module-name].json

🔍 Validating architecture...
[IF passes]
✓ Validation passed: 0 violations
✓ No circular dependencies
✓ Layer boundaries respected
✓ All type references valid

[IF fails - show same format as multi-module Phase 5]

📊 Module summary:
   • Name: [module-name]
   • Layer: [layer-name]
   • Dependencies: [N] ([list module names])
   • Exports: [N] ([breakdown by type: N classes, N types, N interfaces, etc.])
   • Language: [language]

📝 Next steps (TDD approach):

1. **Write failing tests** (RED phase):
   Create: [test file path based on language and conventions]

   [Generate example test structure with actual export names and types from module]
   Tests should cover:
   [For each export]
   - Test [ExportName]: [describe what to test based on export type]

2. **Run tests** (should FAIL):
   [test command for language]
   # All tests should fail - implementation doesn't exist yet

3. **Implement [module-name]** (GREEN phase):
   Create: [source file path based on language and conventions]

   [Generate implementation scaffold with actual imports and types from module definition]
   Implement:
   [For each export]
   - [ExportName]: [brief implementation guidance]

4. **Run tests again**:
   [test command for language]
   # Tests should PASS

5. **Validate architecture**:
   /crucible:validate
   # Should show 0 violations

6. **Refactor if needed** (REFACTOR phase):
   - Improve code quality
   - Add error handling
   - Optimize performance
   - Re-run tests to ensure still passing

✅ Architecture-first TDD workflow complete!
```

### Guided Fixes Workflow

When user accepts help fixing validation violations:

```
🔧 Guided Fixes for [N] Violations

Analyzing violations and suggesting fixes...

[Group violations by type and fixability]

═══ Fixable Automatically ([N] violations) ═══

1. Type Reference Errors ([N] violations)

   Issue: Type '[TypeName]' not found in [N] modules

   Suggested fix:
   [For each affected module]
   • In [module-name].json:
     - Change type '[TypeName]' → '[SuggestedType]'
     [OR]
     - Add export for type '[TypeName]' to [dependency-module]

   Apply these fixes? (Y/n/skip): [wait for input]

2. Missing Export Errors ([N] violations)

   Issue: Methods called but not exported in [N] modules

   Suggested fix:
   [For each affected module and method]
   • In [module-name].json, add to exports:
     ```json
     {
       "name": "[methodName]",
       "type": "method",
       "parameters": [/* inferred from usage */],
       "returns": "/* inferred type */"
     }
     ```

   Apply these fixes? (Y/n/skip): [wait for input]

═══ Needs Manual Review ([N] violations) ═══

3. [Violation Type] ([N] violations)

   Issue: [Description]

   These require manual review because: [reason]

   Recommendations:
   - [Specific recommendation 1]
   - [Specific recommendation 2]

   [Show affected modules and locations]

[After applying fixes]
✓ Applied [N] automatic fixes
ℹ️  [N] violations require manual review

Running validation again...

[Show updated validation results]
```

## Flags

**`--merge`**
Automatically merge with existing modules (no prompt):
```bash
/crucible:architecture "Feature X" --merge
```

**`--replace`**
Automatically replace existing modules (no prompt):
```bash
/crucible:architecture "Feature X" --replace
```

**`--no-validate`**
Skip post-generation validation:
```bash
/crucible:architecture "Feature X" --no-validate
```

**`--layer <layer>`**
Pre-specify the layer for single module:
```bash
/crucible:architecture "Database connector" --layer infrastructure
```

**`--layers <layers>`**
Explicitly set required layers for multi-module architecture:
```bash
/crucible:architecture @prd.md --layers domain,application,infrastructure,presentation
```

**`--language <lang>`**
Override detected language (typescript, rust, python, go):
```bash
/crucible:architecture "Config loader" --language rust
```

**`--template <type>`**
Use template for module generation (service, repository, controller, etc.):
```bash
/crucible:architecture "User service" --template service
```

## Implementation Notes

- **Read manifest.json and rules.json first** to understand existing project structure
- **Detect layer conflicts** by comparing proposed modules against rules.json layers
- **Always ask for confirmation** before modifying manifest.json or rules.json
- **Run crucible validate** after generating modules to catch issues immediately
- **Parse validation output** and categorize violations by type for actionable fixes
- **Generate language-appropriate** test structures and implementation scaffolds
- **Use AskUserQuestion tool** for interactive prompts (layer selection, merge strategy, fix approvals)
- **Show progress phases** clearly: Analysis → Design → Confirmation → Generation → Validation → TDD
- **Provide file:line references** in validation output for easy navigation
- **Link to documentation** for complex issues that need deeper understanding

🎯 Key benefits:
   • Architecture designed upfront (prevents rework)
   • Tests define expected behavior (living documentation)
   • Implementation guided by tests (less bugs)
   • Validation ensures compliance (no drift)
```

## Flags

**`--layer <domain|application|infrastructure>`**
Skip layer selection prompt:
```bash
/crucible:architecture "Payment processing" --layer application
```

**`--depends <modules>`**
Pre-specify dependencies:
```bash
/crucible:architecture "User repository" --depends user,database
```

**`--template <type>`**
Use predefined template:
```bash
/crucible:architecture "Order service" --template service
```

Templates:
- `service` - Application service class
- `repository` - Data access repository
- `controller` - API controller/handler
- `entity` - Domain entity with business logic
- `value-object` - Immutable value object

**`--language <typescript|rust|python|go>`**
Target programming language:
```bash
/crucible:architecture "Config loader" --language rust
```

**`--non-interactive`**
Skip all prompts (use defaults):
```bash
/crucible:architecture "Cache service" --layer infrastructure --non-interactive
```

## Examples

### Basic usage:
```bash
/crucible:architecture "User authentication with JWT tokens"
```

### With layer specified:
```bash
/crucible:architecture "Payment processing service" --layer application
```

### With dependencies:
```bash
/crucible:architecture "User repository" --depends user,database --layer infrastructure
```

### Using template:
```bash
/crucible:architecture "Order service" --template service --depends order,payment
```

### Rust project:
```bash
/crucible:architecture "Config loader" --language rust --layer infrastructure
```

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Invalid layer:
```
❌ Error: Invalid layer 'xyz'

Valid layers:
   • domain - Business logic, core types, entities
   • application - Use cases, services, orchestration
   • infrastructure - External systems, databases, APIs

💡 Learn more:
   Domain-Driven Design (DDD) layer architecture
   Clean Architecture principles
```

### Module already exists:
```
⚠️ Warning: Module 'auth' already exists

Options:
   1. Update existing module (recommended)
   2. Create with different name
   3. Overwrite (destructive)

Choose option [1-3]:
```

### Circular dependency detected:
```
❌ Error: Circular dependency detected

   auth → user → user-service → auth

This creates a circular dependency chain. Consider:
   • Extract shared types to separate module
   • Use dependency injection
   • Restructure module responsibilities

Would you like help refactoring? (Y/n):
```

### Layer violation:
```
⚠️ Warning: Potential layer violation

You're creating a 'domain' module that depends on 'database' (infrastructure).

Domain modules should not depend on infrastructure.

Suggestions:
   1. Change layer to 'application' or 'infrastructure'
   2. Use repository pattern (inject database dependency)
   3. Move shared types to domain, keep implementation in infrastructure

Proceed anyway? (y/N):
```

## Architecture Best Practices

The command should guide users towards good architecture:

1. **Layer boundaries**:
   - Domain → No external dependencies
   - Application → Can depend on domain
   - Infrastructure → Can depend on domain, application

2. **Module naming**:
   - Kebab-case: `user-service`, `payment-gateway`
   - Descriptive: reflects module responsibility
   - Consistent: follow project conventions

3. **Dependency management**:
   - Minimize dependencies
   - Avoid circular dependencies
   - Use dependency injection

4. **Type design**:
   - Use TypeScript features: generics, unions, types
   - Proper nullability: `Type | null` vs `Type`
   - Arrays: `Type[]` for collections
   - Generics: `Promise<T>`, `Partial<T>` for utilities

5. **Export organization**:
   - Group related exports
   - Clear naming conventions
   - Proper visibility (public vs internal)

## Integration with Other Commands

After designing architecture:
- Run `/crucible:validate` to verify
- Use `/crucible:module <name> --update` to modify
- Run `/crucible:review` for architectural analysis
- Use `/crucible:sync` if implementing first and syncing back

## Implementation Notes

- Parse feature description intelligently (NLP-style)
- Suggest sensible defaults based on naming patterns
- Detect existing modules and suggest dependencies
- Validate as you go (prevent invalid architectures)
- Provide rich examples in TDD guidance
- Generate TypeScript-aware type definitions
- Support multiple programming languages
- Be opinionated but allow overrides
- Educate about architecture patterns
- Link to documentation for complex topics
