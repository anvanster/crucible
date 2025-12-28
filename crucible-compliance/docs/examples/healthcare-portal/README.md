# Healthcare Portal Example

This is a complete example of a HIPAA-compliant patient portal architecture using Crucible compliance validation.

## Overview

This example demonstrates:
- Proper PHI field annotations (`@phi`, `@encrypted`)
- Required authentication annotations (`@requires-auth`)
- Audit logging effects (`audit.log`)
- Separation of PHI and non-PHI data
- Minimum necessary principle in data access

## Architecture

```
.crucible/
├── manifest.json          # Project configuration
└── modules/
    ├── patient.json       # Patient domain model with PHI handling
    ├── medical-records.json # Medical records with sensitive data
    ├── appointments.json  # Appointment scheduling
    ├── auth.json         # Authentication and authorization
    └── audit.json        # HIPAA-compliant audit logging
```

## Compliance Patterns

### 1. PHI Annotation

All PHI fields are marked with `@phi` annotation:

```json
{
  "ssn": {
    "type": "string",
    "annotations": ["@phi", "@encrypted"]
  }
}
```

### 2. Encryption at Rest

Sensitive PHI fields include `@encrypted` annotation:

```json
{
  "diagnosis": {
    "type": "Array<Diagnosis>",
    "annotations": ["@phi", "@encrypted"]
  }
}
```

### 3. Audit Logging

All PHI access methods include `audit.log` effect:

```json
{
  "getByPatientId": {
    "effects": ["database.read", "audit.log"],
    "annotations": ["@phi-access", "@requires-auth"]
  }
}
```

### 4. Authentication Required

PHI access requires authentication:

```json
{
  "findById": {
    "annotations": ["@phi-access", "@requires-auth"]
  }
}
```

### 5. Minimum Necessary

Search results return `PatientSummary` instead of full `Patient`:

```json
{
  "search": {
    "returns": {"type": "Promise", "inner": "Array<PatientSummary>"}
  }
}
```

## Validating Compliance

Run the compliance validator:

```bash
# From the crucible-compliance directory
cargo build --release

# Validate this example
./target/release/crucible-comply \
  --project examples/healthcare-portal \
  --frameworks HIPAA \
  --verbose
```

**Note**: This example intentionally includes some compliance gaps to demonstrate how Crucible catches issues. When you run validation, you'll see violations like:

```
✗ HIPAA Violation: phi-storage-encryption
  Location: patient.PatientSummary.dateOfBirth
  Issue: Property with @phi annotations requires @encrypted annotations
  Suggestion: Add @encrypted to the property annotations
```

This is by design - the example shows both compliant patterns (e.g., `Patient.ssn` has both `@phi` and `@encrypted`) and non-compliant patterns (e.g., `PatientSummary.dateOfBirth` has `@phi` but not `@encrypted`).

## Introducing a Violation

To see how Crucible catches violations, modify `patient.json`:

1. Remove `@encrypted` from `ssn`:
```json
{
  "ssn": {
    "type": "string",
    "annotations": ["@phi"]  // Missing @encrypted!
  }
}
```

2. Run validation:
```bash
./target/release/crucible-comply --project examples/healthcare-portal

✗ HIPAA Violation: phi-storage-encryption
  Location: patient.Patient.ssn
  Issue: Property with @phi annotations requires @encrypted annotations
  Suggestion: Add @encrypted to the property annotations
```

## Module Details

### patient.json

The patient module defines:
- `Patient` - Full patient record with all PHI properly annotated
- `Address` - Address component with PHI location data
- `PatientService` - Service with audit logging and auth requirements
- `PatientSummary` - Minimal data for search results (minimum necessary)

### medical-records.json

The medical records module contains the most sensitive PHI:
- `MedicalRecord` - Complete medical record
- `Diagnosis` - ICD codes and descriptions
- `Medication` - Prescriptions and dosages
- `LabResult` - Test results
- `VitalSigns` - Patient vitals
- `MedicalRecordService` - Service with strict access controls

### appointments.json

Appointment scheduling with:
- `Appointment` - Appointment details (reason is PHI)
- `AppointmentService` - Scheduling with audit trails

### auth.json

Authentication and authorization:
- `User` - User with role-based permissions
- `Session` - Session management with timeouts
- `AuthService` - Login/logout with MFA support

### audit.json

HIPAA-compliant audit logging:
- `AuditEntry` - Immutable audit log entry
- `AuditLogger` - Logging service for all PHI access
- Query and export capabilities for compliance audits

## Best Practices Demonstrated

1. **Safe Identifiers**: Use internal UUIDs (`id`) for logging, not PHI identifiers
2. **Explicit PHI Marking**: Every PHI field is annotated
3. **Defense in Depth**: Both annotation and effect requirements
4. **Audit Everything**: All PHI access generates audit entries
5. **Role-Based Access**: Permissions control who can access what
6. **MFA Support**: Multi-factor authentication for sensitive access

## Learn More

- [HIPAA Compliance Guide](../../docs/HIPAA-COMPLIANCE-GUIDE.md)
- [Crucible Documentation](https://github.com/crucible/docs)
