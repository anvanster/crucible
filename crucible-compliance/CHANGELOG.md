# Changelog

All notable changes to crucible-compliance will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-27

### Added

- Initial release of crucible-compliance
- **HIPAA Security Rule Framework** with 53 rules covering:
  - Administrative Safeguards (164.308)
  - Technical Safeguards (164.312)
  - Organizational Requirements (164.314)
  - Documentation Requirements (164.316)
- **Compliance Validator Engine** supporting four rule types:
  - `effect_check` - Validate effects don't access forbidden data
  - `effect_requirement` - Require effects when accessing data
  - `storage_check` - Validate storage annotations
  - `data_access_check` - Validate method annotations
- **Multiple Output Formats**:
  - Text - Human-readable terminal output
  - JSON - Structured output for CI/CD
  - SARIF - IDE integration format
  - Markdown - Documentation and PRs
  - HTML - Audit-ready reports with styling
- **CLI Tool** (`crucible-comply`) with options:
  - Framework selection and custom framework loading
  - Multiple output format support
  - Strict mode for treating warnings as errors
  - Verbose output for debugging
- **Library API** for programmatic integration
- Comprehensive test suite with 84 tests

### Framework Coverage

#### HIPAA Rules by Category

| Category | Error Rules | Warning Rules | Total |
|----------|-------------|---------------|-------|
| Access Control | 12 | 4 | 16 |
| Audit & Logging | 8 | 2 | 10 |
| Encryption | 6 | 2 | 8 |
| Data Protection | 5 | 3 | 8 |
| Authentication | 4 | 3 | 7 |
| Transmission | 3 | 0 | 3 |
| Business Associates | 1 | 0 | 1 |

### Dependencies

- `crucible-core` 0.1.10+ for architecture types and parsing
- `clap` 4.5+ for CLI argument parsing
- `serde` 1.0+ for JSON serialization
- `chrono` 0.4+ for timestamp generation

[0.1.0]: https://github.com/anvanster/crucible/releases/tag/compliance-v0.1.0
