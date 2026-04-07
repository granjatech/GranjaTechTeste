---
phase: 01-rust-foundation
plan: 03
subsystem: backend-services-handlers
tags: [rust, actix-web, services, handlers, cors, swagger, auth, bcrypt, jwt]
dependency_graph:
  requires: [01-01, 01-02]
  provides: [auth-service, granja-service, auditoria-service, http-handlers, route-wiring, cors, swagger-ui]
  affects: [main.rs]
tech_stack:
  added: [actix-cors, utoipa-swagger-ui]
  patterns: [stateless-service-structs, parameterized-sqlx-queries, role-based-access-control, bcrypt-verify-hash, jwt-hs256-generation]
key_files:
  created:
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/services/auth_service.rs
    - granjatech-api/src/services/granja_service.rs
    - granjatech-api/src/services/auditoria_service.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/handlers/auth.rs
    - granjatech-api/src/handlers/granjas.rs
  modified:
    - granjatech-api/src/main.rs
decisions:
  - "Services as stateless structs with PgPool passed as parameter (per D-06)"
  - "SQL queries use column aliases to map PascalCase DB to snake_case structs (runtime queries per D-02)"
  - "CORS uses explicit allowed_origin() calls, not allow_any_origin() (security T-1-05)"
  - "Swagger UI conditionally enabled via SWAGGER_ENABLED env var"
  - "POST create endpoints return 201 Created (per D-07)"
  - "DELETE endpoints return 204 No Content (per D-07)"
metrics:
  duration: "5m 16s"
  completed: "2026-04-07T11:11:57Z"
  tasks: 3
  files_created: 7
  files_modified: 1
---

# Phase 01 Plan 03: Services, Handlers, CORS, and Swagger Summary

Auth service with BCrypt-compatible login + JWT HS256 generation, granja service with 3-role RBAC filtering, audit logging, 11 HTTP handlers with utoipa annotations, CORS with explicit origins, and conditional Swagger UI.

## Task Completion

| Task | Name | Commit | Key Files |
|------|------|--------|-----------|
| 1 | Create auth, granja, and auditoria services | 7884862 | services/auth_service.rs, services/granja_service.rs, services/auditoria_service.rs |
| 2 | Create HTTP handlers with route registration | 6af0d8c | handlers/auth.rs, handlers/granjas.rs, handlers/mod.rs |
| 3 | Wire CORS, Swagger UI, finalize main.rs | 682d6dd | main.rs |

## What Was Built

### AuthService (auth_service.rs)
- **login**: Queries Usuarios JOIN Perfis by email, verifies BCrypt hash (compatible with .NET $2a$ format), generates JWT with nameid/email/role claims using HS256
- **registrar**: Checks duplicate email, generates sequential code (USR-001...), hashes password with bcrypt cost 10, inserts user, manages FinanceiroProdutor associations for perfil_id=3
- **get_all**: Lists all users with perfil_nome (admin only)
- **get_by_id**: Returns user details including produtores_ids from FinanceiroProdutor junction table
- **update**: Updates user fields, optionally re-hashes password, manages FinanceiroProdutor associations
- **delete**: Checks Produtor dependencies (granjas, associations) before deletion

### GranjaService (granja_service.rs)
- **get_all**: Role-based filtering -- Admin sees all, Produtor sees own, Financeiro sees via FinanceiroProdutor junction
- **get_by_id**: Fetches granja with role-based access verification
- **create**: Blocks Financeiro, generates sequential code (GRJ-001...), Admin can assign owner
- **update**: Blocks Financeiro, verifies access, Admin can reassign owner
- **delete**: Blocks Financeiro, verifies access, removes granja

### AuditoriaService (auditoria_service.rs)
- **registrar_log**: Inserts audit record into LogsAuditoria with timestamp, user info, action, and details

### HTTP Handlers
- 6 auth handlers: login, registrar, get_usuarios, get_usuario, update_usuario, delete_usuario
- 5 granjas handlers: get_granjas, get_granja, create_granja, update_granja, delete_granja
- All with utoipa::path annotations for Swagger documentation
- Input validation via validator::Validate on all request bodies
- Role enforcement in handlers (admin-only checks) and services (role filtering)

### main.rs Wiring
- CORS with explicit origins from ALLOWED_ORIGINS env var (semicolon-separated)
- Swagger UI at /swagger-ui/ conditionally enabled
- OpenApi struct with all 11 paths and 9 DTO schemas
- configure_routes registers /api/auth/* and /api/granjas/*

## Deviations from Plan

None - plan executed exactly as written.

## Decisions Made

1. **Stateless service structs (D-06)**: All services are unit structs with associated functions taking PgPool as parameter
2. **Column aliases in SQL (D-02)**: Runtime queries use `SELECT "Id" as id, "Nome" as nome` pattern to map PascalCase DB columns to snake_case Rust struct fields
3. **Explicit CORS origins (T-1-05)**: Not using allow_any_origin(), iterating config.allowed_origins for each allowed_origin() call
4. **Sequential code generation**: Using `MAX("Id") + 1` formatted as USR-XXX / GRJ-XXX (matches .NET pattern)
5. **HTTP status codes (D-07)**: POST create returns 201 Created, DELETE returns 204 No Content, GET/PUT returns 200 OK

## Verification Results

- cargo check: PASSED (0 errors, warnings only for unused models/DTOs from Plan 02)
- Auth handler count: 6 (login, registrar, get_usuarios, get_usuario, update_usuario, delete_usuario)
- Granjas handler count: 5 (get_granjas, get_granja, create_granja, update_granja, delete_granja)
- configure_routes in main.rs: FOUND
- SwaggerUi in main.rs: FOUND
- Cors in main.rs: FOUND
- bcrypt::verify in auth_service.rs: FOUND
- Algorithm::HS256 in auth_service.rs: FOUND
- allow_any_origin: NOT FOUND (correct)

## Known Stubs

None. All services are fully implemented with real SQL queries and business logic.

## Self-Check: PASSED

- All 9 files verified present on disk
- All 3 commit hashes verified in git log (7884862, 6af0d8c, 682d6dd)
- cargo check exits 0 with no errors
