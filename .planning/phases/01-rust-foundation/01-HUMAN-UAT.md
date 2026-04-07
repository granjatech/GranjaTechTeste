---
status: complete
phase: 01-rust-foundation
source: [01-01-SUMMARY.md, 01-02-SUMMARY.md, 01-03-SUMMARY.md]
started: 2026-04-07T00:00:00Z
updated: 2026-04-07T11:35:00Z
---

## Current Test

[testing complete]

## Tests

### 1. Live Database Connection
expected: Server starts and connects to PostgreSQL without errors when DATABASE_URL is set
result: pass

### 2. BCrypt Cross-Platform Compatibility
expected: Login with a password hash created by .NET's BCrypt.Net-Next succeeds in the Rust API
result: pass

### 3. Swagger UI Visual Verification
expected: Swagger UI renders at /swagger-ui/ with all auth and granjas endpoints documented
result: pass

### 4. Role-Based Granjas Filtering
expected: Admin sees all granjas, Produtor sees only own, Financeiro is forbidden from create/update/delete
result: issue
reported: "Admin POST /api/granjas returns 500 'no column found for name: Id'. sqlx(rename = \"Id\") on Granja struct conflicts with SQL aliases (\"Id\" as id). GET returns 200 but empty arrays so mapping error was not triggered until INSERT RETURNING produced a row. Financeiro create correctly blocked with 403."
severity: blocker

## Summary

total: 4
passed: 3
issues: 1
pending: 0
skipped: 0
blocked: 0

## Gaps

- truth: "Admin creates granja and receives created granja object"
  status: failed
  reason: "Admin POST /api/granjas returns 500 'no column found for name: Id'. sqlx(rename) on Granja struct conflicts with SQL column aliases in all queries."
  severity: blocker
  test: 4
  root_cause: "Granja model uses #[sqlx(rename = \"Id\")] etc. which expects PascalCase column names in result sets, but all SQL queries use aliases like '\"Id\" as id' producing lowercase names. The conflict only manifests when a row is actually returned (INSERT RETURNING), not on empty result sets (GET with no data)."
  artifacts:
    - path: "granjatech-api/src/models/granja.rs"
      issue: "#[sqlx(rename = \"Id\")] conflicts with SQL aliases"
    - path: "granjatech-api/src/services/granja_service.rs"
      issue: "All queries use '\"Id\" as id' aliases — redundant with sqlx(rename)"
  missing:
    - "Remove 'as id, as codigo, ...' aliases from all SQL queries in granja_service.rs, OR remove #[sqlx(rename)] from model and keep aliases. Same pattern likely affects auth_service.rs and auditoria_service.rs."
  debug_session: ""
