---
phase: 03-reports-business-logic
plan: 01
subsystem: avicultura-analytics
tags: [rust, actix-web, sqlx, avicultura, analytics, utoipa]
dependency_graph:
  requires: [02-complete-backend-crud]
  provides: [avicultura-service, avicultura-handlers, avicultura-routes]
  affects: [main.rs, handlers/mod.rs, services/mod.rs]
tech_stack:
  added: [rust_decimal_macros]
  patterns: [industry-benchmark-constants, composite-dto-pattern, role-guard-helper]
key_files:
  created:
    - granjatech-api/src/services/avicultura_service.rs
    - granjatech-api/src/handlers/avicultura.rs
  modified:
    - granjatech-api/src/dto/avicultura.rs
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/main.rs
    - granjatech-api/Cargo.toml
decisions:
  - Used rust_decimal_macros for compile-time decimal constants matching .NET PadroesIndustria
  - Implemented Default manually for DTOs containing DateTime<Utc> since chrono does not derive Default
  - Set peso_medio to default MetricaComparacaoDto in comparacao_industria (matching .NET which does not include it in scoring)
metrics:
  duration: 346s
  completed: 2026-04-07
  tasks_completed: 2
  tasks_total: 2
  files_created: 2
  files_modified: 5
---

# Phase 03 Plan 01: Avicultura Analytics Service Summary

AviculturaService with 9 endpoints: 3 real computations (IEP, CA, GMD), 2 real analytics (alertas, comparacao-industria), stub methods for remaining, all with role guards rejecting Financeiro.

## Tasks Completed

| Task | Name | Commit | Key Changes |
|------|------|--------|-------------|
| 1 | Create AviculturaService with computation methods and composite DTOs | 7c8561f | avicultura_service.rs (560 lines), DTOs + Default impls, rust_decimal_macros |
| 2 | Create avicultura handlers with role guards and wire routes + Swagger | 3532dc0 | 9 handlers, route wiring, Swagger paths/schemas/tag |

## Implementation Details

### AviculturaService (services/avicultura_service.rs)

**Real implementations:**
- `calcular_iep` - IEP formula: (ganho_peso_kg * viabilidade * 100) / (CA * idade_dias)
- `calcular_conversao_alimentar` - CA: total_racao / ganho_total_lote_kg
- `calcular_gmd` - GMD: average of weekly weight gain / 7 across pesagem pairs
- `verificar_alertas` - Checks mortality, density, NH3, temperature against industry thresholds
- `comparar_com_industria` - 4-metric scoring (CA, GMD, Viabilidade, IEP) with Excelente/Bom/Abaixo classification

**Stub methods (returning defaults, matching .NET):**
- calcular_viabilidade, calcular_uniformidade, calcular_densidade_atual
- obter_curvas_crescimento, analise_consumo_detalhada, obter_resumo_sanitario
- calcular_projecao_abate, estimar_peso

**Composite methods:**
- `get_metricas` - aggregates all 6 individual metrics
- `get_dashboard` - aggregates metricas + alertas + comparacao + resumo + projecao

### Handlers (handlers/avicultura.rs)

9 GET endpoints under /api/avicultura/{loteId}/:
metricas, analise-consumo, curvas-crescimento, resumo-sanitario, alertas, comparacao-industria, projecao-abate, estimar-peso, dashboard

All protected by `require_admin_or_produtor()` role guard (rejects Financeiro with 403).

## Deviations from Plan

None - plan executed exactly as written.

## Known Stubs

| File | Description | Reason |
|------|-------------|--------|
| avicultura_service.rs | calcular_viabilidade returns 0 | Matches .NET stub (Task.FromResult(0m)) |
| avicultura_service.rs | calcular_uniformidade returns 0 | Matches .NET stub |
| avicultura_service.rs | calcular_densidade_atual returns 0 | Matches .NET stub |
| avicultura_service.rs | obter_curvas_crescimento returns default | Matches .NET stub |
| avicultura_service.rs | analise_consumo_detalhada returns default | Matches .NET stub |
| avicultura_service.rs | obter_resumo_sanitario returns default | Matches .NET stub |
| avicultura_service.rs | calcular_projecao_abate returns default | Matches .NET stub |
| avicultura_service.rs | estimar_peso returns 0 | Matches .NET stub |

All stubs are intentional -- they match the .NET implementation exactly (lines 354-367 of AviculturaService.cs). Parity is maintained.

## Self-Check: PASSED

All 6 files verified present. Both commits (7c8561f, 3532dc0) verified in git log.
