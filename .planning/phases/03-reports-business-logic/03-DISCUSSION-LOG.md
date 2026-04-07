# Phase 3: Reports & Business Logic - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-07
**Phase:** 03-reports-business-logic
**Areas discussed:** Plan grouping, Avicultura computation, Cache strategy, Report query approach, Role-based access, Avicultura DTO completeness, Cache invalidation, Error response format

---

## Plan Grouping

| Option | Description | Selected |
|--------|-------------|----------|
| By domain | Plan 1: Avicultura (9 endpoints). Plan 2: Reports (6 endpoints). Plan 3: Cache + Health. | ✓ |
| By complexity | Plan 1: Health + Cache infra. Plan 2: Reports. Plan 3: Avicultura. Build foundational pieces first. | |
| Cache first, then features | Plan 1: Moka + Health. Plan 2: All endpoints (no cache). Plan 3: Wire cache in. | |

**User's choice:** By domain (Recommended)
**Notes:** None

---

## Avicultura Computation

### Query Style

| Option | Description | Selected |
|--------|-------------|----------|
| Mirror .NET pattern | Many small queries + Rust logic per method. Cache handles performance. | ✓ |
| Batch queries | Fewer DB round-trips. Better raw performance but harder parity verification. | |
| You decide | Claude picks best approach. | |

**User's choice:** Mirror .NET pattern (Recommended)

### Benchmarks

| Option | Description | Selected |
|--------|-------------|----------|
| Hardcoded constants | Same as .NET — const values in Rust code. | ✓ |
| Config file | Move benchmarks to TOML/JSON config. More flexible. | |
| You decide | Claude picks. | |

**User's choice:** Hardcoded constants (Recommended)

---

## Cache Strategy

### Cache API

| Option | Description | Selected |
|--------|-------------|----------|
| Full parity | All 5 methods wrapping moka::future::Cache. | ✓ |
| Minimal (getOrSet only) | Only the main use case. | |
| You decide | Claude picks. | |

**User's choice:** Full parity (Recommended)

### Cache Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Dashboard + Reports | Dashboard KPIs (5 min), avicultura dashboard (5 min), reports (10 min). | ✓ |
| Reports only | Only cache heavy report endpoints. | |
| You decide | Claude determines. | |

**User's choice:** Dashboard + Reports (Recommended)

---

## Report Query Approach

### Query Plan

| Option | Description | Selected |
|--------|-------------|----------|
| Multiple queries per report | Fetch base entity, then related data separately. | ✓ |
| JOINs with manual mapping | Large JOINs returning flat rows, manual assembly. | |
| You decide | Claude picks per-endpoint. | |

**User's choice:** Multiple queries per report (Recommended)

### Debug Routes

| Option | Description | Selected |
|--------|-------------|----------|
| Health only | Only /health. Skip debug endpoints. | ✓ |
| All debug endpoints | Replicate all for parity. | |
| You decide | Claude decides. | |

**User's choice:** Health only (Recommended)

---

## Role-Based Access

### Role Restrictions

| Option | Description | Selected |
|--------|-------------|----------|
| Exact .NET parity | Avicultura: Admin+Produtor. Reports: all 3 roles. No data filtering. | ✓ |
| Add data filtering | Also filter report data by user's granjas. | |
| You decide | Claude picks. | |

**User's choice:** Exact .NET parity (Recommended)

### Ownership Checks

| Option | Description | Selected |
|--------|-------------|----------|
| Match .NET (no check) | Any authorized user can query any loteId. | ✓ |
| Add ownership check | Verify lote belongs to user's granjas. | |
| You decide | Claude decides. | |

**User's choice:** Match .NET (no check) (Recommended)

---

## Avicultura DTO Completeness

| Option | Description | Selected |
|--------|-------------|----------|
| Create typed DTOs | Add MetricasLoteDto and DashboardAviculturaDto. Better Swagger docs. | ✓ |
| Use serde_json::json! | Inline JSON, matching .NET anonymous objects. No Swagger types. | |
| You decide | Claude picks for best Swagger docs. | |

**User's choice:** Create typed DTOs (Recommended)

---

## Cache Invalidation

| Option | Description | Selected |
|--------|-------------|----------|
| TTL-only | No write-through invalidation. Natural expiry. No Phase 2 changes. | ✓ |
| Write-through invalidation | Add cache.remove() in CRUD services. More accurate. | |
| You decide | Claude picks. | |

**User's choice:** TTL-only (Recommended)

---

## Error Response Format

| Option | Description | Selected |
|--------|-------------|----------|
| Normalize to AppError | Consistent {"message": "..."} for all endpoints. | ✓ |
| Replicate .NET inconsistency | Avicultura: {message, error}. Reports: {message}. | |
| You decide | Claude normalizes. | |

**User's choice:** Normalize to AppError (Recommended)

---

## Claude's Discretion

- Internal query optimization within multiple-query approach
- Moka cache configuration (max capacity, eviction policy)
- Avicultura service method ordering
- Report DTO field ordering for .NET anonymous objects

## Deferred Ideas

None — discussion stayed within phase scope.
