---
phase: 02-complete-backend-crud
reviewed: 2026-04-07T12:00:00Z
depth: standard
files_reviewed: 26
files_reviewed_list:
  - granjatech-api/Cargo.toml
  - granjatech-api/src/dto/profile.rs
  - granjatech-api/src/handlers/auditoria.rs
  - granjatech-api/src/handlers/consumo.rs
  - granjatech-api/src/handlers/dashboard.rs
  - granjatech-api/src/handlers/estoque.rs
  - granjatech-api/src/handlers/financas.rs
  - granjatech-api/src/handlers/leituras.rs
  - granjatech-api/src/handlers/lotes.rs
  - granjatech-api/src/handlers/mod.rs
  - granjatech-api/src/handlers/pesagem.rs
  - granjatech-api/src/handlers/profile.rs
  - granjatech-api/src/handlers/sanitario.rs
  - granjatech-api/src/handlers/sensores.rs
  - granjatech-api/src/main.rs
  - granjatech-api/src/models/log_auditoria.rs
  - granjatech-api/src/services/consumo_service.rs
  - granjatech-api/src/services/dashboard_service.rs
  - granjatech-api/src/services/estoque_service.rs
  - granjatech-api/src/services/financas_service.rs
  - granjatech-api/src/services/lote_service.rs
  - granjatech-api/src/services/mod.rs
  - granjatech-api/src/services/pesagem_service.rs
  - granjatech-api/src/services/profile_service.rs
  - granjatech-api/src/services/sanitario_service.rs
  - granjatech-api/src/services/sensor_service.rs
findings:
  critical: 2
  warning: 5
  info: 3
  total: 10
status: issues_found
---

# Phase 02: Code Review Report

**Reviewed:** 2026-04-07T12:00:00Z
**Depth:** standard
**Files Reviewed:** 26
**Status:** issues_found

## Summary

Reviewed the complete Phase 2 backend CRUD implementation for GranjaTech's Rust migration. The code is generally well-structured, follows consistent patterns, and properly implements role-based access control, audit logging, and input validation across all new modules (Lotes, Dashboard, Financas, Consumo, Pesagem, Sanitario, Sensores, Estoque, Auditoria, Profile).

Two critical issues were identified: an unauthenticated IoT endpoint that lacks any form of input validation beyond schema validation (potential abuse vector), and a race condition in the lote code generation that can produce duplicate codes under concurrent requests. Five warnings address missing pagination on unbounded queries, a stale JWT issue after profile email changes, authorization bypass in consumo/pesagem read endpoints, deprecated chrono::Duration usage, and a TOCTOU race in the unique sensor identifier check.

## Critical Issues

### CR-01: Lote Code Generation Race Condition (Duplicate Codes)

**File:** `granjatech-api/src/services/lote_service.rs:169-175`
**Issue:** The sequential code generation (`LT-001`, `LT-002`, etc.) uses `SELECT MAX("Id")` followed by a separate `INSERT`. Under concurrent requests, two transactions can read the same MAX(Id) value and generate identical codes. This is a classic TOCTOU (time-of-check-time-of-use) race condition.
**Fix:** Use the database's RETURNING clause with a sequence or generate the code from the newly inserted ID:
```rust
// Option A: Generate code from the RETURNING Id after insert
let lote = sqlx::query_as::<_, Lote>(
    r#"INSERT INTO "Lotes" ("Identificador", ...)
       VALUES ($1, ...)
       RETURNING *"#,
)
// ... binds ...
.fetch_one(pool)
.await?;

let codigo = format!("LT-{:03}", lote.id);
sqlx::query(r#"UPDATE "Lotes" SET "Codigo" = $1 WHERE "Id" = $2"#)
    .bind(&codigo)
    .bind(lote.id)
    .execute(pool)
    .await?;

// Option B: Use a database sequence or SERIAL column for Codigo
```

### CR-02: Unauthenticated IoT Leitura Endpoint Lacks Rate Limiting or Device Validation

**File:** `granjatech-api/src/handlers/leituras.rs:22-30` and `granjatech-api/src/services/sensor_service.rs:223-257`
**Issue:** The `POST /api/leituras` endpoint is completely unauthenticated (by design, for IoT devices). However, there is no rate limiting, no API key, no device token, and no validation that the `identificador_unico` belongs to a legitimate device. Any attacker who discovers a sensor's `identificador_unico` can flood the database with arbitrary readings, corrupt sensor data, or perform a denial-of-service attack. The `identificador_unico` values are also exposed through the authenticated `/api/sensores` list endpoint, making enumeration trivial for any authenticated user.
**Fix:** Add at minimum one of:
1. An API key header check (e.g., `X-Device-Key`) configured per deployment
2. Rate limiting middleware on the `/api/leituras` route (e.g., using `actix-governor`)
3. A shared secret or HMAC signature in the request body
```rust
// Example: Simple API key check
pub async fn post_leitura(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateLeituraDto>,
) -> Result<HttpResponse, AppError> {
    let api_key = req.headers().get("X-Device-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing device key".into()))?;
    // Validate api_key against config
    // ...
}
```

## Warnings

### WR-01: Auditoria Endpoint Returns All Logs Without Pagination

**File:** `granjatech-api/src/handlers/auditoria.rs:30-38`
**Issue:** `GET /api/auditoria` fetches ALL audit logs from the database with no pagination (`LIMIT`/`OFFSET`). As the system accumulates audit entries, this query will return increasingly large result sets, eventually causing timeouts or memory exhaustion on both the database and API server. The .NET original likely has the same issue, but this is a correctness concern for production use.
**Fix:** Add query parameters for pagination:
```rust
#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}
// In the handler, default to page=1, page_size=50, add LIMIT/OFFSET to the query
```

### WR-02: Profile Email Update Does Not Invalidate JWT Token

**File:** `granjatech-api/src/services/profile_service.rs:80-118`
**Issue:** When a user updates their email via `PUT /api/profile`, the email stored in the database changes, but the JWT token still contains the old email in `claims.email`. All subsequent requests will use the stale email from the JWT for audit logging until the token expires (8 hours per the .NET config). This means audit logs will record the old email for actions taken after the email change, creating misleading audit trails.
**Fix:** Either return a new JWT token in the update response so the frontend can replace it, or fetch the current email from the database for audit logging instead of relying on claims:
```rust
// Option A: Return new token in response
Ok(HttpResponse::Ok().json(serde_json::json!({
    "message": "Perfil atualizado com sucesso",
    "token": new_jwt_token  // regenerated with new email
})))
```

### WR-03: Consumo and Pesagem List Endpoints Skip Granja Access Verification

**File:** `granjatech-api/src/services/consumo_service.rs:140-173` and `granjatech-api/src/services/pesagem_service.rs:89-106`
**Issue:** The `list_racao`, `list_agua`, and `PesagemService::list` functions accept a `lote_id` directly and query the database without verifying that the authenticated user has access to the lote's parent granja. A Produtor could read consumo/pesagem data for lotes belonging to other Produtors by guessing lote IDs. The handler checks that the role is Admin or Produtor, but does not verify ownership of the specific lote. Compare with `LoteService::get_by_id` which correctly calls `verificar_acesso_granja`.
**Fix:** Add granja access verification before returning data:
```rust
pub async fn list_racao(
    pool: &PgPool,
    lote_id: i32,
    user_id: i32,
    user_role: &str,
) -> Result<Vec<ConsumoRacaoResponseDto>, AppError> {
    // Verify user has access to this lote's granja
    let granja_id: Option<i32> = sqlx::query_scalar(
        r#"SELECT "GranjaId" FROM "Lotes" WHERE "Id" = $1"#
    ).bind(lote_id).fetch_optional(pool).await?;
    // ... verify ownership ...
}
```

### WR-04: Sensor Unique Identifier Check Has TOCTOU Race Condition

**File:** `granjatech-api/src/services/sensor_service.rs:81-103`
**Issue:** The uniqueness check for `IdentificadorUnico` uses `SELECT EXISTS(...)` followed by a separate `INSERT`. Two concurrent requests with the same identifier could both pass the existence check and then one would fail at the database level (if there is a unique constraint) or both would succeed (if there is no unique constraint), creating duplicate sensors.
**Fix:** Rely on a database-level UNIQUE constraint on `"Sensores"."IdentificadorUnico"` (which should exist) and handle the constraint violation error gracefully, or use `INSERT ... ON CONFLICT DO NOTHING`:
```rust
// Remove the SELECT EXISTS check, add UNIQUE constraint handling
let sensor = sqlx::query_as::<_, Sensor>(
    r#"INSERT INTO "Sensores" ("Tipo", "IdentificadorUnico", "GranjaId")
       VALUES ($1, $2, $3)
       RETURNING "Id", "Tipo", "IdentificadorUnico", "GranjaId""#,
)
// ...
.fetch_one(pool)
.await
.map_err(|e| match e {
    sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
        AppError::BadRequest("Sensor com este identificador unico ja existe.".into())
    }
    _ => AppError::from(e),
})?;
```

### WR-05: Deprecated chrono::Duration Usage Will Break on Future Versions

**File:** `granjatech-api/src/services/financas_service.rs:199`
**Issue:** `Duration::minutes(5)` from the `chrono` crate has been deprecated in favor of `TimeDelta::minutes(5)` or `Duration::try_minutes(5)`. While this compiles today, it will produce warnings and eventually fail to compile in future chrono versions.
**Fix:**
```rust
use chrono::TimeDelta;
// ...
if user_role != "Administrador"
    && Utc::now() - transacao.timestamp_criacao > TimeDelta::minutes(5)
```

## Info

### IN-01: Duplicated verificar_acesso_granja Helper Across Services

**File:** `granjatech-api/src/services/lote_service.rs:520-568`, `granjatech-api/src/services/estoque_service.rs:224-255`, `granjatech-api/src/services/sensor_service.rs:260-291`
**Issue:** The `verificar_acesso_granja` function is duplicated across three service modules with slightly different implementations (LoteService includes Financeiro access, EstoqueService and SensorService do not). This duplication increases maintenance burden and risks inconsistency.
**Fix:** Extract a shared `granja_access::verificar_acesso` function into a common module under `services/` and parameterize which roles are allowed.

### IN-02: Dashboard Service Uses f64 for Year/Month Extraction Instead of Integer Cast

**File:** `granjatech-api/src/services/dashboard_service.rs:203-204`
**Issue:** PostgreSQL's `EXTRACT()` returns `double precision` (f64), and the code casts it with `as i32` / `as usize`. While this works for valid month/year values, the floating-point intermediate representation is unnecessary. Using `DATE_PART` or casting in SQL (`EXTRACT(...)::integer`) would be cleaner.
**Fix:** Cast in SQL: `EXTRACT(YEAR FROM "Data")::integer as "ano"` and change the struct fields to `Option<i32>`.

### IN-03: Consumo and Pesagem Resumo Endpoints Return Untyped serde_json::Value

**File:** `granjatech-api/src/services/consumo_service.rs:176-228`, `granjatech-api/src/services/pesagem_service.rs:109-162`, `granjatech-api/src/services/sanitario_service.rs:105-188`
**Issue:** Several "resumo" endpoints return `serde_json::Value` instead of typed DTOs. This bypasses compile-time type checking, makes the API contract implicit rather than explicit, and prevents these types from appearing in the OpenAPI/Swagger documentation.
**Fix:** Define typed response DTOs (e.g., `ConsumoResumoDto`, `PesagemResumoDto`, `SanitarioResumoDto`) and use them as return types.

---

_Reviewed: 2026-04-07T12:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
