# Phase 01 -- Rust Foundation: Security Verification

**Phase:** 01 -- rust-foundation
**Date:** 2026-04-07
**ASVS Level:** 1
**Threats Closed:** 5/5
**Status:** SECURED

## Threat Verification

| Threat ID | Category | Disposition | Status | Evidence |
|-----------|----------|-------------|--------|----------|
| T-1-01 | Information Disclosure | mitigate | CLOSED | JWT secret loaded from env var `JWT_KEY` via `std::env::var("JWT_KEY").expect(...)` in `config.rs:16-17`. No hardcoded secrets in source. `.env` listed in `.gitignore:375`. `.env.example` committed with placeholder values only. Config panics on missing `JWT_KEY` (line 17) and `DATABASE_URL` (line 14). |
| T-1-02 | Tampering | mitigate | CLOSED | `middleware/jwt.rs:61` creates `Validation::new(Algorithm::HS256)` -- enforces HMAC-SHA256 only (rejects "none" algorithm). `jwt.rs:62-63` calls `set_issuer` and `set_audience` for full token validation. `auth_service.rs:35` uses `bcrypt::verify()` for constant-time password comparison. `auth_service.rs:78` hashes with cost factor 10. |
| T-1-03 | Elevation of Privilege | mitigate | CLOSED | `handlers/auth.rs:72,99,130,163` check `claims.role != "Administrador"` returning `AppError::Forbidden` for non-admin users on user management endpoints. `granja_service.rs:120,182,235` block Financeiro from CUD operations. `granja_service.rs:18-55` filters list results by role: Admin=all, Produtor=own (`WHERE "UsuarioId" = $1`), Financeiro=via junction table (`INNER JOIN "FinanceiroProdutor"`). |
| T-1-04 | Tampering (SQL injection) | mitigate | CLOSED | All SQL queries in `auth_service.rs` and `granja_service.rs` use `sqlx::query_as` / `sqlx::query` / `sqlx::query_scalar` with `.bind()` parameterized placeholders (`$1`, `$2`, etc.). Zero string interpolation found in any SQL statement across both files. |
| T-1-05 | Spoofing (CORS) | mitigate | CLOSED | `main.rs:75` creates `Cors::default()` (not `allow_any_origin`). `main.rs:84-86` iterates `allowed_origins` calling `cors.allowed_origin(origin)` for each configured origin. Origins loaded from `ALLOWED_ORIGINS` env var in `config.rs:22-25`. `allow_any_origin` not present anywhere in source. |

## Unregistered Flags

None. No `## Threat Flags` section found in any SUMMARY.md file for this phase.

## Accepted Risks Log

None for this phase.

## Transfer Documentation

None for this phase.
