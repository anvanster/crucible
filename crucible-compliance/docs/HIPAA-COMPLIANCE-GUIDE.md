# HIPAA Compliance Guide for Developers

A practical guide to building HIPAA-compliant applications using Crucible compliance validation.

## Table of Contents

1. [Introduction to HIPAA for Developers](#introduction-to-hipaa-for-developers)
2. [Protected Health Information (PHI)](#protected-health-information-phi)
3. [Common HIPAA Violations in Code](#common-hipaa-violations-in-code)
4. [Using Crucible for HIPAA Compliance](#using-crucible-for-hipaa-compliance)
5. [Complete Example: Patient Portal](#complete-example-patient-portal)
6. [Checklist for HIPAA-Compliant Development](#checklist-for-hipaa-compliant-development)
7. [FAQ](#faq)

---

## Introduction to HIPAA for Developers

### What is HIPAA?

The Health Insurance Portability and Accountability Act (HIPAA) is a US federal law that sets standards for protecting sensitive patient health information. For developers, the key regulations are:

- **Privacy Rule**: Controls who can access PHI
- **Security Rule**: Technical safeguards for electronic PHI (ePHI)
- **Breach Notification Rule**: Requirements when PHI is exposed

### Why Should Developers Care?

HIPAA violations are expensive:
- **Tier 1** (Unknowing): $100 - $50,000 per violation
- **Tier 2** (Reasonable Cause): $1,000 - $50,000 per violation
- **Tier 3** (Willful Neglect, Corrected): $10,000 - $50,000 per violation
- **Tier 4** (Willful Neglect, Not Corrected): $50,000+ per violation
- **Annual Maximum**: Up to $1.5 million per violation category

Many violations stem from code-level issues that developers can prevent.

### The AI Coding Challenge

AI coding assistants (Claude, Copilot, Cursor) dramatically increase productivity but don't understand healthcare compliance. They may generate code that:

- Logs patient identifiers
- Stores PHI without encryption
- Transmits data without proper security
- Bypasses access controls

Crucible catches these issues before they reach production.

---

## Protected Health Information (PHI)

### What Qualifies as PHI?

PHI includes any individually identifiable health information. The 18 HIPAA identifiers are:

| Category | Identifiers |
|----------|-------------|
| **Personal** | Names, Social Security Numbers, Phone/Fax numbers, Email addresses |
| **Geographic** | Street addresses, City, State, ZIP (more specific than state) |
| **Dates** | Birth date, Admission date, Discharge date, Death date |
| **Numbers** | Medical record numbers, Health plan beneficiary numbers, Account numbers, Certificate/license numbers |
| **Identifiers** | Vehicle identifiers, Device identifiers, Web URLs, IP addresses |
| **Biometric** | Fingerprints, Voiceprints, Facial photographs |
| **Other** | Any other unique identifying number or code |

### Common PHI Fields in Code

```typescript
// These fields contain PHI
interface Patient {
  // Personal identifiers
  ssn: string;                    // PHI - Social Security Number
  firstName: string;              // PHI - Name
  lastName: string;               // PHI - Name
  email: string;                  // PHI - Contact info
  phone: string;                  // PHI - Contact info

  // Geographic
  address: Address;               // PHI - Location
  zipCode: string;                // PHI - Location

  // Dates
  dateOfBirth: Date;              // PHI - Date
  admissionDate: Date;            // PHI - Date

  // Medical identifiers
  medicalRecordNumber: string;    // PHI - Medical identifier
  healthPlanId: string;           // PHI - Plan identifier

  // Medical data
  diagnosis: string[];            // PHI - Medical data
  medications: Medication[];      // PHI - Medical data
  labResults: LabResult[];        // PHI - Medical data

  // Safe identifiers (not PHI when alone)
  id: string;                     // Safe - Internal UUID
  createdAt: Date;                // Safe - System date
}
```

### Annotating PHI in Crucible

Use annotations to mark PHI fields:

```json
{
  "module": "patient",
  "exports": {
    "Patient": {
      "type": "interface",
      "properties": {
        "id": {
          "type": "string",
          "annotations": []
        },
        "ssn": {
          "type": "string",
          "annotations": ["@phi", "@encrypted"]
        },
        "diagnosis": {
          "type": "Array<string>",
          "annotations": ["@phi", "@encrypted"]
        }
      }
    }
  }
}
```

---

## Common HIPAA Violations in Code

### Violation 1: PHI in Application Logs

**The Problem**: Developers often log patient information for debugging.

```typescript
// VIOLATION: Logging PHI
logger.info(`Processing appointment for ${patient.name}, DOB: ${patient.dob}`);
logger.debug(`Patient SSN: ${patient.ssn}`);
```

**Why It's a Violation**: Logs are often:
- Stored without encryption
- Retained longer than necessary
- Accessible to unauthorized personnel
- Sent to third-party logging services

**Compliant Solution**:
```typescript
// COMPLIANT: Log only anonymized identifiers
logger.info(`Processing appointment for patient ${patient.id}`);
auditLog.record('appointment_access', {
  patientId: patient.id,
  userId: currentUser.id,
  action: 'view'
});
```

**Crucible Detection**:
```bash
$ crucible-comply --project . --frameworks HIPAA

✗ HIPAA Violation: no-phi-in-logs
  Location: appointments.AppointmentService.logPatient
  Issue: Method with logging effect accesses data with @phi annotations
  Suggestion: Use redacted logging or remove PHI from logs
```

### Violation 2: Unencrypted PHI Storage

**The Problem**: Storing PHI without encryption at rest.

```typescript
// VIOLATION: Unencrypted storage
await db.patients.insert({
  id: uuid(),
  name: patient.name,
  ssn: patient.ssn,        // Stored in plaintext!
  diagnosis: patient.diagnosis
});
```

**Why It's a Violation**: HIPAA requires encryption of ePHI at rest. If the database is breached, all data is exposed.

**Compliant Solution**:
```typescript
// COMPLIANT: Encrypt sensitive fields
await db.patients.insert({
  id: uuid(),
  name: encrypt(patient.name),
  ssn: encrypt(patient.ssn),
  diagnosis: encrypt(patient.diagnosis)
});
```

Or use field-level encryption:
```json
{
  "ssn": {
    "type": "string",
    "annotations": ["@phi", "@encrypted"]
  }
}
```

**Crucible Detection**:
```bash
✗ HIPAA Violation: phi-storage-encryption
  Location: patient.PatientRecord.ssn
  Issue: Property with @phi annotations requires @encrypted annotations
  Suggestion: Add @encrypted to the property annotations
```

### Violation 3: Missing Audit Logs

**The Problem**: Accessing PHI without creating audit trails.

```typescript
// VIOLATION: No audit logging
async function getPatientRecord(id: string) {
  return await db.patients.findById(id);
}
```

**Why It's a Violation**: HIPAA requires audit controls to track who accessed what PHI and when.

**Compliant Solution**:
```typescript
// COMPLIANT: Audit all PHI access
async function getPatientRecord(id: string, userId: string) {
  await auditLog.record({
    action: 'patient_record_access',
    patientId: id,
    userId: userId,
    timestamp: new Date(),
    ipAddress: request.ip
  });
  return await db.patients.findById(id);
}
```

**Crucible Detection**:
```bash
✗ HIPAA Violation: phi-access-requires-audit
  Location: patient.PatientRepository.getPatientData
  Issue: Method with @phi-access annotations requires audit.log effects
  Suggestion: Add audit.log to the method's effects
```

### Violation 4: Missing Authentication

**The Problem**: Accessing PHI without verifying user identity.

```typescript
// VIOLATION: No authentication check
app.get('/patients/:id', async (req, res) => {
  const patient = await getPatient(req.params.id);
  res.json(patient);
});
```

**Why It's a Violation**: HIPAA requires verification of identity before granting access to PHI.

**Compliant Solution**:
```typescript
// COMPLIANT: Require authentication
app.get('/patients/:id',
  authenticate,           // Verify user identity
  authorizePatientAccess, // Verify user can access this patient
  async (req, res) => {
    const patient = await getPatient(req.params.id);
    res.json(patient);
  }
);
```

**Crucible Detection**:
```bash
✗ HIPAA Violation: phi-requires-authentication
  Location: patient.PatientService.getPatient
  Issue: Accessing data with @phi annotations requires @requires-auth
  Suggestion: Add @requires-auth to the method annotations
```

---

## Using Crucible for HIPAA Compliance

### Installation

```bash
# Navigate to crucible-compliance directory
cd crucible-compliance

# Build the CLI
cargo build --release

# The binary is at target/release/crucible-comply
```

### Basic Usage

```bash
# Validate a project against HIPAA
crucible-comply --project /path/to/project --frameworks HIPAA

# List available frameworks
crucible-comply --list-frameworks

# Generate JSON report
crucible-comply --project . --output json > compliance-report.json

# Generate Markdown report
crucible-comply --project . --output markdown > COMPLIANCE.md

# Generate HTML report for audits
crucible-comply --project . --output html > compliance-report.html

# Generate SARIF for IDE integration
crucible-comply --project . --output sarif > crucible.sarif
```

### Setting Up Your Project

1. **Create the Crucible architecture directory**:
```bash
mkdir -p .crucible/modules
```

2. **Create manifest.json**:
```json
{
  "version": "0.1.0",
  "project": {
    "name": "my-healthcare-app",
    "language": "typescript",
    "architecture_pattern": "layered"
  },
  "modules": ["patient", "medical-records", "auth", "audit"],
  "strict_validation": true
}
```

3. **Define modules with PHI annotations**:
```json
{
  "module": "patient",
  "version": "1.0.0",
  "layer": "domain",
  "exports": {
    "Patient": {
      "type": "interface",
      "properties": {
        "id": {
          "type": "string",
          "required": true,
          "annotations": []
        },
        "ssn": {
          "type": "string",
          "annotations": ["@phi", "@encrypted"]
        }
      }
    },
    "PatientService": {
      "type": "class",
      "methods": {
        "getPatient": {
          "inputs": [{"name": "id", "type": "string"}],
          "returns": {"type": "Patient"},
          "effects": ["audit.log"],
          "annotations": ["@phi-access", "@requires-auth"]
        }
      }
    }
  }
}
```

4. **Run validation**:
```bash
crucible-comply --project . -v
```

### CI/CD Integration

Add to your GitHub Actions workflow:

```yaml
name: Compliance Check

on: [push, pull_request]

jobs:
  compliance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Build crucible-comply
        run: cargo build --release
        working-directory: crucible-compliance

      - name: Run HIPAA validation
        run: |
          ./crucible-compliance/target/release/crucible-comply \
            --project . \
            --frameworks HIPAA \
            --output sarif > crucible.sarif

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: crucible.sarif
```

### Git Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
echo "Running HIPAA compliance check..."

crucible-comply --project . --frameworks HIPAA --strict

if [ $? -ne 0 ]; then
  echo "HIPAA compliance check failed. Commit blocked."
  exit 1
fi
```

---

## Complete Example: Patient Portal

See the `examples/healthcare-portal/` directory for a complete example.

### Project Structure

```
healthcare-portal/
├── .crucible/
│   ├── manifest.json
│   └── modules/
│       ├── patient.json
│       ├── appointments.json
│       ├── medical-records.json
│       ├── auth.json
│       └── audit.json
├── src/
│   ├── patient/
│   ├── appointments/
│   ├── medical-records/
│   ├── auth/
│   └── audit/
└── README.md
```

### Key Patterns

**1. Separate PHI and non-PHI identifiers**:
```json
{
  "id": {
    "type": "string",
    "description": "Internal UUID - safe for logging",
    "annotations": []
  },
  "patientId": {
    "type": "string",
    "description": "External patient identifier",
    "annotations": ["@phi"]
  }
}
```

**2. Always audit PHI access**:
```json
{
  "getPatientRecord": {
    "effects": ["database.read", "audit.log"],
    "annotations": ["@phi-access", "@requires-auth"]
  }
}
```

**3. Mark encryption requirements**:
```json
{
  "diagnosis": {
    "type": "Array<string>",
    "annotations": ["@phi", "@encrypted"]
  }
}
```

---

## Checklist for HIPAA-Compliant Development

### Architecture

- [ ] All PHI fields marked with `@phi` annotation
- [ ] Sensitive fields marked with `@encrypted` annotation
- [ ] PHI access methods marked with `@requires-auth`
- [ ] PHI access methods include `audit.log` effect

### Logging

- [ ] No PHI in application logs
- [ ] Using anonymized identifiers in logs
- [ ] Audit logs for all PHI access
- [ ] Audit logs include: who, what, when, from where

### Storage

- [ ] PHI encrypted at rest
- [ ] Encryption keys properly managed
- [ ] Database access controlled

### Transmission

- [ ] All PHI transmitted over HTTPS/TLS
- [ ] API endpoints properly authenticated
- [ ] Session timeouts configured

### Access Control

- [ ] Authentication required for PHI access
- [ ] Role-based access control implemented
- [ ] Minimum necessary access enforced

### Validation

- [ ] Crucible compliance check in CI/CD
- [ ] Pre-commit hook for local validation
- [ ] Regular compliance audits

---

## FAQ

### Q: Does Crucible guarantee HIPAA compliance?

No. Crucible is a tool that helps developers avoid common code-level violations. HIPAA compliance also requires administrative, physical, and organizational controls that are beyond what code validation can address.

### Q: What if I get a false positive?

You can:
1. Add appropriate annotations to clarify the field's purpose
2. Customize the framework rules for your specific use case
3. Report the issue to improve the rules

### Q: Can I create custom compliance rules?

Yes. Create a JSON file following the compliance framework schema and load it with `--framework-path`.

### Q: How do I handle third-party services?

Document third-party PHI access in your architecture. Crucible can validate that your code properly protects data before sending to third parties.

### Q: What about data at rest in third-party databases?

Ensure your database provider is HIPAA-compliant (has a BAA) and use field-level encryption for additional protection.

---

## Resources

- [HHS HIPAA Security Rule](https://www.hhs.gov/hipaa/for-professionals/security/index.html)
- [HIPAA Journal](https://www.hipaajournal.com/)
- [NIST HIPAA Security Toolkit](https://www.nist.gov/healthcare)
- [Crucible Documentation](https://github.com/crucible/docs)

---

## Support

For questions about Crucible compliance validation:
- GitHub Issues: [crucible/issues](https://github.com/crucible/issues)
- Documentation: [crucible/docs](https://github.com/crucible/docs)

For HIPAA compliance questions, consult with a qualified healthcare compliance professional.
