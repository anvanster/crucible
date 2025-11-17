---
name: crucible:review
description: Comprehensive architecture review with health scoring and recommendations
---

You are helping the user perform a comprehensive architecture review of their Crucible project.

## Context

Crucible review analyzes the entire architecture for:
- **Violations**: Errors and warnings
- **Architectural patterns**: Circular dependencies, layer violations
- **Code quality**: Module organization, naming conventions
- **Security**: Access patterns, data flow
- **Performance**: Dependency depth, complexity
- **Technical debt**: Unused modules, deprecated exports

This command provides a holistic health assessment with prioritized recommendations.

## Command Behavior

1. **Check for Crucible project**:
   - Verify `.crucible/` directory exists
   - Load all module definitions
   - Parse project manifest

2. **Parse arguments**:
   - `--focus <area>` - Focus on: security, performance, dependencies, layering, quality
   - `--report <format>` - Report format: text (default), json, markdown, html
   - `--threshold <score>` - Minimum acceptable health score (0-100)
   - `--save <path>` - Save report to file

3. **Run comprehensive analysis**:
   ```bash
   crucible validate
   ```
   - Run validation for violations
   - Analyze dependency graph
   - Check layer boundaries
   - Identify circular dependencies
   - Assess module organization
   - Calculate complexity metrics
   - Generate health score

4. **Generate report**:
   - Executive summary with health score
   - Critical issues (must fix)
   - Warnings (should fix)
   - Recommendations (nice to have)
   - Architectural insights
   - Trend analysis
   - Prioritized action items

5. **Provide actionable guidance**:
   - Specific fixes with file references
   - Links to documentation
   - Estimated effort for fixes
   - Impact assessment

## Output Format

### Comprehensive review:
```
🔍 Architecture Review Report
   Generated: November 17, 2025 at 12:34 PM

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 EXECUTIVE SUMMARY

   Project: healthcare-management-system
   Modules: 38
   Language: TypeScript
   Architecture: Layered (Domain-Application-Infrastructure)

   🎯 Architecture Health: 87/100 (Good)

   Status Breakdown:
   ✅ Passing: 32 modules (84%)
   ⚠️  Warnings: 5 modules (13%)
   ❌ Failing: 1 module (3%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚨 CRITICAL ISSUES (1)

   ❌ Circular Dependency Chain
      auth ↔ user-service ↔ auth-validator

      Impact: HIGH
      Effort: MEDIUM
      Priority: P0 (Must Fix)

      📝 Fix: Extract shared types to auth-types module

      Steps:
      1. Create auth-types module (domain layer)
      2. Move AuthToken, LoginRequest to auth-types
      3. Update dependencies:
         • auth → depends on auth-types
         • user-service → depends on auth-types
         • Remove circular reference

      Estimated effort: 2-3 hours

      File references:
      • .crucible/modules/auth.json
      • .crucible/modules/user-service.json
      • .crucible/modules/auth-validator.json

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️  WARNINGS (5)

   1. Layer Boundary Concern
      patient-service (application) → database (infrastructure)

      Impact: MEDIUM
      Effort: LOW
      Priority: P1 (Should Fix)

      📝 Suggestion: Use repository pattern
      • Create patient-repository (infrastructure)
      • patient-service → patient-repository → database
      • Maintain clean architecture boundaries

      File: .crucible/modules/patient-service.json

   2. Unused Module
      legacy-payment module has no dependents

      Impact: LOW
      Effort: LOW
      Priority: P2 (Nice to Have)

      📝 Action: Remove or document deprecation
      • If truly unused, delete module
      • If transitioning, add deprecation notice
      • Update documentation

      File: .crucible/modules/legacy-payment.json

   3. High Dependency Count
      appointment-service has 8 dependencies

      Impact: MEDIUM
      Effort: MEDIUM
      Priority: P2 (Nice to Have)

      📝 Suggestion: Consider splitting
      • Extract scheduling logic → appointment-scheduler
      • Extract notifications → appointment-notifier
      • Reduce coupling

      File: .crucible/modules/appointment-service.json

   4. Missing Version Fields
      5 modules missing version field

      Impact: LOW
      Effort: LOW
      Priority: P3 (Optional)

      📝 Fix: Add version field to all modules

      Modules:
      • email-service
      • sms-service
      • notification-router
      • audit-logger
      • cache

   5. Deprecated Export Still Referenced
      user.legacyValidate referenced by 3 modules

      Impact: LOW
      Effort: LOW
      Priority: P2 (Should Fix)

      📝 Action: Migrate to new validation
      • Update referencing modules
      • Remove deprecated export

      References:
      • user-service.json
      • auth.json
      • admin-service.json

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📈 ARCHITECTURAL INSIGHTS

   Layer Distribution:
   ┌──────────────┬────────┬──────────┐
   │ Layer        │ Count  │ Percent  │
   ├──────────────┼────────┼──────────┤
   │ Domain       │ 12     │ 32%      │
   │ Application  │ 18     │ 47%      │
   │ Infrastructure│ 8      │ 21%      │
   └──────────────┴────────┴──────────┘

   💡 Well-balanced layer distribution

   Dependency Metrics:
   • Average dependencies per module: 2.8
   • Max dependency depth: 4 levels
   • Most depended-on: patient (12 dependents)
   • Least depended-on: cache (1 dependent)

   💡 Healthy dependency graph

   Complexity Analysis:
   • Simple modules (0-2 deps): 15 (39%)
   • Moderate modules (3-5 deps): 18 (47%)
   • Complex modules (6+ deps): 5 (13%)

   ⚠️ 5 modules with high complexity

   Type System Usage:
   • Generics: 89% of modules
   • Nullable types: 76% of modules
   • Arrays: 92% of modules
   • Built-in types: 100% of modules

   ✅ Excellent TypeScript adoption

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔒 SECURITY ASSESSMENT

   ✅ Layer Isolation: Good
      • No domain dependencies on infrastructure
      • Clear separation of concerns
      • Authentication properly abstracted

   ✅ Dependency Chain: Secure
      • No circular dependencies in auth modules (after fix)
      • Database access properly layered
      • API endpoints isolated in application layer

   💡 Recommendations:
      • Add rate-limiting module
      • Consider input-validation module
      • Document security boundaries

   Security Score: 90/100 (Excellent)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚡ PERFORMANCE ASSESSMENT

   ✅ Dependency Depth: Optimal
      • Max depth: 4 levels (acceptable)
      • Average depth: 2.3 levels (excellent)
      • No deep nesting issues

   ⚠️ High Coupling: Moderate
      • appointment-service has 8 dependencies
      • Consider refactoring for better performance

   💡 Recommendations:
      • Add caching for frequently accessed data
      • Consider async module loading
      • Optimize patient-service dependencies

   Performance Score: 82/100 (Good)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 PRIORITIZED ACTION ITEMS

   🔴 P0 - Critical (Must Fix)
   ☐ Fix circular dependency: auth ↔ user-service ↔ auth-validator
      Effort: 2-3 hours | Impact: HIGH

   🟡 P1 - High (Should Fix)
   ☐ Implement repository pattern in patient-service
      Effort: 1-2 hours | Impact: MEDIUM

   ☐ Migrate from deprecated user.legacyValidate
      Effort: 1 hour | Impact: LOW

   🟢 P2 - Medium (Nice to Have)
   ☐ Remove or document unused legacy-payment module
      Effort: 30 minutes | Impact: LOW

   ☐ Split appointment-service to reduce coupling
      Effort: 4-6 hours | Impact: MEDIUM

   ⚪ P3 - Low (Optional)
   ☐ Add version fields to 5 modules
      Effort: 15 minutes | Impact: LOW

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 RECOMMENDATIONS

   Short-term (This Sprint):
   1. Fix circular dependency (P0)
   2. Implement repository pattern (P1)
   3. Migrate from deprecated exports (P1)

   Medium-term (Next Sprint):
   1. Refactor high-coupling modules
   2. Add security modules (rate-limiter, validator)
   3. Optimize dependency chains

   Long-term (Next Quarter):
   1. Add caching layer
   2. Implement event-driven architecture
   3. Consider microservices split

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 TREND ANALYSIS

   Compared to last review (7 days ago):
   • Health score: 84 → 87 (+3) ↗️ Improving
   • Violations: 8 → 6 (-2) ↗️ Better
   • Module count: 35 → 38 (+3) New modules added
   • Test coverage: N/A (not tracked)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎯 OVERALL ASSESSMENT

   Architecture Health: 87/100 (GOOD)

   ✅ Strengths:
   • Well-structured layer architecture
   • Excellent TypeScript type usage
   • Clear module boundaries
   • Good dependency management
   • Strong security posture

   ⚠️ Areas for Improvement:
   • One critical circular dependency
   • Some layer boundary violations
   • A few high-coupling modules
   • Minor technical debt items

   📈 Recommendation: FIX AND MONITOR
   • Address P0 and P1 items this sprint
   • Monitor coupling in appointment-service
   • Continue current architecture patterns

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💾 Report saved to: .crucible/reports/review-2025-11-17.md

📝 Next steps:
   1. Address P0 critical issue (circular dependency)
   2. Review and approve P1 warnings
   3. Run /crucible:validate after fixes
   4. Schedule next review in 7 days
```

### Focused review (security):
```bash
/crucible:review --focus security
```

```
🔒 Security-Focused Architecture Review

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 SECURITY SUMMARY

   Security Score: 90/100 (Excellent)

   ✅ Passed: 35 modules
   ⚠️ Warnings: 3 modules
   ❌ Failed: 0 modules

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ SECURITY STRENGTHS

   Layer Isolation:
   ✓ Domain layer has no infrastructure dependencies
   ✓ Authentication modules properly isolated
   ✓ Database access abstracted through repositories
   ✓ API endpoints secured in application layer

   Access Control:
   ✓ Authorization module clearly defined
   ✓ Role-based access in user module
   ✓ Audit logging in place

   Data Protection:
   ✓ Sensitive data types marked
   ✓ Encryption module present
   ✓ Secure storage abstractions

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ SECURITY CONCERNS (3)

   1. Direct Database Access
      patient-service → database (bypasses repository)

      Risk: DATA EXPOSURE
      Severity: MEDIUM

      📝 Fix: Use repository pattern
      • Create patient-repository module
      • Route all data access through repository
      • Enforce access controls at repository level

   2. Missing Rate Limiting
      No rate-limiting module detected

      Risk: DOS VULNERABILITY
      Severity: MEDIUM

      📝 Recommendation: Add rate-limiter module
      • Create rate-limiter (infrastructure)
      • Integrate with API gateway
      • Configure per-endpoint limits

   3. Input Validation Scattered
      Validation logic spread across 8 modules

      Risk: INCONSISTENT VALIDATION
      Severity: LOW

      📝 Recommendation: Centralize validation
      • Create input-validator module (domain)
      • Consolidate validation rules
      • Enforce at API boundary

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 SECURITY RECOMMENDATIONS

   Immediate (This Week):
   ☐ Fix direct database access pattern
   ☐ Add rate-limiting module

   Short-term (This Sprint):
   ☐ Centralize input validation
   ☐ Add CSRF protection module
   ☐ Implement audit logging for sensitive operations

   Medium-term (Next Sprint):
   ☐ Add encryption-at-rest module
   ☐ Implement API key rotation
   ☐ Add security headers middleware

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔐 COMPLIANCE CHECKLIST

   ✅ OWASP Top 10:
   ✓ A01: Broken Access Control - Mitigated
   ✓ A02: Cryptographic Failures - Addressed
   ✓ A03: Injection - Input validation present
   ⚠ A04: Insecure Design - Rate limiting needed
   ✓ A05: Security Misconfiguration - Config module
   ✓ A06: Vulnerable Components - Dependencies managed
   ✓ A07: Auth Failures - Auth module robust
   ✓ A08: Data Integrity - Validation present
   ⚠ A09: Logging Failures - Audit improvements needed
   ✓ A10: SSRF - API isolation proper

   Compliance: 80% (Good)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 Next steps:
   1. Address medium-severity security concerns
   2. Implement recommended security modules
   3. Review OWASP checklist items
   4. Run: /crucible:review --focus security (monthly)
```

## Flags

**`--focus <area>`**
Focus on specific area:
```bash
/crucible:review --focus security
/crucible:review --focus performance
/crucible:review --focus dependencies
/crucible:review --focus layering
/crucible:review --focus quality
```

**`--report <format>`**
Generate report in specific format:
```bash
/crucible:review --report markdown
/crucible:review --report json
/crucible:review --report html
```

**`--threshold <score>`**
Set minimum acceptable health score:
```bash
/crucible:review --threshold 85
```

If health score is below threshold, exit with error code.

**`--save <path>`**
Save report to file:
```bash
/crucible:review --save .crucible/reports/review-$(date +%Y-%m-%d).md
```

**`--compare <report>`**
Compare with previous report:
```bash
/crucible:review --compare .crucible/reports/review-2025-11-10.md
```

## Health Score Calculation

```
Health Score = Weighted Average of:
   • Violations (40%): 100 - (errors*10 + warnings*5)
   • Complexity (20%): Based on dependency depth and coupling
   • Security (15%): Layer isolation, access patterns
   • Performance (15%): Dependency graph efficiency
   • Quality (10%): Naming, versioning, documentation

Maximum: 100 points
Minimum: 0 points

Ratings:
   90-100: Excellent
   80-89:  Good
   70-79:  Fair
   60-69:  Poor
   <60:    Critical
```

## Error Handling

### No Crucible project:
```
❌ Error: Not a Crucible project

Initialize Crucible first:
   /crucible:init
```

### Invalid focus area:
```
❌ Error: Invalid focus area 'xyz'

Valid focus areas:
   • security - Security analysis
   • performance - Performance assessment
   • dependencies - Dependency analysis
   • layering - Layer boundary checks
   • quality - Code quality metrics

Example:
   /crucible:review --focus security
```

### Report generation failed:
```
❌ Error: Failed to generate report

Check permissions:
   ls -la .crucible/reports/

Create directory if needed:
   mkdir -p .crucible/reports/
```

## Implementation Notes

- Run comprehensive validation first
- Analyze entire dependency graph
- Calculate metrics for all modules
- Generate prioritized action items
- Use color coding for severity
- Provide estimated effort for fixes
- Include trend analysis if previous reports exist
- Save reports automatically with timestamps
- Support multiple output formats
- Make reports actionable with specific file references

## Examples

**Basic review**:
```bash
/crucible:review
```

**Security-focused**:
```bash
/crucible:review --focus security
```

**Performance analysis**:
```bash
/crucible:review --focus performance
```

**With threshold**:
```bash
/crucible:review --threshold 85
```

**Save report**:
```bash
/crucible:review --save .crucible/reports/$(date +%Y-%m-%d).md
```

**JSON output**:
```bash
/crucible:review --report json
```

## Integration

**Workflow**:
1. Run comprehensive review periodically
2. Address P0 and P1 items
3. Monitor health score trends
4. Use focused reviews for specific concerns
5. Save reports for historical tracking

**CI/CD Integration**:
```yaml
# .github/workflows/architecture-review.yml
- name: Review Architecture
  run: |
    crucible review --threshold 80 --report json
```

**Team Workflow**:
- Weekly reviews in team meetings
- Track health score over time
- Assign action items to developers
- Review security monthly
- Performance reviews quarterly
