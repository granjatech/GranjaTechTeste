---
phase: 02-complete-backend-crud
plan: 01
subsystem: api
tags: [rust, actix-web, sqlx, lotes, dashboard, crud, mortalidade, kpis]

requires:
  - phase: 01-rust-foundation
    provides: "Actix-web project skeleton, models, DTOs, JWT middleware, GranjaService pattern, AuditoriaService"
provides:
  - "LoteService with full CRUD, mortalidade registration, computed properties"
  - "DashboardService with KPI aggregation and monthly summary"
  - "9 HTTP endpoints: 7 lotes + 2 dashboard"
  - "Swagger UI entries for lotes and dashboard tags"
affects: [03-remaining-endpoints, frontend-lotes, frontend-dashboard]

tech-stack:
  added: [sqlx rust_decimal feature]
  patterns: [role-filtered SQL queries, computed properties in service layer, verificar_acesso_granja helper, quantity clamping for mortalidade]

key-files:
  created:
    - granjatech-api/src/services/lote_service.rs
    - granjatech-api/src/services/dashboard_service.rs
    - granjatech-api/src/handlers/lotes.rs
    - granjatech-api/src/handlers/dashboard.rs
  modified:
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/main.rs
    - granjatech-api/Cargo.toml

key-decisions:
  - "Added rust_decimal feature to sqlx to support Decimal type in query_as (pre-existing models used rust_decimal but sqlx lacked the feature)"
  - "Extracted verificar_acesso_granja helper to avoid duplicating role-check logic across every LoteService method"
  - "Used EXTRACT(YEAR/MONTH) with f64 return type for PostgreSQL monthly aggregation (EXTRACT returns float8)"

patterns-established:
  - "verificar_acesso_granja: reusable role-based granja access check for any entity linked to a granja"
  - "map_lote_to_response: private function computing idade_atual_dias, viabilidade, densidade_atual from raw Lote"
  - "Mortalidade clamping: dto.quantidade.min(lote.quantidade_aves_atual) to prevent negative bird counts"
  - "pt-BR month formatting via hardcoded MESES_PT_BR array with format jan/26"

requirements-completed: [LOTE-01, LOTE-02, LOTE-03, LOTE-04, LOTE-05, LOTE-06, LOTE-07, LOTE-08, DASH-01, DASH-02]

duration: 5min
completed: 2026-04-07
---

# Phase 02 Plan 01: Lotes/Dashboard Summary

**Lote CRUD with mortalidade registration, computed properties (viabilidade, densidade, idade), and Dashboard KPIs/monthly summary with role-filtered aggregation and pt-BR formatting**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-07T12:47:54Z
- **Completed:** 2026-04-07T12:53:47Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- LoteService with 7 methods: get_all, get_by_id, create, update, delete, registrar_mortalidade, listar_mortalidades -- all with role-based access control
- DashboardService with get_kpis (financial aggregation + active lotes count) and get_resumo_mensal (monthly summary with pt-BR formatting)
- 9 HTTP endpoints registered with utoipa Swagger annotations, route scopes, and schema registrations
- Computed properties (idade_atual_dias, viabilidade, densidade_atual) calculated in service layer, mortalidade quantity clamped to prevent negative bird counts

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement LoteService and DashboardService** - `48d8b75` (feat)
2. **Task 2: Implement handlers, routes, and Swagger** - `2269728` (feat)

## Files Created/Modified
- `granjatech-api/src/services/lote_service.rs` - LoteService with full CRUD, mortalidade, computed properties, role filtering
- `granjatech-api/src/services/dashboard_service.rs` - DashboardService with KPI aggregation and monthly summary
- `granjatech-api/src/handlers/lotes.rs` - 7 HTTP handlers with utoipa annotations
- `granjatech-api/src/handlers/dashboard.rs` - 2 HTTP handlers with utoipa annotations
- `granjatech-api/src/services/mod.rs` - Added lote_service and dashboard_service modules
- `granjatech-api/src/handlers/mod.rs` - Added lotes/dashboard modules and route registration
- `granjatech-api/src/main.rs` - Added 9 paths, 7 schemas, 2 tags to OpenApi derive
- `granjatech-api/Cargo.toml` - Added rust_decimal feature to sqlx dependency

## Decisions Made
- Added `rust_decimal` feature to sqlx Cargo dependency because existing Lote model uses `rust_decimal::Decimal` for AreaGalpao but sqlx only had `bigdecimal` feature enabled -- this was a blocking issue (Rule 3)
- Extracted `verificar_acesso_granja` as a private helper method in LoteService to centralize role-based granja ownership checks, reducing duplication across 7 service methods
- Used `f64` for PostgreSQL EXTRACT return type in MonthlyRaw struct since EXTRACT returns float8 in PostgreSQL

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added rust_decimal feature to sqlx**
- **Found during:** Task 1 (LoteService implementation)
- **Issue:** Lote model uses `rust_decimal::Decimal` for `area_galpao` field, but sqlx dependency only had `bigdecimal` feature -- `Decimal: sqlx::Decode` trait bound not satisfied
- **Fix:** Added `rust_decimal` to sqlx features in Cargo.toml
- **Files modified:** granjatech-api/Cargo.toml
- **Verification:** cargo check passes
- **Committed in:** 48d8b75 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential fix for compilation. No scope creep.

## Issues Encountered
- System-level `libbfd-2.42-system.so` missing from default library path -- required setting `LD_LIBRARY_PATH` to gcc-toolchain location for cargo build/check. This is a pre-existing environment issue, not related to plan changes.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Lote and Dashboard endpoints ready for integration testing
- Pattern established for remaining CRUD services (Financas, Consumo, Pesagem, Sanitario, etc.)
- verificar_acesso_granja helper can be reused by any future service that needs granja-level access control

---
*Phase: 02-complete-backend-crud*
*Completed: 2026-04-07*
