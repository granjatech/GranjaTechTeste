# Phase 1: Rust Foundation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-06
**Phase:** 01-Rust Foundation
**Areas discussed:** Project structure, SQLx query style, Naming language, Error handling, JWT token format, Service layer pattern, API response parity, Config & env setup

---

## Project Structure

| Option | Description | Selected |
|--------|-------------|----------|
| Single crate | One Cargo project with modules (models/, dto/, handlers/, services/, middleware/). Matches migration plan. | ✓ |
| Cargo workspace | Multiple crates mirroring .NET layers (domain, application, infrastructure, api). | |
| Hybrid | Single crate now, refactor to workspace later if needed. | |

**User's choice:** Single crate
**Notes:** Matches the migration plan. Simpler to set up, faster compile for a project this size (~4.5k lines).

---

## SQLx Query Style

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime queries | sqlx::query_as with string SQL. No DATABASE_URL needed at compile time. | ✓ |
| Compile-time checked | sqlx::query! macro. Validates SQL at compile time but requires DB connection during build. | |
| Offline mode | Compile-time checked with .sqlx/ cache files committed to git. | |

**User's choice:** Runtime queries
**Notes:** Easier CI/CD, works in Docker builds without a running database.

---

## Naming Language

| Option | Description | Selected |
|--------|-------------|----------|
| Portuguese | Keep Portuguese names matching DB columns and .NET code. 1:1 struct-to-column mapping. | ✓ |
| English | Translate all names to English. Requires #[sqlx(rename)] everywhere. | |
| Mixed | Struct fields in Portuguese, local variables in English. | |

**User's choice:** Portuguese
**Notes:** Easiest migration — no rename attributes needed.

---

## Error Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Simple HTTP-mapped | 5 variants (NotFound, BadRequest, Unauthorized, Forbidden, Internal) with String messages. | ✓ |
| Domain-specific variants | More variants per domain (DuplicateEmail, InvalidCredentials, GranjaNotFound, etc.). | |
| You decide | Let Claude pick during implementation. | |

**User's choice:** Simple HTTP-mapped
**Notes:** Covers all current .NET patterns with minimal boilerplate.

---

## JWT Token Format

| Option | Description | Selected |
|--------|-------------|----------|
| Match .NET claims exactly | Same claim names (nameid, email, role), same signing, same 8h expiration. Token interop. | ✓ |
| Standard JWT claims | Use standard claim names (sub, email, role). Breaks token interop. | |
| You decide | Let Claude analyze actual .NET token structure and match. | |

**User's choice:** Match .NET claims exactly
**Notes:** Existing .NET-issued tokens should work in Rust and vice versa.

---

## Service Layer Pattern

| Option | Description | Selected |
|--------|-------------|----------|
| Direct structs | Plain structs with impl blocks. No traits. PgPool passed as parameter. | ✓ |
| Traits + impl | Mirror .NET pattern with trait definitions and implementations. | |
| Free functions | No structs — just async functions in service modules. | |

**User's choice:** Direct structs
**Notes:** Services have no state, tests aren't in v1 scope. Simpler is better.

---

## API Response Parity

| Option | Description | Selected |
|--------|-------------|----------|
| Replicate exactly | Match every response code and format from .NET, including inconsistencies. | |
| Normalize responses | Fix inconsistencies: 201 for creates, consistent NotFound format, proper HTTP semantics. | ✓ |
| You decide | Let Claude analyze each endpoint and replicate exact behavior. | |

**User's choice:** Normalize responses
**Notes:** Vue frontend will be built against the cleaner API from the start.

---

## Config & Env Setup

| Option | Description | Selected |
|--------|-------------|----------|
| Single Config struct | One flat struct loaded from .env via dotenvy + std::env::var. | ✓ |
| Nested config structs | Separate structs per concern (JwtConfig, DbConfig, CorsConfig). | |
| You decide | Let Claude pick based on actual config values needed. | |

**User's choice:** Single Config struct
**Notes:** Sufficient for ~6 environment variables.

---

## Claude's Discretion

- Tracing/logging configuration and log levels
- Swagger/utoipa annotation style and grouping
- CORS middleware configuration details
- Internal module organization within models/ and dto/

## Deferred Ideas

None — discussion stayed within phase scope.
