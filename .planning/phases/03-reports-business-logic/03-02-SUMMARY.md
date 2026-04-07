---
phase: 03-reports-business-logic
plan: 02
subsystem: reports
tags: [rust, actix-web, sqlx, reports, relatorios]
dependency_graph:
  requires: [02-CRUD-services, 02-handlers, 02-DTOs]
  provides: [relatorio-service, relatorio-avancado-service, report-handlers, report-routes]
  affects: [main.rs-swagger, handlers-mod-routes]
tech_stack:
  added: []
  patterns: [multi-query-report-aggregation, role-based-query-filtering, date-range-validation]
key_files:
  created:
    - granjatech-api/src/services/relatorio_service.rs
    - granjatech-api/src/services/relatorio_avancado_service.rs
    - granjatech-api/src/handlers/relatorios.rs
  modified:
    - granjatech-api/src/dto/relatorios.rs
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/main.rs
decisions:
  - Used separate SQL queries per lote in avicultura report (matching .NET Include pattern with D-07 multi-query approach)
  - Computed CA and IEP inline in Rust matching .NET CalcularConversaoAlimentar and CalcularIEP formulas
  - ParametrosAceitaveis logic replicated as NH3<=25, temp 18-33, CO2<=3000
metrics:
  duration: 648s
  completed: "2026-04-07"
  tasks_completed: 2
  tasks_total: 2
  files_created: 3
  files_modified: 4
---

# Phase 03 Plan 02: Report Endpoints Summary

All 6 report endpoints implemented with role-based filtering, date validation, and multi-query aggregation patterns mirroring .NET RelatorioService and RelatorioAvancadoService behavior.

## Completed Tasks

| Task | Name | Commit | Key Files |
|------|------|--------|-----------|
| 1 | RelatorioService + RelatorioAvancadoService + DTOs | 0da7e63 | relatorio_service.rs, relatorio_avancado_service.rs, dto/relatorios.rs |
| 2 | Handlers, routes, and Swagger wiring | af79b72 | handlers/relatorios.rs, handlers/mod.rs, main.rs |

## What Was Built

### RelatorioService (5 methods)
- `financeiro_simplificado` - Simplified financial report with role-filtered transactions (RELA-02)
- `financeiro` - Full financial report merging lote + general transactions by date DESC (RELA-03)
- `producao` - Production report with lotes filtered by DataEntrada range (RELA-04)
- `avicultura` - Avicultura report with resumoGeral, detalhesPorLote, benchmarks (RELA-05)
- `desempenho_lote` - Detailed lote performance with curvaCrescimento, consumoRacao, qualidadeAmbiental (RELA-06)

### RelatorioAvancadoService (6 methods)
- `financeiro` - Advanced financial report by granja (RELA-07)
- `geral` - General report with consumo, pesagens, sanitario, sensores aggregated by day (RELA-07)
- `consumo` - Sector report: consumo racao + agua grouped by day (RELA-07)
- `pesagem` - Sector report: pesagens with kg conversion (RELA-07)
- `sanitario` - Sector report: eventos sanitarios (RELA-07)
- `sensores` - Sector report: sensor readings via LeituraSensor JOIN Sensor (RELA-07)

### Report DTOs Added
- RelatorioAviculturaDto, ResumoGeralAviculturaDto, DetalheLoteAviculturaDto, BenchmarksAviculturaDto
- RelatorioDesempenhoLoteDto, PerformanceLoteDto, CurvaCrescimentoItemDto
- ConsumoRacaoGroupDto, ConsumoRacaoDiaDto, ConsumoAguaItemDto
- HistoricoSanitarioItemDto, AnaliseMortalidadeItemDto, QualidadeAmbientalItemDto

### Handlers (6 endpoints)
- GET /api/relatorios/financeiro-simplificado
- GET /api/relatorios/financeiro
- GET /api/relatorios/producao
- GET /api/relatorios/avicultura
- GET /api/relatorios/desempenho-lote/{loteId}
- GET /api/relatorios/avancado

### Security Implementation
- All handlers require JWT authentication (Claims extractor)
- Role-based query filtering: Administrador sees all, Produtor sees own granjas, Financeiro sees linked produtores
- Date validation: inicio < fim, period <= 365 days
- LIMIT 1000 on all list queries (T-03-06 DoS mitigation)
- SQLx parameterized queries prevent injection (T-03-05)
- Granja existence validation on avancado endpoint (T-03-07)

## Deviations from Plan

None - plan executed exactly as written.

## Decisions Made

1. **Multi-query per lote in avicultura**: Each lote gets separate queries for consumo_racao, consumo_agua, pesagens, eventos_sanitarios -- matching the .NET Include() pattern but using D-07 sequential queries approach
2. **CA formula**: `totalRacaoKg / ((pesoRecente - 45g) * avesAtuais / 1000)` matching .NET CalcularConversaoAlimentar
3. **IEP formula**: `(ganhoPesoKg * viabilidade * 100) / (CA * idadeDias)` matching .NET CalcularIEP
4. **ParametrosAceitaveis**: NH3 <= 25 ppm AND temperature 18-33C AND CO2 <= 3000 ppm

## Verification

- `cargo check` passes with zero errors (warnings only for unused fields in helper structs)
- All 6 handler functions present with utoipa annotations
- Routes registered under /api/relatorios/ scope
- Swagger schemas and tag added to main.rs

## Self-Check: PASSED

All 8 files verified present. Both commits (0da7e63, af79b72) verified in git log.
