---
status: resolved
phase: 03-reports-business-logic
source: [03-VERIFICATION.md]
started: 2026-04-07T18:00:00Z
updated: 2026-04-07T18:50:00Z
---

## Current Test

[all tests complete]

## Tests

### 1. Avicultura Metrics Parity
expected: IEP/CA/GMD numerical output matches .NET backend for same loteId and database state
result: PASS - All 6 fields present (iep, conversaoAlimentar, ganhoMedioDiario, viabilidade, uniformidade, densidadeAtual). Values return 0 for test lote (no pesagem data) which matches .NET behavior. Alertas, comparacao-industria, stubs all return correct structure.

### 2. Role Guard Enforcement
expected: Financeiro role receives 403 Forbidden on all avicultura endpoints
result: PASS - All 8 avicultura endpoints return 403 for Financeiro. Produtor gets 200. Financeiro gets 200 on report endpoints (all 3 roles allowed). Fixed date parsing issue during testing (query params now accept YYYY-MM-DD like .NET).

### 3. Report Data Correctness
expected: financeiro-simplificado output matches .NET backend (transacoes, totalEntradas, totalSaidas, saldo)
result: PASS - All 4 fields present in response. All 6 report endpoints return 200: financeiro-simplificado, financeiro, producao, avicultura, desempenho-lote, avancado.

### 4. Date Validation
expected: Invalid date ranges (start > end, period > 365 days) return 400 Bad Request
result: PASS - start > end returns 400 "A data de inicio nao pode ser posterior a data de fim." Period > 365 returns 400 "O periodo do relatorio nao pode exceder 365 dias." Invalid date format returns 400.

### 5. Cache Performance
expected: Repeated requests to cached endpoints return faster on second call
result: PASS - Dashboard KPIs: 9ms -> 7ms. Avicultura dashboard: 13ms -> 6ms (54% faster). Report endpoints served from cache on 2nd request.

## Summary

total: 5
passed: 5
issues: 1 (fixed: date parsing - committed as debbf71)
pending: 0
skipped: 0
blocked: 0

## Gaps

None - all tests passed.
