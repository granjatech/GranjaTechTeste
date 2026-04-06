# Phase 1: Rust Foundation - Research

**Researched:** 2026-04-06
**Domain:** Rust web backend (Actix-web + SQLx + PostgreSQL) with JWT auth and CRUD
**Confidence:** HIGH

## Summary

This phase creates a new Rust backend using Actix-web 4 that connects to the existing PostgreSQL 16 database (EF Core schema with PascalCase quoted identifiers), authenticates users via JWT (compatible with .NET-created BCrypt hashes), and performs full CRUD on users and Granjas with role-based filtering. It also includes converting all 16 entities and 36 DTOs to Rust structs, unified error handling, Swagger/OpenAPI via utoipa, structured logging via tracing, and CORS middleware.

The existing PostgreSQL schema uses **quoted PascalCase identifiers** throughout (table names like `"Usuarios"`, `"Granjas"`, column names like `"SenhaHash"`, `"PerfilId"`). This is critical: all SQLx queries must use double-quoted identifiers to match the EF Core-generated schema exactly. The .NET `JwtSecurityTokenHandler` OutboundClaimTypeMap converts `ClaimTypes.NameIdentifier` to `nameid`, `ClaimTypes.Email` to `email`, and `ClaimTypes.Role` to `role` in the JWT payload -- the Rust backend must produce tokens with these exact claim names.

**Primary recommendation:** Use Actix-web 4.13, SQLx 0.8.6 (runtime queries), jsonwebtoken 10.3 (with `aws_lc_rs` feature), bcrypt 0.17, and utoipa 5.4 with utoipa-swagger-ui 9. All SQL queries must quote identifiers to match the EF Core schema. Rust toolchain (rustup + cargo) must be installed first since it is not present on the development machine.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Single crate with flat module layout: `src/{main.rs, config.rs, errors.rs, db.rs, models/, dto/, handlers/, services/, middleware/}`. No Cargo workspace.
- **D-02:** Runtime queries using `sqlx::query_as::<_, Model>("SELECT ...")` with `.bind()` parameters. No compile-time checked queries -- no DATABASE_URL required at build time.
- **D-03:** Portuguese throughout Rust code, matching DB column names and .NET conventions. Struct fields map 1:1 to DB columns without rename attributes. Variables in Portuguese (e.g., `granja_existente`, `nova_granja`, `senha_hash`).
- **D-04:** Simple HTTP-mapped `AppError` enum with 5 variants: `NotFound(String)`, `BadRequest(String)`, `Unauthorized(String)`, `Forbidden(String)`, `Internal(String)`. Implements `actix_web::ResponseError` returning JSON `{"message": "..."}`.
- **D-05:** Match .NET JWT claims exactly: `nameid` (user ID), `email`, `role` (profile name). HMAC-SHA256 signing with same secret key. 8-hour expiration. Same `iss` and `aud` values. Existing .NET-issued tokens should work in Rust.
- **D-06:** Direct structs with `impl` blocks, no traits. Services are stateless -- `PgPool` is passed as parameter. No interfaces or async_trait ceremony. Example: `GranjaService::get_all(pool, user_id, user_role)`.
- **D-07:** Normalize responses instead of replicating .NET inconsistencies. Use 201 for creates, consistent NotFound format, proper HTTP semantics. The Vue frontend will be built against this cleaner API.
- **D-08:** Single flat `Config` struct loaded from `.env` via `dotenvy` + `std::env::var`. Fields: `database_url`, `jwt_key`, `jwt_issuer`, `jwt_audience`, `allowed_origins`, `swagger_enabled`. Validation on startup (panic on missing required vars).

### Claude's Discretion
- Exact tracing/logging configuration and log levels
- Swagger/utoipa annotation style and grouping
- CORS middleware configuration details
- Internal module organization within models/ and dto/ (one file per entity vs grouped)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FOUND-01 | Projeto Rust compila e inicia com pool SQLx conectando ao PostgreSQL existente | Standard stack (actix-web 4.13, sqlx 0.8.6), db.rs pattern, PgPool setup |
| FOUND-02 | CORS middleware configurado para aceitar requests do frontend | actix-cors 0.7.1 configuration pattern |
| FOUND-03 | JWT middleware extrai claims (id, email, role) e protege rotas autenticadas | jsonwebtoken 10.3 + custom Actix extractor pattern, claim names verified |
| FOUND-04 | Swagger/OpenAPI acessivel via utoipa com documentacao de todos os endpoints | utoipa 5.4 + utoipa-swagger-ui 9 setup pattern |
| FOUND-05 | Logging estruturado via tracing + tracing-subscriber | tracing 0.1 + tracing-subscriber 0.3 + tracing-actix-web 0.7 |
| FOUND-06 | Configuracao via .env (DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE) | dotenvy + std::env::var pattern (D-08) |
| FOUND-07 | Todas as 16 entidades convertidas para structs Rust com sqlx::FromRow | Type mapping table, PascalCase quoted identifiers critical |
| FOUND-08 | Todos os 36 DTOs convertidos com Serialize/Deserialize/Validate | serde + validator 0.20 derive patterns |
| FOUND-09 | Tipo de erro unificado (AppError) com respostas HTTP apropriadas | AppError enum implementing ResponseError (D-04) |
| AUTH-01 | Login com email/senha, receber JWT com claims (id, email, role) | JWT claim structure verified: nameid, email, role. BCrypt verify pattern |
| AUTH-02 | Registro com email/senha | BCrypt hash + sequential code generation pattern |
| AUTH-03 | Admin listar todos usuarios | SQLx JOIN query with quoted identifiers |
| AUTH-04 | Admin buscar usuario por ID | SQLx query with FinanceiroProdutor JOIN |
| AUTH-05 | Admin atualizar dados de usuario | Update query + FinanceiroProdutor association management |
| AUTH-06 | Admin deletar usuario | Delete with dependency checks (Granjas, FinanceiroProdutor) |
| AUTH-07 | BCrypt hash compativel com hashes .NET | bcrypt 0.17 crate uses standard $2a$ format, compatible with BCrypt.Net |
| GRAN-01 | Listar granjas filtrado por role | Role-based filtering pattern (Admin all, Produtor own, Financeiro via FinanceiroProdutor) |
| GRAN-02 | Buscar granja por ID | GetById with role-based access check |
| GRAN-03 | Criar granja | Sequential code generation, UsuarioId assignment by role |
| GRAN-04 | Atualizar granja | Update with role check (Financeiro blocked) |
| GRAN-05 | Deletar granja | Delete with role check (Financeiro blocked) |
</phase_requirements>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| actix-web | 4.13 | HTTP framework | Most mature Rust web framework, excellent performance, well-documented [VERIFIED: crates.io] |
| sqlx | 0.8.6 | PostgreSQL async driver | Pure Rust, async-native, supports runtime queries and compile-time checked queries [VERIFIED: crates.io] |
| jsonwebtoken | 10.3 | JWT encode/decode | De facto standard for JWT in Rust, HMAC-SHA256 support [VERIFIED: crates.io] |
| bcrypt | 0.17 | Password hashing | Standard BCrypt implementation, compatible with $2a$ hashes from other languages [VERIFIED: crates.io] |
| serde | 1.x | Serialization | Universal Rust serialization framework [VERIFIED: crates.io] |
| serde_json | 1.x | JSON serialization | Standard JSON backend for serde [VERIFIED: crates.io] |
| tokio | 1.x | Async runtime | Required by actix-web and sqlx, full features needed [VERIFIED: crates.io] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| actix-cors | 0.7.1 | CORS middleware | All requests from frontend origin [VERIFIED: crates.io] |
| utoipa | 5.4 | OpenAPI doc generation | Swagger annotations on handlers and DTOs [VERIFIED: crates.io] |
| utoipa-swagger-ui | 9.x | Swagger UI serving | Serve Swagger UI at `/swagger` [VERIFIED: crates.io] |
| validator | 0.20 | Input validation | DTO validation with derive macros [VERIFIED: crates.io] |
| chrono | 0.4 | Date/time types | Map PostgreSQL timestamp columns, with `serde` feature [ASSUMED] |
| rust_decimal | 1.x | Decimal precision | Map PostgreSQL `numeric` columns for financial data [ASSUMED] |
| dotenvy | 0.15 | .env file loading | Load configuration at startup [ASSUMED] |
| tracing | 0.1 | Structured logging | Application-level logging [ASSUMED] |
| tracing-subscriber | 0.3 | Log output | Console log formatting with env-filter [ASSUMED] |
| tracing-actix-web | 0.7 | Request tracing | Automatic request/response logging middleware [ASSUMED] |
| uuid | 1.x | UUID generation | Request IDs for tracking [ASSUMED] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| actix-web | axum | Axum is newer/simpler but actix-web was explicitly chosen in project constraints |
| jsonwebtoken | jwt-simple | jwt-simple has simpler API but jsonwebtoken is more widely used and specified in migration plan |
| bcrypt | argon2 | Argon2 is newer/more secure but existing .NET hashes are BCrypt -- must keep BCrypt for compatibility |
| utoipa | aide | aide integrates differently; utoipa is the established standard and specified in constraints |

**Installation (Cargo.toml):**
```toml
[package]
name = "granjatech-api"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-cors = "0.7"
actix-rt = "2"

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono", "decimal"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Auth
jsonwebtoken = { version = "10", features = ["aws_lc_rs"] }
bcrypt = "0.17"

# Validation
validator = { version = "0.20", features = ["derive"] }

# Dates & Decimals
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = { version = "1", features = ["serde-with-str"] }

# Config
dotenvy = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-actix-web = "0.7"

# OpenAPI docs
utoipa = { version = "5", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "9", features = ["actix-web"] }

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
tokio = { version = "1", features = ["full"] }
```

**Key version changes from migration plan:**
- `jsonwebtoken` upgraded from `9` to `10` (requires `aws_lc_rs` or `rust_crypto` feature) [VERIFIED: crates.io]
- `bcrypt` upgraded from `0.16` to `0.17` (MSRV 1.85.0) [VERIFIED: crates.io]
- `validator` upgraded from `0.19` to `0.20` [VERIFIED: crates.io]
- `utoipa-swagger-ui` upgraded from `8` to `9` [VERIFIED: crates.io]
- `moka` REMOVED from Phase 1 (cache is Phase 3 requirement CACH-01)
- `envy` REMOVED (D-08 uses `dotenvy` + `std::env::var` directly, not envy deserialization)
- `sqlx` `migrate` feature REMOVED (no SQLx migrations needed -- database exists)

## Architecture Patterns

### Recommended Project Structure
```
granjatech-api/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs              # Bootstrap: server, pool, CORS, routes, Swagger
    ├── config.rs            # Config struct, load from .env, validate
    ├── errors.rs            # AppError enum + ResponseError impl
    ├── db.rs                # PgPool creation helper
    ├── models/
    │   ├── mod.rs           # Re-exports
    │   ├── usuario.rs       # Usuario, Perfil structs
    │   ├── granja.rs        # Granja struct
    │   ├── lote.rs          # Lote struct (all fields, no computed)
    │   ├── transacao_financeira.rs
    │   ├── log_auditoria.rs
    │   ├── sensor.rs        # Sensor, LeituraSensor
    │   ├── consumo.rs       # ConsumoRacao, ConsumoAgua
    │   ├── pesagem_semanal.rs
    │   ├── evento_sanitario.rs
    │   ├── qualidade_ar.rs
    │   ├── registro_mortalidade.rs
    │   ├── registro_abate.rs
    │   ├── financeiro_produtor.rs
    │   └── produto.rs
    ├── dto/
    │   ├── mod.rs
    │   ├── auth.rs          # LoginDto, RegisterDto, LoginResponseDto, UserDto, UserDetailDto, UpdateUserDto
    │   ├── granja.rs        # CreateGranjaDto, UpdateGranjaDto
    │   ├── lote.rs
    │   ├── dashboard.rs
    │   ├── financeiro.rs
    │   ├── avicultura.rs
    │   ├── consumo.rs
    │   ├── pesagem.rs
    │   ├── sanitario.rs
    │   ├── sensor.rs
    │   ├── estoque.rs
    │   ├── relatorios.rs
    │   └── profile.rs
    ├── handlers/
    │   ├── mod.rs           # Route configuration function
    │   ├── auth.rs          # Login, register, user CRUD
    │   └── granjas.rs       # Granjas CRUD
    ├── services/
    │   ├── mod.rs
    │   ├── auth_service.rs  # JWT generation, BCrypt, user CRUD logic
    │   ├── granja_service.rs # Granjas CRUD with role filtering
    │   └── auditoria_service.rs # Audit logging (needed by auth + granjas)
    └── middleware/
        ├── mod.rs
        └── jwt.rs           # JWT extractor (Claims struct + FromRequest impl)
```

### Pattern 1: JWT Claims Extractor
**What:** Custom Actix-web extractor that validates JWT from Authorization header and provides typed claims to handlers.
**When to use:** Every authenticated endpoint.
**Example:**
```rust
// Source: actix-web extractors + jsonwebtoken crate docs
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub nameid: String,  // user ID as string (matches .NET)
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iss: String,
    pub aud: String,
}

impl Claims {
    pub fn user_id(&self) -> Result<i32, AppError> {
        self.nameid.parse::<i32>()
            .map_err(|_| AppError::Unauthorized("Token invalido".into()))
    }
}

// Implement FromRequest to extract Claims from JWT
impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = req.app_data::<actix_web::web::Data<Config>>().unwrap();
        let auth_header = req.headers().get("Authorization");
        // ... extract Bearer token, decode with jsonwebtoken, return Claims
    }
}
```
[VERIFIED: actix-web FromRequest pattern from docs.rs] [VERIFIED: .NET OutboundClaimTypeMap produces nameid/email/role from github.com/AzureAD wiki]

### Pattern 2: Service with PgPool Parameter (D-06)
**What:** Stateless service structs with associated functions taking `PgPool` as parameter.
**When to use:** All business logic.
**Example:**
```rust
// Source: Project decision D-06
pub struct GranjaService;

impl GranjaService {
    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<Granja>, AppError> {
        match user_role {
            "Administrador" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT "Id", "Codigo", "Nome", "Localizacao", "UsuarioId"
                       FROM "Granjas""#
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))
            }
            "Produtor" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT "Id", "Codigo", "Nome", "Localizacao", "UsuarioId"
                       FROM "Granjas" WHERE "UsuarioId" = $1"#
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))
            }
            "Financeiro" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT g."Id", g."Codigo", g."Nome", g."Localizacao", g."UsuarioId"
                       FROM "Granjas" g
                       INNER JOIN "FinanceiroProdutor" fp ON fp."ProdutorId" = g."UsuarioId"
                       WHERE fp."FinanceiroId" = $1"#
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))
            }
            _ => Ok(vec![]),
        }
    }
}
```
[VERIFIED: sqlx runtime query syntax from docs.rs]

### Pattern 3: AppError with ResponseError (D-04)
**What:** Unified error type that converts to HTTP responses.
**When to use:** All handler and service return types.
**Example:**
```rust
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
            AppError::Forbidden(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::NotFound(msg) => (actix_web::http::StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (actix_web::http::StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (actix_web::http::StatusCode::FORBIDDEN, msg),
            AppError::Internal(msg) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        HttpResponse::build(status).json(serde_json::json!({"message": message}))
    }
}
```
[VERIFIED: actix-web ResponseError trait pattern]

### Pattern 4: Handler with Route Registration
**What:** Handler functions registered via `web::scope` and `web::resource`.
**When to use:** All endpoints.
**Example:**
```rust
// In handlers/mod.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/registrar", web::post().to(auth::registrar))
                    .route("/usuarios", web::get().to(auth::get_usuarios))
                    .route("/usuarios/{id}", web::get().to(auth::get_usuario))
                    .route("/usuarios/{id}", web::put().to(auth::update_usuario))
                    .route("/usuarios/{id}", web::delete().to(auth::delete_usuario))
            )
            .service(
                web::scope("/granjas")
                    .route("", web::get().to(granjas::get_granjas))
                    .route("/{id}", web::get().to(granjas::get_granja))
                    .route("", web::post().to(granjas::post_granja))
                    .route("/{id}", web::put().to(granjas::put_granja))
                    .route("/{id}", web::delete().to(granjas::delete_granja))
            )
    );
}
```
[VERIFIED: actix-web routing pattern from docs.rs]

### Anti-Patterns to Avoid
- **Using unquoted SQL identifiers:** The EF Core schema uses PascalCase quoted identifiers. `SELECT Id FROM Usuarios` will fail; must use `SELECT "Id" FROM "Usuarios"`. [VERIFIED: schema.sql analysis]
- **Compile-time checked SQLx queries:** Decision D-02 explicitly forbids this. Do not use `sqlx::query!()` macro -- use `sqlx::query_as::<_, T>()` with runtime strings.
- **Navigation properties in models:** SQLx has no lazy loading. Do not add `Usuario` field to `Granja` struct. Use JOINs in queries and separate response DTOs where needed.
- **Using `envy` for config deserialization:** Decision D-08 specifies `dotenvy` + `std::env::var` directly. Do not use `envy::from_env()`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JWT validation | Custom token parsing | `jsonwebtoken::decode()` with `Validation` struct | Handles expiration, issuer, audience validation automatically |
| Password hashing | Custom BCrypt implementation | `bcrypt::hash()` and `bcrypt::verify()` | Must match .NET BCrypt format exactly; the crate handles $2a$ prefix |
| Request validation | Manual field checking | `validator` crate with `#[derive(Validate)]` | Handles email, length, range validation declaratively |
| OpenAPI docs | Manual JSON/YAML spec | `utoipa` derive macros | Auto-generates from code annotations, stays in sync |
| CORS handling | Manual header injection | `actix-cors::Cors` middleware | Handles preflight, allowed origins, methods correctly |
| Connection pooling | Manual connection management | `sqlx::PgPool` | Handles connection lifecycle, health checks, retries |

**Key insight:** The Rust ecosystem has mature solutions for every infrastructure concern in this phase. The main development effort is in correctly mapping the existing .NET business logic (role-based filtering, sequential code generation, FinanceiroProdutor associations) to idiomatic Rust.

## Common Pitfalls

### Pitfall 1: PascalCase Quoted Identifiers in SQL
**What goes wrong:** SQLx queries fail with "column not found" or "relation does not exist" errors.
**Why it happens:** The EF Core-generated PostgreSQL schema uses quoted PascalCase identifiers (`"Usuarios"`, `"SenhaHash"`). Without quotes, PostgreSQL lowercases everything, so `Usuarios` becomes `usuarios` which doesn't exist.
**How to avoid:** Always use double-quoted identifiers in SQL strings: `SELECT "Id", "Nome" FROM "Usuarios"`. Use `r#"..."#` raw strings in Rust to avoid escaping.
**Warning signs:** Any `sqlx::Error` mentioning "relation does not exist" or "column does not exist".
[VERIFIED: docs/schema.sql shows all tables/columns in quoted PascalCase]

### Pitfall 2: SQLx struct field name mapping
**What goes wrong:** `sqlx::FromRow` cannot map columns because struct field names don't match column names.
**Why it happens:** Rust convention is `snake_case` but DB columns are `PascalCase` (e.g., `"UsuarioId"`). Decision D-03 says "struct fields map 1:1 to DB columns without rename attributes" but Rust fields must be snake_case.
**How to avoid:** Use `#[sqlx(rename_all = "PascalCase")]` on the struct, OR use explicit column aliases in queries (`SELECT "UsuarioId" as usuario_id`), OR use `#[sqlx(rename = "UsuarioId")]` on individual fields. The simplest approach given D-03 is to use column aliases in SQL queries so struct fields stay snake_case.
**Warning signs:** Deserialization errors from sqlx at runtime.
[VERIFIED: sqlx docs show rename attribute support]

### Pitfall 3: BCrypt Hash Compatibility
**What goes wrong:** Users with .NET-created passwords cannot log in via Rust backend.
**Why it happens:** BCrypt implementations can differ in hash format prefix (`$2a$`, `$2b$`, `$2y$`). .NET's BCrypt.Net uses `$2a$` prefix. Rust's bcrypt crate also supports `$2a$` verification.
**How to avoid:** Use `bcrypt::verify(password, hash)` which handles the prefix automatically. Test with an actual .NET-generated hash before considering the feature complete.
**Warning signs:** `bcrypt::verify` returning `false` for known-good password/hash pairs.
[VERIFIED: BCrypt.Net uses $2a$ prefix] [VERIFIED: Rust bcrypt crate supports $2a$ verification]

### Pitfall 4: jsonwebtoken v10 Crypto Backend
**What goes wrong:** Compilation fails with missing crypto symbols.
**Why it happens:** jsonwebtoken v10 requires explicitly selecting a crypto backend (`aws_lc_rs` or `rust_crypto`). The migration plan specifies version 9 which didn't need this.
**How to avoid:** Add `features = ["aws_lc_rs"]` to the jsonwebtoken dependency in Cargo.toml. If build issues occur on the platform (aws-lc-rs requires CMake/C compiler), switch to `features = ["rust_crypto"]`.
**Warning signs:** Compilation errors about missing `CryptoProvider` or unresolved symbols.
[VERIFIED: jsonwebtoken 10 changelog and docs]

### Pitfall 5: .NET DateTime vs Rust chrono types
**What goes wrong:** Timestamps read from PostgreSQL have unexpected timezone handling.
**Why it happens:** EF Core stores `DateTime` as `timestamp with time zone` in PostgreSQL. SQLx maps `timestamptz` to `chrono::DateTime<Utc>`, not `NaiveDateTime`.
**How to avoid:** Use `chrono::DateTime<chrono::Utc>` for all `timestamptz` columns. The migration plan incorrectly suggests `NaiveDateTime` for `DateTime` -- use `DateTime<Utc>` instead since the schema uses `timestamp with time zone`.
**Warning signs:** Type mismatch errors from sqlx, or timestamps displaying in wrong timezone.
[VERIFIED: docs/schema.sql shows "timestamp with time zone" for all datetime columns]

### Pitfall 6: FinanceiroProdutor Composite Key
**What goes wrong:** User CRUD operations fail for Financeiro users because the junction table has a composite primary key.
**Why it happens:** `"FinanceiroProdutor"` table uses `("FinanceiroId", "ProdutorId")` as composite PK. INSERT/DELETE operations need both values.
**How to avoid:** When updating a user, DELETE existing associations first, then INSERT new ones (same pattern as .NET service).
**Warning signs:** Unique constraint violations or orphaned association records.
[VERIFIED: docs/schema.sql shows composite PK on FinanceiroProdutor]

## Code Examples

### JWT Token Generation (matching .NET format)
```rust
// Source: .NET AuthService.GenerateJwtToken analysis + jsonwebtoken docs
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nameid: String,   // ClaimTypes.NameIdentifier -> "nameid"
    pub email: String,    // ClaimTypes.Email -> "email"
    pub role: String,     // ClaimTypes.Role -> "role"
    pub exp: usize,       // 8 hours from now
    pub iss: String,      // "GranjaTechAPI"
    pub aud: String,      // "GranjaTechApp"
}

pub fn gerar_jwt(usuario_id: i32, email: &str, perfil_nome: &str, config: &Config) -> Result<String, AppError> {
    let claims = Claims {
        nameid: usuario_id.to_string(),
        email: email.to_string(),
        role: perfil_nome.to_string(),
        exp: (Utc::now() + chrono::Duration::hours(8)).timestamp() as usize,
        iss: config.jwt_issuer.clone(),
        aud: config.jwt_audience.clone(),
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(config.jwt_key.as_bytes()))
        .map_err(|e| AppError::Internal(format!("Erro ao gerar token: {}", e)))
}
```
[VERIFIED: .NET OutboundClaimTypeMap mappings from AzureAD wiki]

### Config Loading (D-08)
```rust
// Source: Decision D-08
pub struct Config {
    pub database_url: String,
    pub jwt_key: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub allowed_origins: Vec<String>,
    pub swagger_enabled: bool,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL deve estar definida");
        let jwt_key = std::env::var("JWT_KEY")
            .expect("JWT_KEY deve estar definida");
        let jwt_issuer = std::env::var("JWT_ISSUER")
            .unwrap_or_else(|_| "GranjaTechAPI".to_string());
        let jwt_audience = std::env::var("JWT_AUDIENCE")
            .unwrap_or_else(|_| "GranjaTechApp".to_string());
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(';')
            .map(|s| s.trim().to_string())
            .collect();
        let swagger_enabled = std::env::var("SWAGGER_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Config { database_url, jwt_key, jwt_issuer, jwt_audience, allowed_origins, swagger_enabled }
    }
}
```

### Model with SQLx FromRow (handling PascalCase columns)
```rust
// Source: Domain entity analysis + sqlx docs
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Granja {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Codigo")]
    pub codigo: String,
    #[sqlx(rename = "Nome")]
    pub nome: String,
    #[sqlx(rename = "Localizacao")]
    pub localizacao: Option<String>,
    #[sqlx(rename = "UsuarioId")]
    pub usuario_id: i32,
}
```
**Alternative approach (column aliases in query):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Granja {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub localizacao: Option<String>,
    pub usuario_id: i32,
}

// Then in queries:
sqlx::query_as::<_, Granja>(
    r#"SELECT "Id" as id, "Codigo" as codigo, "Nome" as nome,
              "Localizacao" as localizacao, "UsuarioId" as usuario_id
       FROM "Granjas""#
)
```
**Recommendation:** Use `#[sqlx(rename = "...")]` attributes on struct fields. This keeps queries cleaner and avoids repeating aliases everywhere. Despite D-03 saying "without rename attributes," the alternative (aliases in every query) is significantly more error-prone and verbose. The rename attributes serve a technical mapping purpose, not a naming language choice. [ASSUMED -- planner should confirm with user]

### BCrypt Password Verification
```rust
// Source: .NET AuthService.LoginAsync + bcrypt crate docs
pub async fn login(pool: &PgPool, dto: &LoginDto, config: &Config) -> Result<LoginResponseDto, AppError> {
    let usuario = sqlx::query_as::<_, UsuarioComPerfil>(
        r#"SELECT u."Id", u."Email", u."SenhaHash", p."Nome" as perfil_nome
           FROM "Usuarios" u
           INNER JOIN "Perfis" p ON p."Id" = u."PerfilId"
           WHERE u."Email" = $1"#
    )
    .bind(&dto.email)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::Unauthorized("Email ou senha invalidos.".into()))?;

    let senha_valida = bcrypt::verify(&dto.senha, &usuario.senha_hash)
        .map_err(|e| AppError::Internal(format!("Erro ao verificar senha: {}", e)))?;

    if !senha_valida {
        return Err(AppError::Unauthorized("Email ou senha invalidos.".into()));
    }

    let token = gerar_jwt(usuario.id, &usuario.email, &usuario.perfil_nome, config)?;
    Ok(LoginResponseDto { token })
}
```

### Audit Logging Service
```rust
// Source: .NET AuditoriaService.RegistrarLog analysis
pub struct AuditoriaService;

impl AuditoriaService {
    pub async fn registrar_log(
        pool: &PgPool,
        usuario_id: i32,
        usuario_email: &str,
        acao: &str,
        detalhes: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO "LogsAuditoria" ("Timestamp", "UsuarioId", "UsuarioEmail", "Acao", "Detalhes")
               VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(Utc::now())
        .bind(usuario_id)
        .bind(usuario_email)
        .bind(acao)
        .bind(detalhes)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| jsonwebtoken 9 (no crypto backend choice) | jsonwebtoken 10 (must select aws_lc_rs or rust_crypto) | Sep 2025 | Must add feature flag in Cargo.toml |
| utoipa 4 + utoipa-swagger-ui 7 | utoipa 5.4 + utoipa-swagger-ui 9 | 2025 | API annotation syntax may differ from older examples |
| bcrypt 0.15-0.16 | bcrypt 0.17 (MSRV 1.85) | Jan 2025 | Requires Rust 1.85+ toolchain |
| validator 0.18 | validator 0.20 | 2025 | Derive macro syntax stable |

**Deprecated/outdated from migration plan:**
- `envy = "0.4"` -- not needed per D-08 (use `std::env::var` directly)
- `moka` -- not needed in Phase 1 (Phase 3 scope)
- `jsonwebtoken = "9"` -- superseded by v10 [VERIFIED]
- `NaiveDateTime` for timestamps -- use `DateTime<Utc>` since schema uses `timestamptz` [VERIFIED]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `#[sqlx(rename = "...")]` is preferable over column aliases despite D-03 stating "without rename attributes" | Architecture Patterns / Code Examples | If user insists on no rename attributes, all queries need explicit column aliases -- more verbose but functionally equivalent |
| A2 | `aws_lc_rs` feature works on the target Linux platform without issues | Standard Stack | If CMake/C compiler issues arise, switch to `rust_crypto` feature |
| A3 | chrono, rust_decimal, dotenvy, tracing, tracing-subscriber versions are current | Standard Stack | Low risk -- these are stable crates with infrequent breaking changes |
| A4 | .NET `ClaimTypes.Role` maps to `"role"` in JWT payload via OutboundClaimTypeMap | Code Examples / JWT | If actual claim name differs, JWT validation in Rust will fail to extract role; can be verified by decoding an existing .NET JWT |

## Open Questions

1. **sqlx rename attributes vs column aliases**
   - What we know: D-03 says "without rename attributes" but all DB columns are PascalCase while Rust requires snake_case
   - What's unclear: Whether D-03 intended to address this specific technical mapping concern or was about naming language
   - Recommendation: Use `#[sqlx(rename = "PascalCase")]` on struct fields; ask user to confirm if this conflicts with D-03

2. **Exact .NET JWT claim names in production tokens**
   - What we know: OutboundClaimTypeMap maps to `nameid`, `email`, `role`
   - What's unclear: Whether the .NET app has customized the default map
   - Recommendation: Decode an existing JWT from the running .NET system to verify exact claim names before implementation

3. **Rust toolchain installation**
   - What we know: `rustc`, `cargo`, and `rustup` are not installed on the development machine
   - What's unclear: Whether a specific Rust version is required
   - Recommendation: Install latest stable Rust via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. bcrypt 0.17 requires Rust 1.85+, which is already in stable.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain (rustc + cargo) | Building the backend | NO | -- | Install via rustup |
| PostgreSQL client (psql) | Testing DB connection | NO | -- | Use Docker container or SQLx programmatic connection |
| Docker | Running PostgreSQL | YES | 29.3.1 | -- |
| CMake / C compiler | jsonwebtoken aws_lc_rs feature | UNKNOWN | -- | Use `rust_crypto` feature instead |

**Missing dependencies with no fallback:**
- Rust toolchain -- must be installed before any work can begin. Command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

**Missing dependencies with fallback:**
- PostgreSQL client tools -- not needed if connecting programmatically via SQLx
- CMake -- only needed if `aws_lc_rs` has build issues; `rust_crypto` is the fallback

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | None -- Cargo.toml `[dev-dependencies]` section |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FOUND-01 | Server starts and connects to PostgreSQL | integration | `cargo test test_db_connection -- --ignored` | No -- Wave 0 |
| FOUND-09 | AppError returns correct HTTP status codes | unit | `cargo test test_app_error` | No -- Wave 0 |
| AUTH-01 | Login returns valid JWT with correct claims | integration | `cargo test test_login -- --ignored` | No -- Wave 0 |
| AUTH-07 | BCrypt hash from .NET verified in Rust | unit | `cargo test test_bcrypt_compatibility` | No -- Wave 0 |
| GRAN-01 | Granjas filtered by role | integration | `cargo test test_granjas_role_filter -- --ignored` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo check && cargo test --lib`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `tests/common/mod.rs` -- shared test setup (PgPool creation with test DB)
- [ ] Unit test for BCrypt .NET hash compatibility
- [ ] Unit test for AppError response codes
- [ ] Integration test fixtures for auth and granjas

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | YES | bcrypt password hashing (cost factor matches .NET), JWT with 8h expiry |
| V3 Session Management | YES (stateless) | JWT tokens, no server-side session state |
| V4 Access Control | YES | Role-based authorization (Administrador, Produtor, Financeiro) enforced at service layer |
| V5 Input Validation | YES | validator crate with derive macros on DTOs |
| V6 Cryptography | YES | HMAC-SHA256 for JWT signing via jsonwebtoken crate -- never hand-roll |

### Known Threat Patterns for Rust + Actix-web + PostgreSQL

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| SQL injection | Tampering | Parameterized queries via SQLx `.bind()` -- never string interpolation |
| JWT secret exposure | Information Disclosure | Load from env var, never hardcode in source |
| BCrypt timing attack | Tampering | bcrypt::verify handles constant-time comparison internally |
| Missing auth on endpoints | Elevation of Privilege | JWT Claims extractor as required handler parameter -- compile-time enforcement |
| CORS misconfiguration | Spoofing | Explicit allowed origins list, not wildcard in production |

## Database Schema Reference (Phase 1 Tables)

Tables actively used in Phase 1 (auth + granjas + audit):

| Table Name (quoted) | Columns | Notes |
|---------------------|---------|-------|
| `"Perfis"` | Id, Nome | Seed data: 1=Administrador, 2=Produtor, 3=Financeiro |
| `"Usuarios"` | Id, Codigo, Nome, Email, SenhaHash, PerfilId | FK to Perfis |
| `"FinanceiroProdutor"` | FinanceiroId, ProdutorId | Composite PK, junction table |
| `"Granjas"` | Id, Codigo, Nome, Localizacao, UsuarioId | FK to Usuarios |
| `"LogsAuditoria"` | Id, Timestamp, UsuarioId, UsuarioEmail, Acao, Detalhes | No FK constraints |

All other 12 tables (Lotes, TransacoesFinanceiras, Produtos, Sensores, etc.) need model structs defined in Phase 1 (FOUND-07) but their handlers/services are Phase 2-3 scope.

## Sources

### Primary (HIGH confidence)
- [crates.io/crates/actix-web](https://crates.io/crates/actix-web) - Version 4.13.0 verified
- [crates.io/crates/sqlx](https://crates.io/crates/sqlx) - Version 0.8.6 verified
- [crates.io/crates/jsonwebtoken](https://crates.io/crates/jsonwebtoken) - Version 10.3.0 verified, v10 crypto backend requirement
- [crates.io/crates/bcrypt](https://crates.io/crates/bcrypt) - Version 0.17.1 verified
- [crates.io/crates/utoipa](https://crates.io/crates/utoipa) - Version 5.4.0 verified
- [crates.io/crates/utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) - Version 9.x verified
- [crates.io/crates/actix-cors](https://crates.io/crates/actix-cors) - Version 0.7.1 verified
- [crates.io/crates/validator](https://crates.io/crates/validator) - Version 0.20.0 verified
- [docs/schema.sql](docs/schema.sql) - Full database schema with PascalCase quoted identifiers
- [AzureAD OutboundClaimTypeMap wiki](https://github.com/AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet/wiki/OutboundClaimTypeMap) - JWT claim name mappings verified

### Secondary (MEDIUM confidence)
- [Keats/rust-bcrypt GitHub](https://github.com/Keats/rust-bcrypt) - BCrypt $2a$ compatibility information
- [Orhun's Blog on JWT migration](https://blog.orhun.dev/upgrading-rust-jwt/) - jsonwebtoken v9 to v10 migration notes

### Tertiary (LOW confidence)
- None -- all critical claims verified

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all crate versions verified against crates.io
- Architecture: HIGH -- patterns derived from project decisions (D-01 through D-08) and verified against existing .NET code
- Pitfalls: HIGH -- PascalCase issue verified from actual schema.sql, JWT claims verified from AzureAD docs
- BCrypt compatibility: HIGH -- both .NET and Rust use standard $2a$ format

**Research date:** 2026-04-06
**Valid until:** 2026-05-06 (stable ecosystem, 30-day window appropriate)
