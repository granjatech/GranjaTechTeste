# Phase 2: Complete Backend CRUD - Context

**Gathered:** 2026-04-07
**Status:** Ready for planning

<domain>
## Phase Boundary

Every remaining CRUD endpoint from the original .NET backend responds identically in the Rust backend. Covers Lotes (with mortalidade and calculated properties), Dashboard KPIs/monthly summary, Financas, Consumo, Pesagem, Sanitario, Sensores, Estoque, Auditoria, and Profile endpoints.

</domain>

<decisions>
## Implementation Decisions

### Plan Grouping
- **D-01:** Split into 3 plans by complexity:
  - Plan 1: Lotes (complex, calculated properties, mortalidade) + Dashboard (aggregation queries)
  - Plan 2: Financas + Consumo + Pesagem + Sanitario (standard CRUD, lote-dependent)
  - Plan 3: Sensores + Estoque + Auditoria + Profile (simple CRUD, read-only, independent)

### Lote Calculated Properties
- **D-02:** IEP, conversao alimentar, and viabilidade computed in Rust code after fetching raw data, matching the .NET pattern where `Lote` entity has calculated properties (`CalcularIEP()`, `Viabilidade` getter). Not computed in SQL.

### Audit Logging
- **D-03:** Explicit `AuditoriaService::registrar_log()` calls in each service method after CUD operations, same as Phase 1 `GranjaService` pattern. No abstraction layer or auto-logging middleware.

### Carried Forward from Phase 1
- **D-04:** All Phase 1 decisions (D-01 through D-08) carry forward unchanged:
  - Single crate, flat module layout
  - Runtime SQLx queries with `query_as` and `.bind()`
  - Portuguese naming matching DB columns
  - Simple `AppError` enum with 5 variants
  - JWT claims matching .NET format (nameid, email, role)
  - Stateless service structs, `PgPool` passed as parameter
  - Normalized API responses (201 for creates, proper HTTP semantics)
  - `.env` config via dotenvy

### Claude's Discretion
- Internal service method organization within each file
- Query optimization for Dashboard aggregation endpoints
- DTO field ordering and grouping within struct definitions
- Handler grouping in `main.rs` route configuration

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan
- `plano-migracao-granjatech.md` -- Complete migration plan with endpoint-by-endpoint conversion guide

### Phase 1 Rust code (established patterns to follow)
- `granjatech-api/src/services/granja_service.rs` -- Service pattern template (role filtering, audit logging, error handling)
- `granjatech-api/src/handlers/granjas.rs` -- Handler pattern template (utoipa annotations, Claims extractor, response format)
- `granjatech-api/src/services/auditoria_service.rs` -- Audit logging service to call from all CUD operations
- `granjatech-api/src/middleware/jwt.rs` -- Claims middleware extractor
- `granjatech-api/src/errors.rs` -- AppError enum

### .NET reference implementation (source of truth for behavior)
- `GranjaTech.Api/Controllers/LotesController.cs` -- Lotes endpoints including mortalidade
- `GranjaTech.Api/Controllers/DashboardController.cs` -- Dashboard KPIs and monthly summary
- `GranjaTech.Api/Controllers/FinancasController.cs` -- Financial transactions CRUD
- `GranjaTech.Api/Controllers/ConsumoController.cs` -- Feed/water consumption endpoints
- `GranjaTech.Api/Controllers/PesagemController.cs` -- Weighing endpoints
- `GranjaTech.Api/Controllers/SanitarioController.cs` -- Sanitary events and vaccination schedule
- `GranjaTech.Api/Controllers/SensoresController.cs` -- Sensors CRUD
- `GranjaTech.Api/Controllers/LeiturasController.cs` -- Sensor readings
- `GranjaTech.Api/Controllers/EstoqueController.cs` -- Stock products CRUD
- `GranjaTech.Api/Controllers/AuditoriaController.cs` -- Audit logs read-only
- `GranjaTech.Api/Controllers/ProfileController.cs` -- Profile view/edit and password change
- `GranjaTech.Infrastructure/Services/Implementations/LoteService.cs` -- Lote business logic, calculated properties
- `GranjaTech.Infrastructure/Services/Implementations/DashboardService.cs` -- Dashboard aggregation queries
- `GranjaTech.Infrastructure/Services/Implementations/FinancasService.cs` -- Financial service
- `GranjaTech.Infrastructure/Services/Implementations/SensorService.cs` -- Sensor service
- `GranjaTech.Infrastructure/Services/Implementations/EstoqueService.cs` -- Stock service
- `GranjaTech.Infrastructure/Services/Implementations/AuditoriaService.cs` -- Audit service (.NET version)
- `GranjaTech.Domain/Lote.cs` -- Lote entity with calculated properties (CalcularIEP, Viabilidade, ConversaoAlimentar)

### Existing Rust DTOs (already created in Phase 1)
- `granjatech-api/src/dto/` -- All 14 DTO modules already exist (lote, dashboard, financeiro, consumo, pesagem, sanitario, sensor, estoque, profile, avicultura, relatorios)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `GranjaService` + handler: Complete template for service + handler pattern (role filtering, audit logging, utoipa annotations)
- `AuditoriaService`: Already implemented in Rust — ready to call from new services
- All 16 entity models: Already defined in `granjatech-api/src/models/`
- All DTOs: Already defined in `granjatech-api/src/dto/` (14 domain modules)
- `Claims` middleware extractor: Provides `user_id()` and `role` for all handlers
- `AppError`: Unified error type with `From<sqlx::Error>` impl

### Established Patterns
- **Service pattern:** `pub struct XService; impl XService { pub async fn get_all(pool, user_id, user_role) -> Result<Vec<X>, AppError> }`
- **Handler pattern:** `pub async fn get_x(pool: web::Data<PgPool>, claims: Claims, ...) -> Result<HttpResponse, AppError>`
- **Role filtering:** `match user_role { "Administrador" => ..., "Produtor" => ..., "Financeiro" => ... }`
- **Audit logging:** `AuditoriaService::registrar_log(pool, user_id, "Acao", "Detalhes").await?;`
- **Query style:** `sqlx::query_as::<_, Model>("SQL").bind(param).fetch_all/one/optional(pool).await?`

### Integration Points
- `granjatech-api/src/main.rs` -- Route registration (add new handler routes)
- `granjatech-api/src/services/mod.rs` -- Service module declarations
- `granjatech-api/src/handlers/mod.rs` -- Handler module declarations
- Swagger/utoipa: Add new handlers to `#[openapi(paths(...))]` in main.rs

</code_context>

<specifics>
## Specific Ideas

No specific requirements -- follow Phase 1 patterns exactly, replicating .NET behavior in Rust.

</specifics>

<deferred>
## Deferred Ideas

None -- discussion stayed within phase scope.

</deferred>

---

*Phase: 02-complete-backend-crud*
*Context gathered: 2026-04-07*
