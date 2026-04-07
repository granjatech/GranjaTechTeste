---
phase: 02-complete-backend-crud
plan: 03
subsystem: backend-api
tags: [rust, actix-web, sensors, stock, audit, profile, bcrypt, utoipa]
dependency_graph:
  requires: [02-02]
  provides: [sensor-crud, estoque-crud, auditoria-read, profile-management, public-leitura-endpoint]
  affects: [granjatech-api]
tech_stack:
  added: []
  patterns: [public-endpoint-no-claims, bcrypt-verify-hash, granja-ownership-check, admin-only-guard]
key_files:
  created:
    - granjatech-api/src/services/sensor_service.rs
    - granjatech-api/src/services/estoque_service.rs
    - granjatech-api/src/services/profile_service.rs
    - granjatech-api/src/handlers/sensores.rs
    - granjatech-api/src/handlers/leituras.rs
    - granjatech-api/src/handlers/estoque.rs
    - granjatech-api/src/handlers/auditoria.rs
    - granjatech-api/src/handlers/profile.rs
  modified:
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/main.rs
    - granjatech-api/src/dto/profile.rs
    - granjatech-api/src/models/log_auditoria.rs
decisions:
  - Public leitura endpoint has no Claims extractor matching .NET LeiturasController pattern
  - BCrypt cost 12 for password change to maintain .NET compatibility
  - LogAuditoria model got ToSchema derive added for utoipa OpenAPI compatibility
metrics:
  duration_seconds: 321
  completed: "2026-04-07T14:13:35Z"
  tasks_completed: 2
  tasks_total: 2
---

# Phase 02 Plan 03: Sensores/Estoque/Auditoria/Profile Summary

SensorService with public IoT leitura endpoint, EstoqueService CRUD, audit log read, and ProfileService with bcrypt password verification -- completing all remaining backend CRUD endpoints for .NET parity.

## Task Results

### Task 1: Implement SensorService, EstoqueService, ProfileService
**Commit:** c702c8d

- **SensorService** (sensor_service.rs): list/create/delete with Admin+Produtor granja ownership checks, list_leituras with access verification, registrar_leitura as public endpoint (no user context, lookup by IdentificadorUnico)
- **EstoqueService** (estoque_service.rs): Full CRUD (list/create/update/delete) with granja ownership verification, Financeiro blocked
- **ProfileService** (profile_service.rs): get_profile with FinanceiroProdutor association lookup, update_profile with email uniqueness check, change_password with bcrypt::verify for old password and bcrypt::hash(cost=12) for new
- **ChangePasswordDto** added to dto/profile.rs with senha_atual and nova_senha fields
- All CUD operations call AuditoriaService::registrar_log

### Task 2: Implement handlers and register all routes
**Commit:** 27bf0aa

- **sensores.rs**: 4 handlers (get_sensores, create_sensor, delete_sensor, get_leituras_sensor) with Admin+Produtor role check
- **leituras.rs**: 1 public handler (post_leitura) -- NO Claims extractor, matches .NET LeiturasController
- **estoque.rs**: 4 handlers (get_produtos, create_produto, update_produto, delete_produto) with Admin+Produtor role check
- **auditoria.rs**: 1 handler (get_logs) with Administrador-only check
- **profile.rs**: 3 handlers (get_profile, update_profile, change_password) for any authenticated user
- Route registration for all 5 new scopes (/sensores, /leituras, /estoque, /auditoria, /profile)
- OpenAPI: 13 new paths, 12 new schemas, 5 new tags added to main.rs

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added ToSchema derive to LogAuditoria model**
- **Found during:** Task 2
- **Issue:** LogAuditoria used as response body in auditoria handler but lacked utoipa::ToSchema derive, causing compilation failure
- **Fix:** Added `use utoipa::ToSchema;` and `ToSchema` to derive macro in models/log_auditoria.rs
- **Files modified:** granjatech-api/src/models/log_auditoria.rs
- **Commit:** 27bf0aa

## Verification

- cargo check: PASSED (compiles with only unused-item warnings)
- All 12 handler functions exist with utoipa path annotations
- POST /api/leituras handler has NO Claims parameter (confirmed public)
- GET /api/auditoria checks claims.role == "Administrador"
- Profile change-password uses bcrypt::verify + bcrypt::hash(cost=12)
- All 5 new OpenAPI tags registered

## Self-Check: PASSED

- All 8 created files exist on disk
- Commits c702c8d and 27bf0aa found in git log
- SensorService struct, registrar_leitura, bcrypt verify/hash all present
- post_leitura handler confirmed public (no Claims parameter)
- Auditoria handler confirmed admin-only check
- OpenAPI registration confirmed for leituras and profile
