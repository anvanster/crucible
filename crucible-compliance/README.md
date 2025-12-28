# Crucible Compliance

Compliance validation for Crucible architecture definitions. Validate your architecture against regulatory frameworks like HIPAA, PCI-DSS, and SOC2 before writing code.

## Overview

Crucible Compliance validates your architecture definitions against compliance frameworks:

- **HIPAA** - Healthcare data protection (53 rules covering Security Rule requirements)
- **PCI-DSS** - Payment card data security (coming soon)
- **SOC2** - Service organization controls (coming soon)

## Installation

```bash
cargo install crucible-compliance
```

Or build from source (part of the Crucible monorepo):

```bash
git clone https://github.com/anvanster/crucible
cd crucible/crucible-compliance
cargo build --release
```

## Usage

### CLI

```bash
# Validate a project against all loaded frameworks
crucible-comply --project /path/to/project

# Validate against specific framework
crucible-comply --project . --frameworks HIPAA

# Use custom framework file
crucible-comply --project . --framework-path ./my-framework.json

# Output as JSON for CI/CD integration
crucible-comply --project . --output json

# Output as SARIF for IDE integration
crucible-comply --project . --output sarif

# Output as HTML for audit reports
crucible-comply --project . --output html -O report.html

# List available frameworks
crucible-comply --list-frameworks

# Strict mode (fail on warnings too)
crucible-comply --project . --strict
```

### Library Usage

```rust
use crucible_compliance::{
    ComplianceValidator, Framework, FrameworkLoader,
    OutputFormat, ReportConfig, Reporter,
};
use crucible_core::Parser;

// Load compliance frameworks
let mut loader = FrameworkLoader::new();
loader.load_directory("./frameworks")?;

// Load your Crucible project
let parser = Parser::new(".crucible");
let project = parser.parse_project()?;

// Validate against HIPAA
let framework = loader.get("HIPAA").unwrap();
let validator = ComplianceValidator::new(framework);
let report = validator.validate(&project)?;

// Check results
if report.passed() {
    println!("Compliance validation passed!");
} else {
    println!("Found {} errors and {} warnings",
        report.error_count(),
        report.warning_count()
    );
}

// Generate formatted output
let reporter = Reporter::html();
let html = reporter.format(&report);
```

## Architecture Annotations

Crucible Compliance uses annotations in your architecture definitions to validate compliance. Common annotations include:

### Data Classification
- `@phi` - Protected Health Information (HIPAA)
- `@pii` - Personally Identifiable Information
- `@ephi` - Electronic PHI

### Access Control
- `@requires-auth` - Requires authentication
- `@requires-role` - Requires role-based authorization
- `@phi-access` - Method accesses PHI

### Security Controls
- `@encrypted` - Data is encrypted at rest
- `@https-only` - Requires TLS transport
- `@audit-logged` - Access is audit logged

### Example Module

```json
{
  "module": "patient",
  "version": "1.0.0",
  "exports": {
    "PatientRecord": {
      "type": "class",
      "properties": {
        "ssn": {
          "type": "string",
          "annotations": ["@phi", "@encrypted"]
        },
        "name": {
          "type": "string",
          "annotations": ["@pii"]
        }
      },
      "methods": {
        "getRecord": {
          "annotations": ["@requires-auth", "@phi-access"],
          "effects": ["audit.log"],
          "returns": { "type": "PatientRecord" }
        }
      }
    }
  }
}
```

## Output Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| `text` | Human-readable terminal output | Development |
| `json` | Structured JSON | CI/CD pipelines, automation |
| `sarif` | Static Analysis Results Format | IDE integration, GitHub |
| `markdown` | Markdown report | Documentation, PRs |
| `html` | Styled HTML report | Audits, compliance evidence |

## Frameworks

### HIPAA Security Rule

The HIPAA framework includes 53 rules covering:

- **Administrative Safeguards** (164.308)
  - Security management process
  - Workforce security
  - Information access management
  - Security awareness training
  - Security incident procedures
  - Contingency planning

- **Technical Safeguards** (164.312)
  - Access control (unique user ID, emergency access, automatic logoff, encryption)
  - Audit controls
  - Integrity controls
  - Person/entity authentication
  - Transmission security

- **Organizational Requirements** (164.314)
  - Business associate contracts

Each rule includes:
- Severity level (error/warning)
- HIPAA requirement reference
- Rationale explaining the requirement
- Violation cost estimates where applicable
- Examples of violations and compliant code

### Custom Frameworks

Create custom compliance frameworks by defining JSON files:

```json
{
  "compliance_framework": "MyFramework",
  "version": "1.0.0",
  "description": "Custom compliance rules",
  "requirements": [...],
  "rules": [
    {
      "id": "my-rule",
      "severity": "error",
      "description": "Description of the rule",
      "validates": {
        "type": "effect_check",
        "when_effect": ["logging"],
        "forbidden_data": ["@sensitive"]
      }
    }
  ]
}
```

## Rule Types

### effect_check
Validates that certain effects don't access forbidden data:
```json
{
  "type": "effect_check",
  "when_effect": ["logging"],
  "forbidden_data": ["@phi"]
}
```

### effect_requirement
Requires certain effects when accessing specific data:
```json
{
  "type": "effect_requirement",
  "when_accessing": ["@phi-access"],
  "required_effects": ["audit.log"]
}
```

### storage_check
Validates storage annotations for data types:
```json
{
  "type": "storage_check",
  "when_accessing": ["@phi"],
  "required_annotations": ["@encrypted"]
}
```

### data_access_check
Validates method annotations for data access:
```json
{
  "type": "data_access_check",
  "when_accessing": ["@phi"],
  "required_annotations": ["@requires-auth"]
}
```

## License

This software is licensed under the [Business Source License 1.1](LICENSE).

The Licensed Work will become available under the Apache License 2.0 on December 31, 2029.

### What's Permitted

- Using the Licensed Work to validate your own organization's infrastructure and applications
- Integrating the Licensed Work into internal CI/CD pipelines
- Forking and modifying for internal organizational use
- Contributing improvements back to the project
- Evaluation, testing, and development purposes
- Academic and research use

### What's Not Permitted

- Offering the Licensed Work as part of a commercial compliance validation service
- Redistributing the compliance rule definitions as a standalone dataset
- Providing managed compliance-as-a-service offerings to third parties

## Related Projects

- [Crucible](https://github.com/anvanster/crucible) - Architecture-first development framework
- [crucible-core](https://crates.io/crates/crucible-core) - Core types and validation engine
