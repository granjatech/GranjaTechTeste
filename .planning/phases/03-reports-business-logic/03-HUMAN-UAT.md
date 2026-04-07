---
status: partial
phase: 03-reports-business-logic
source: [03-VERIFICATION.md]
started: 2026-04-07T18:00:00Z
updated: 2026-04-07T18:00:00Z
---

## Current Test

[awaiting human testing]

## Tests

### 1. Avicultura Metrics Parity
expected: IEP/CA/GMD numerical output matches .NET backend for same loteId and database state
result: [pending]

### 2. Role Guard Enforcement
expected: Financeiro role receives 403 Forbidden on all avicultura endpoints
result: [pending]

### 3. Report Data Correctness
expected: financeiro-simplificado output matches .NET backend (transacoes, totalEntradas, totalSaidas, saldo)
result: [pending]

### 4. Date Validation
expected: Invalid date ranges (start > end, period > 365 days) return 400 Bad Request
result: [pending]

### 5. Cache Performance
expected: Repeated requests to cached endpoints return faster on second call
result: [pending]

## Summary

total: 5
passed: 0
issues: 0
pending: 5
skipped: 0
blocked: 0

## Gaps
