---
phase: 03-reports-business-logic
plan: 03
subsystem: api
tags: [moka, cache, actix-web, rust, health-check]

# Dependency graph
requires:
  - phase: 03-reports-business-logic/01
    provides: Avicultura service and handlers
  - phase: 03-reports-business-logic/02
    provides: Report service and handlers
provides:
  - CacheService with dual TTL tiers (5min/10min) wrapping moka
  - Cache integration for dashboard, avicultura, and report endpoints
  - Enhanced /health endpoint with service name and timestamp
affects: [04-frontend, 06-docker]

# Tech tracking
tech-stack:
  added: [moka 0.12 with future feature]
  patterns: [dual-tier cache (short 5min, long 10min), user-scoped cache keys for role-filtered data, get_or_set pattern for transparent caching]

key-files:
  created:
    - granjatech-api/src/services/cache_service.rs
  modified:
    - granjatech-api/Cargo.toml
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/main.rs
    - granjatech-api/src/handlers/dashboard.rs
    - granjatech-api/src/handlers/avicultura.rs
    - granjatech-api/src/handlers/relatorios.rs
    - granjatech-api/src/dto/dashboard.rs
    - granjatech-api/src/dto/avicultura.rs
    - granjatech-api/src/dto/relatorios.rs
    - granjatech-api/src/dto/financeiro.rs
    - granjatech-api/src/dto/lote.rs

key-decisions:
  - "Dual moka Cache instances for two TTL tiers instead of per-entry TTL"
  - "Cache keys include user_id and role for role-filtered endpoints to prevent cross-user data leakage"
  - "JSON serialization for cache values (serde_json round-trip through String)"
  - "Added Deserialize to all cached DTOs for JSON cache deserialization"

patterns-established:
  - "Cache integration pattern: handler receives web::Data<CacheService>, builds cache_key with user context, calls get_or_set"
  - "Short TTL (5min) for dashboards, long TTL (10min) for heavy report endpoints"

requirements-completed: [RELA-01, CACH-01]

# Metrics
duration: 10min
completed: 2026-04-07
---

# Phase 3 Plan 3: CacheService and Health Check Summary

**moka-based CacheService with dual TTL tiers (5min/10min) integrated into dashboard, avicultura, and report handlers with user-scoped cache keys**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-07T17:20:11Z
- **Completed:** 2026-04-07T17:30:31Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments
- Created CacheService with 5 public methods (get, set, remove, remove_by_pattern, get_or_set) wrapping moka::future::Cache
- Integrated caching into 7 handler functions across dashboard, avicultura, and report modules
- Enhanced /health endpoint to return service name and timestamp alongside status
- Added Deserialize derive to all DTOs used with caching for JSON round-trip support

## Task Commits

Each task was committed atomically:

1. **Task 1: Add moka dependency and create CacheService with 5-method API** - `5848304` (feat)
2. **Task 2: Wire CacheService into main.rs, enhance health check, integrate cache into handlers** - `916aee6` (feat)

## Files Created/Modified
- `granjatech-api/src/services/cache_service.rs` - CacheService with dual TTL tiers and 5-method API
- `granjatech-api/Cargo.toml` - Added moka 0.12 dependency with future feature
- `granjatech-api/src/services/mod.rs` - Registered cache_service module
- `granjatech-api/src/main.rs` - Wired CacheService as app state, enhanced /health endpoint
- `granjatech-api/src/handlers/dashboard.rs` - Added cache to get_kpis and get_resumo_mensal (5min TTL)
- `granjatech-api/src/handlers/avicultura.rs` - Added cache to get_dashboard (5min TTL)
- `granjatech-api/src/handlers/relatorios.rs` - Added cache to 5 report handlers (10min TTL)
- `granjatech-api/src/dto/dashboard.rs` - Added Deserialize to DashboardKpiDto, MonthlySummaryDto
- `granjatech-api/src/dto/avicultura.rs` - Added Deserialize to all dashboard-related DTOs
- `granjatech-api/src/dto/relatorios.rs` - Added Deserialize to all report DTOs
- `granjatech-api/src/dto/financeiro.rs` - Added Deserialize to report response DTOs
- `granjatech-api/src/dto/lote.rs` - Added Deserialize to LoteResponseDto (used in RelatorioProducaoDto)

## Decisions Made
- Used dual moka Cache instances (short_cache 5min, long_cache 10min) rather than per-entry TTL since moka applies TTL at cache-instance level
- Cache keys include user_id and role for role-filtered endpoints (dashboard, financeiro reports) to prevent cross-user data leakage (T-03-08)
- Cache values stored as JSON strings via serde_json, requiring Deserialize on all cached DTOs
- SetorReportDto kept without Deserialize since it's a generic struct not directly cached

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added Deserialize to LoteResponseDto**
- **Found during:** Task 2 (cache integration for report handlers)
- **Issue:** RelatorioProducaoDto contains Vec<LoteResponseDto> which needs Deserialize for cache round-trip, but LoteResponseDto only had Serialize
- **Fix:** Added Deserialize derive to LoteResponseDto in dto/lote.rs
- **Files modified:** granjatech-api/src/dto/lote.rs
- **Verification:** cargo check passes
- **Committed in:** 916aee6 (Task 2 commit)

**2. [Rule 2 - Missing Critical] Added Deserialize to financeiro report DTOs**
- **Found during:** Task 2 (cache integration for report handlers)
- **Issue:** RelatorioFinanceiroDto and RelatorioFinanceiroSimplificadoDto contain TransacaoSimplificadaDto which needed Deserialize
- **Fix:** Added Deserialize derive to all Serialize-only DTOs in financeiro.rs
- **Files modified:** granjatech-api/src/dto/financeiro.rs
- **Verification:** cargo check passes
- **Committed in:** 916aee6 (Task 2 commit)

**3. [Rule 3 - Blocking] Reverted Deserialize from generic SetorReportDto**
- **Found during:** Task 2 (adding Deserialize globally to relatorios DTOs)
- **Issue:** SetorReportDto has a generic type parameter bounded on Serialize; adding Deserialize required complex lifetime bounds
- **Fix:** Kept SetorReportDto as Serialize-only since it's not directly cached
- **Files modified:** granjatech-api/src/dto/relatorios.rs
- **Verification:** cargo check passes
- **Committed in:** 916aee6 (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (2 missing critical, 1 blocking)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
- Build environment in worktree has broken `libbfd-2.42-system.so` preventing native compilation of zstd-sys; resolved by using shared CARGO_TARGET_DIR from main repo which already had compiled artifacts

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All Phase 3 plans complete (avicultura service, report endpoints, cache layer)
- Backend API feature-complete for reports and business logic
- Ready for Phase 4 (frontend) or Phase 5/6 (infrastructure)

## Self-Check: PASSED

All files exist, all commits verified.

---
*Phase: 03-reports-business-logic*
*Completed: 2026-04-07*
