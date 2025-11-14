# Crucible Branding Guide

## Name

**Crucible** - A vessel for refining and testing under intense conditions; a place where something is tested or transformed.

## Tagline

**"Forge architecture that withstands AI at scale"**

Alternative taglines:
- "Test your architecture before you build it"
- "Architecture refined under pressure"
- "Where architecture meets validation"

## Why "Crucible"?

A crucible is both:
1. **A container for refining metals** - Just as Crucible refines architectural ideas into validated specifications
2. **A severe test or trial** - Crucible validates architecture under strict rules before code is written

The name evokes:
- **Transformation** - Raw architectural ideas → Validated specifications
- **Refinement** - Continuous improvement through validation
- **Strength** - Architecture that's been tested and proven
- **Forge** - Building something durable and reliable

## Brand Voice

- **Technical but approachable** - Precise without being intimidating
- **Confident** - This solves a real problem
- **Pragmatic** - Focused on practical value, not theory
- **Open** - Community-driven, vendor-neutral

## Key Messaging

### Primary Message
Crucible is an open standard for defining application architecture that AI coding assistants can validate, understand, and maintain.

### Value Propositions
1. **For AI-assisted developers**: Maintain consistency across AI-generated code
2. **For architects**: Validate architectural decisions before implementation
3. **For teams**: Document and enforce architectural patterns
4. **For tool builders**: Standard format for architecture tooling

## Usage Examples

### Correct Usage
- "Crucible is an open standard..."
- "Use Crucible to validate your architecture"
- "The Crucible specification defines..."
- "Install the Crucible CLI: `cargo install crucible`"

### Project Components
- **crucible** - The specification and standard
- **crucible-cli** - Command-line tool
- **crucible-core** - Core validation engine
- **.crucible/** - Project directory for architecture files

### File Extensions
- `.json` - Standard JSON format
- No custom extension needed (unlike .proto, .graphql, etc.)

## Visual Identity (Future)

### Logo Concepts
- Abstract crucible/vessel shape
- Geometric forge/refinement imagery
- Fire/transformation symbolism
- Clean, modern, technical aesthetic

### Color Palette Ideas
- **Primary**: Deep orange/red (forge, heat, transformation)
- **Secondary**: Steel blue/gray (structure, reliability)
- **Accent**: Bright white/silver (refined output)

## Comparisons (How to position)

### What Crucible Is
- ✅ An open standard for application architecture
- ✅ A validation framework for architectural decisions
- ✅ A format AI assistants can understand
- ✅ Language-agnostic and tool-agnostic

### What Crucible Is Not
- ❌ Not a programming language
- ❌ Not implementation code
- ❌ Not a database schema tool
- ❌ Not tied to any specific AI assistant

## Elevator Pitch (30 seconds)

"Crucible is an open standard that lets you define your application architecture in a machine-readable format. AI coding assistants read it to maintain consistency, and you can validate architectural decisions before writing any code. Think of it as OpenAPI, but for your entire application, not just HTTP endpoints."

## Elevator Pitch (60 seconds)

"AI coding assistants are great at writing code but struggle with maintaining consistency across large codebases. They lose context between sessions and can introduce architectural drift.

Crucible solves this by providing a formal specification for application architecture. You define your modules, interfaces, dependencies, and constraints in JSON files. AI assistants read these files to understand your system, and Crucible validates that all changes respect your architectural rules—before any code is written.

It's an open standard, works with any language, and integrates with Claude Code, GitHub Copilot, and Cursor. Think of it as infrastructure-as-code, but for application architecture."

## Naming Conventions

### Commands
```bash
crucible validate           # Not: crucible-validate
crucible init              # Not: init-crucible
crucible generate          # Not: crucible-gen
```

### Packages
```
crucible-cli               # CLI tool
crucible-core              # Core library
crucible-codegen           # Code generators
crucible-vscode            # VS Code extension
```

### GitHub
```
github.com/crucible-spec/crucible        # Main repo
github.com/crucible-spec/crucible-cli    # CLI repo
```

## Community Language

### Preferred Terms
- "Crucible spec" or "the specification"
- "Crucible file" or "architecture definition"
- "Validate" (not "check" or "verify")
- "Module definition" (not "module spec")
- "Architecture pattern" (not "design pattern")

### Avoid
- "ADK" (old name)
- "Architecture Definition Kit" (old full name)
- "Crucible language" (it's JSON, not a new language)
- "Crucible code" (it's specification, not implementation)

## Competitive Positioning

### vs OpenAPI
"OpenAPI is for HTTP APIs. Crucible is for entire applications—modules, dependencies, business logic, and architectural patterns."

### vs TypeScript .d.ts
"TypeScript definitions are language-specific and about types. Crucible is language-agnostic and about architecture."

### vs Documentation
"Documentation gets out of date. Crucible is validated automatically and AI assistants use it as source of truth."

### vs UML/C4
"UML and C4 are visual. Crucible is machine-readable—AI can understand it, tools can validate it, and code generators can use it."

## Social Media Hashtags

- #Crucible
- #AIAssistedDev
- #ArchitectureAsCode
- #AIArchitecture
- #ClaudeCode
- #GitHubCopilot

## Launch Messaging

### Announcement Title Ideas
- "Introducing Crucible: Architecture Validation for AI-Assisted Development"
- "Crucible: An Open Standard for AI-Native Architecture"
- "Architecture as Code Meets AI: Introducing Crucible"

### Key Launch Points
1. AI coding assistants need structured architecture context
2. Current solutions (docs, comments) don't work at scale
3. Crucible provides machine-readable, validatable specifications
4. Open standard—works with any tool, any language
5. Already working with Claude Code team (if applicable)

## FAQ Responses

**Q: Another standard? Why?**
A: AI assistants need something they can parse and validate. Text docs don't cut it.

**Q: Do I have to use AI?**
A: No. Crucible is valuable for architecture validation and documentation even without AI.

**Q: Is this just for TypeScript?**
A: No. Crucible works with TypeScript, Rust, Python, Go, Java, and more.

**Q: Who's behind this?**
A: Crucible is an open standard. Anyone can contribute, implement, or adopt it.

## Website Copy (Future)

### Hero
"**Forge architecture that withstands AI at scale**

Define, validate, and maintain application architecture that AI coding assistants can actually understand."

### Features Section
- ⚡ **Validate Before You Build** - Catch architectural issues before writing code
- 🤖 **AI-Native Design** - Optimized for AI assistant consumption
- 🔄 **Stay Consistent** - Maintain architectural integrity across AI-generated code
- 🌐 **Language Agnostic** - One architecture, multiple implementations

## License Note

The Crucible specification is released under CC0 1.0 Universal (Public Domain). This allows:
- Anyone to implement tools
- Companies to adopt without licensing concerns
- Community to contribute freely
- Forks and derivatives without restriction

## Version Info

- **Current Version**: 0.1.0
- **Status**: Early specification
- **Release Date**: November 2025

---

**Remember**: Crucible is about making architecture concrete, validatable, and AI-accessible. Every piece of communication should reinforce this core value proposition.
