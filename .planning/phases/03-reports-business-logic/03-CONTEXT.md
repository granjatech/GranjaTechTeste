# Phase 3: Reports & Business Logic - Context

**Gathered:** 2026-04-07
**Status:** Ready for planning

<domain>
## Phase Boundary

All advanced analytics (avicultura module) and report endpoints producing results identical to the .NET backend. Includes 9 avicultura endpoints (metrics, growth curves, alerts, industry comparison, slaughter projection, full dashboard), 6 report endpoints (simplified financial, full financial, production, aviculture, lote performance, advanced with filters), health check endpoint, and moka cache layer for heavy endpoints.

</domain>

<decisions>
## Implementation Decisions

### Plan Grouping
- **D-01:** Split into 3 plans by domain:
  - Plan 1: Avicultura service (9 endpoints — metricas, analise-consumo, curvas-crescimento, resumo-sanitario, alertas, comparacao-industria, projecao-abate, estimar-peso, dashboard)
  - Plan 2: Reports — RelatorioService + RelatorioAvancadoService (6 report endpoints: financeiro-simplificado, financeiro, producao, avicultura, desempenho-lote, avancado)
  - Plan 3: Cache layer (moka) + Health check endpoint + wiring cache into dashboard and report endpoints

### Avicultura Computation
- **D-02:** Mirror .NET individual compute method pattern — keep separate service methods (CalcularIEP, CalcularConversaoAlimentar, etc.) that each make their own DB queries. Simpler to verify parity with .NET. Cache in Plan 3 handles performance.
- **D-03:** Industry benchmark values hardcoded as Rust constants, same as .NET. No config file or external data source.

### Cache Strategy
- **D-04:** Full 5-method CacheService API wrapping `moka::future::Cache`: get, set, remove, remove_by_pattern, get_or_set. Matches .NET ICacheService interface.
- **D-05:** Cache applied to dashboard KPIs (5 min TTL), avicultura dashboard (5 min TTL), and heavy report endpoints (10 min TTL).
- **D-06:** TTL-only expiration — no write-through invalidation. Cache entries expire naturally. No modifications to existing Phase 2 CRUD services.

### Report Query Approach
- **D-07:** Multiple queries per report (fetch base entity, then related data in separate queries). No large JOINs with manual mapping. Matches the logical structure of .NET's EF Core .Include() pattern.
- **D-08:** Health check only at `/health` endpoint. Skip .NET debug endpoints (debug/memory, debug/test-basic, debug/simple) — they are dev utilities, not production features.

### Role-Based Access
- **D-09:** Exact .NET role parity: Avicultura endpoints restricted to Administrador+Produtor. Report endpoints allow all 3 roles (Administrador, Produtor, Financeiro). No data filtering in reports. No ownership checks on avicultura loteId queries.

### DTO Completeness
- **D-10:** Create typed DTOs for all composite responses (MetricasLoteDto, DashboardAviculturaDto) instead of using serde_json::json!(). Better Swagger documentation via utoipa.

### Error Response Format
- **D-11:** Normalize all errors to existing AppError enum (returns `{"message": "..."}`), consistent with Phase 1 D-04. Do not replicate .NET's inconsistent `{message, error}` format from AviculturaController.

### Carried Forward from Phases 1-2
- **D-12:** All Phase 1 decisions (D-01 through D-08) and Phase 2 decisions (D-01 through D-04) carry forward unchanged:
  - Single crate, flat module layout
  - Runtime SQLx queries with `query_as` and `.bind()`
  - Portuguese naming matching DB columns
  - Simple `AppError` enum with 5 variants
  - Stateless service structs, `PgPool` passed as parameter
  - Normalized API responses
  - Explicit `AuditoriaService::registrar_log()` calls after CUD operations

### Claude's Discretion
- Internal query optimization within the multiple-query-per-report approach
- Moka cache configuration details (max capacity, eviction policy)
- Avicultura service method ordering and grouping
- Report DTO field ordering and naming for anonymous .NET objects

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan
- `plano-migracao-granjatech.md` — Complete migration plan with endpoint-by-endpoint conversion guide

### .NET reference implementation (source of truth for behavior)
- `GranjaTech.Api/Controllers/AviculturaController.cs` — 9 avicultura endpoints with role restriction (Administrador+Produtor)
- `GranjaTech.Api/Controllers/RelatoriosController.cs` — Report endpoints + health check + advanced reports
- `GranjaTech.Infrastructure/Services/Implementations/AviculturaService.cs` — Complex avicultura business logic (metrics, growth curves, alerts, industry comparison, slaughter projection)
- `GranjaTech.Infrastructure/Services/Implementations/RelatorioService.cs` — Financial and production report queries
- `GranjaTech.Infrastructure/Services/Implementations/RelatorioAvancadoService.cs` — Advanced reports (financeiro, geral, setor)
- `GranjaTech.Infrastructure/Services/Implementations/MemoryCacheService.cs` — .NET cache implementation (reference for API design)
- `GranjaTech.Infrastructure/Services/Interfaces/ICacheService.cs` — Cache interface (5 methods: get, set, remove, removeByPattern, getOrSet)
- `GranjaTech.Domain/Lote.cs` — Lote entity with calculated properties (CalcularIEP, Viabilidade, ConversaoAlimentar) used by avicultura

### Existing Rust code (patterns to follow)
- `granjatech-api/src/services/granja_service.rs` — Service pattern template
- `granjatech-api/src/services/dashboard_service.rs` — Aggregation query patterns
- `granjatech-api/src/handlers/dashboard.rs` — Handler pattern for aggregation endpoints
- `granjatech-api/src/services/lote_service.rs` — Lote service with calculated properties (IEP, CA, viabilidade)
- `granjatech-api/src/dto/avicultura.rs` — 14 avicultura DTOs already created
- `granjatech-api/src/dto/relatorios.rs` — 8 report DTOs already created
- `granjatech-api/src/errors.rs` — AppError enum for normalized error responses
- `granjatech-api/src/main.rs` — Route registration (add new handler routes + Swagger paths)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- All avicultura DTOs (14 structs in `dto/avicultura.rs`) — ready to use for service responses
- All report DTOs (8 structs in `dto/relatorios.rs`) — ready to use for report endpoints
- `LoteService` with `calcular_iep()`, `calcular_conversao_alimentar()`, `viabilidade()` — can be called from avicultura service
- `DashboardService` — aggregation query patterns reusable for report queries
- `AuditoriaService::registrar_log()` — audit logging for any CUD operations

### Established Patterns
- **Service pattern:** `pub struct XService; impl XService { pub async fn method(pool, ...) -> Result<T, AppError> }`
- **Handler pattern:** `pub async fn handler(pool: web::Data<PgPool>, claims: Claims, ...) -> Result<HttpResponse, AppError>`
- **Query style:** `sqlx::query_as::<_, Model>("SQL").bind(param).fetch_all/one/optional(pool).await?`
- **Role filtering:** `match user_role { "Administrador" => ..., "Produtor" => ..., "Financeiro" => ... }`

### Integration Points
- `granjatech-api/src/main.rs` — Route registration for new avicultura + report handlers
- `granjatech-api/src/services/mod.rs` — New service module declarations
- `granjatech-api/src/handlers/mod.rs` — New handler module declarations
- Swagger/utoipa: Add new handlers to `#[openapi(paths(...))]` in main.rs
- Moka cache: Add as `web::Data<CacheService>` in app state (main.rs)

</code_context>

<specifics>
## Specific Ideas

No specific requirements — follow .NET behavior exactly, replicating all avicultura analytics and report logic in Rust with the established patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 03-reports-business-logic*
*Context gathered: 2026-04-07*
