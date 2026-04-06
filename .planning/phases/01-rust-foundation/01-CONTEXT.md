# Phase 1: Rust Foundation - Context

**Gathered:** 2026-04-06
**Status:** Ready for planning

<domain>
## Phase Boundary

A running Rust backend (Actix-web 4 + SQLx) that connects to the existing PostgreSQL 16 database, authenticates users via JWT (compatible with existing .NET-created BCrypt passwords), and performs full CRUD on Granjas with role-based filtering. Includes all 16 entity structs, all 36 DTOs, unified error handling, Swagger/OpenAPI via utoipa, structured logging via tracing, and CORS middleware.

</domain>

<decisions>
## Implementation Decisions

### Project Structure
- **D-01:** Single crate with flat module layout: `src/{main.rs, config.rs, errors.rs, db.rs, models/, dto/, handlers/, services/, middleware/}`. No Cargo workspace.

### SQLx Query Style
- **D-02:** Runtime queries using `sqlx::query_as::<_, Model>("SELECT ...")` with `.bind()` parameters. No compile-time checked queries — no DATABASE_URL required at build time.

### Naming Language
- **D-03:** Portuguese throughout Rust code, matching DB column names and .NET conventions. Struct fields map 1:1 to DB columns without rename attributes. Variables in Portuguese (e.g., `granja_existente`, `nova_granja`, `senha_hash`).

### Error Handling
- **D-04:** Simple HTTP-mapped `AppError` enum with 5 variants: `NotFound(String)`, `BadRequest(String)`, `Unauthorized(String)`, `Forbidden(String)`, `Internal(String)`. Implements `actix_web::ResponseError` returning JSON `{"message": "..."}`.

### JWT Token Format
- **D-05:** Match .NET JWT claims exactly: `nameid` (user ID), `email`, `role` (profile name). HMAC-SHA256 signing with same secret key. 8-hour expiration. Same `iss` and `aud` values. Existing .NET-issued tokens should work in Rust.

### Service Layer Pattern
- **D-06:** Direct structs with `impl` blocks, no traits. Services are stateless — `PgPool` is passed as parameter. No interfaces or async_trait ceremony. Example: `GranjaService::get_all(pool, user_id, user_role)`.

### API Response Format
- **D-07:** Normalize responses instead of replicating .NET inconsistencies. Use 201 for creates, consistent NotFound format, proper HTTP semantics. The Vue frontend will be built against this cleaner API.

### Configuration
- **D-08:** Single flat `Config` struct loaded from `.env` via `dotenvy` + `std::env::var`. Fields: `database_url`, `jwt_key`, `jwt_issuer`, `jwt_audience`, `allowed_origins`, `swagger_enabled`. Validation on startup (panic on missing required vars).

### Claude's Discretion
- Exact tracing/logging configuration and log levels
- Swagger/utoipa annotation style and grouping
- CORS middleware configuration details
- Internal module organization within models/ and dto/ (one file per entity vs grouped)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan
- `plano-migracao-granjatech.md` — Complete migration plan with target architecture, file layout, technology mapping, and endpoint-by-endpoint conversion guide

### Existing .NET backend (reference implementation)
- `GranjaTech.Api/Program.cs` — Composition root: DI, middleware pipeline, JWT config, CORS, Swagger setup
- `GranjaTech.Api/Controllers/AuthController.cs` — Auth endpoints (login, register, user CRUD)
- `GranjaTech.Api/Controllers/GranjasController.cs` — Granjas CRUD with role-based authorization
- `GranjaTech.Infrastructure/Services/Implementations/AuthService.cs` — JWT generation, BCrypt hashing, user management
- `GranjaTech.Infrastructure/Services/Implementations/GranjaService.cs` — Granjas CRUD with role-based filtering pattern
- `GranjaTech.Infrastructure/GranjaTechDbContext.cs` — Database schema, entity relationships, seed data

### Domain entities
- `GranjaTech.Domain/*.cs` — All 16 entity classes to convert to Rust structs

### DTOs
- `GranjaTech.Application/DTOs/*.cs` — All 36 DTOs to convert to Rust structs

### Configuration
- `GranjaTech.Api/appsettings.json` — JWT settings, connection string format, CORS config
- `.env.example` — Required environment variables

### Database schema
- `docs/banco.sql` — Database creation script (if exists)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Migration plan (`plano-migracao-granjatech.md`) provides complete file-by-file conversion guide with target Rust structure
- Existing .NET entities serve as direct templates for Rust struct field definitions
- Existing .NET DTOs provide exact field lists and validation rules for Rust DTO structs

### Established Patterns
- **Role-based filtering:** Services check `userRole` and filter data accordingly (Administrador sees all, Produtor sees own, Financeiro sees associated producers). This pattern must be replicated in every Rust service.
- **Audit logging:** CUD operations call `_auditoriaService.RegistrarLog()` after successful save. Rust services should follow the same pattern.
- **GetCurrentUser():** Private method extracting (userId, userRole) from JWT claims — Rust middleware extractor should provide equivalent.
- **Controller response pattern:** Ok(data), Ok({message}), NotFound(), BadRequest({message}), NoContent() — normalized per D-07.

### Integration Points
- PostgreSQL 16 database — same tables, same schema, no migrations needed
- BCrypt password hashes — must be cross-compatible between .NET and Rust
- JWT tokens — must use identical claim structure for potential interop

</code_context>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches within the decisions above.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 01-rust-foundation*
*Context gathered: 2026-04-06*
