---
status: partial
phase: 02-complete-backend-crud
source: [02-VERIFICATION.md]
started: 2026-04-07T00:00:00Z
updated: 2026-04-07T00:00:00Z
---

## Current Test

[awaiting human testing]

## Tests

### 1. End-to-End HTTP Response Parity
expected: Start both backends (.NET and Rust) against the same PostgreSQL database. Compare JSON responses for all 37 endpoints — field names, types, nesting, and HTTP status codes should match.
result: [pending]

### 2. 5-Minute Edit Window (Financas)
expected: Create a financial transaction, wait 5 minutes, then attempt to update as a non-admin user. The Rust backend should return 400 with the time-window-expired error, matching .NET behavior.
result: [pending]

### 3. Public Leitura Endpoint
expected: POST /api/leituras with a valid sensor identificador_unico and reading data, without any JWT token. Should return 201 Created, matching .NET behavior for IoT device ingestion.
result: [pending]

### 4. Mortalidade Clamping
expected: Register mortalidade with a quantity exceeding QuantidadeAvesAtual. The Rust backend should clamp the value to the remaining birds (not reject), matching .NET behavior.
result: [pending]

## Summary

total: 4
passed: 0
issues: 0
pending: 4
skipped: 0
blocked: 0

## Gaps
