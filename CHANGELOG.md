# Changelog

All notable changes to Crucible will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2025-01-17

### Enhanced
- **`/crucible:architecture` Command**: Major automation improvements with comprehensive project intelligence
  - Automatic manifest.json detection and smart merge/replace strategy prompts
  - Intelligent rules.json layer conflict detection and automatic updates
  - Multi-phase workflow with clear progress indicators (Analysis → Design → Confirmation → Generation → Validation → TDD)
  - Post-generation validation with categorized violations by type
  - Guided fixes workflow with automatic and manual fix suggestions
  - Generic, language-agnostic instructions using placeholders for any project type
  - Added comprehensive flags: `--merge`, `--replace`, `--no-validate`, `--layer`, `--layers`, `--language`, `--template`
  - Interactive prompts for all critical decisions using AskUserQuestion tool
  - Phase-based execution prevents common user errors and improves workflow efficiency

### Documentation
- Completely revised `/crucible:architecture` command documentation (~15KB)
- Added detailed implementation notes for Claude Code integration
- Comprehensive examples using generic placeholders (`[module-name]`, `[layer]`, etc.)
- Conditional logic blocks for branching behavior across different project scenarios

## [0.1.5] - 2025-01-17

### Added
- **Complete Claude Code Integration**: 8 native slash commands for seamless architecture-first development
  - `/crucible:validate` - Run architecture validation with actionable fixes
  - `/crucible:architecture` - Design architecture for new features (architecture-first TDD)
  - `/crucible:init` - Initialize Crucible in current project
  - `/crucible:module` - Create or update module definitions interactively
  - `/crucible:review` - Comprehensive architecture review with health scoring
  - `/crucible:sync` - Sync architecture ↔ code bidirectionally
  - `/crucible:analyze` - Deep dive into module dependencies and usage
  - `/crucible:diff` - Show git-style differences between architecture and code
- All slash commands automatically generated on `crucible init`
- `--here` flag for `crucible init` to initialize in existing project directory
- `--force` flag for `crucible init` to reinitialize with interactive confirmation prompt
- Interactive confirmation when using `--force` to prevent accidental architecture deletion
- Comprehensive error messages with actionable suggestions for init command
- Validation to prevent accidental overwrites of existing `.crucible/` directories

### Changed
- `crucible init --name` is now optional when using `--here` flag
- Enhanced error messages with helpful options and examples
- Improved user experience with clear warnings for destructive operations

### Documentation
- Added complete slash commands documentation (~107KB across 8 command files)
- Created `docs/SLASH_COMMANDS_IMPLEMENTATION_PLAN.md` - Complete implementation plan
- Created `docs/CLAUDE_CODE_INTEGRATION.md` - User-facing integration guide
- Created `docs/SLASH_COMMANDS_SUMMARY.md` - Implementation summary and statistics
- Updated `README.md` with Claude Code Integration section
- Updated all command documentation with `--here` and `--force` flag usage

## [0.1.4] - 2025-01-16

### Added
- Initial release with core architecture validation
- TypeScript type system support
- Module dependency validation
- Basic CLI commands: `init`, `validate`, `generate`

[0.1.6]: https://github.com/anvanster/crucible/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/anvanster/crucible/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/anvanster/crucible/releases/tag/v0.1.4
