# Phase 2: Complete Backend CRUD - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-04-07
**Phase:** 02-complete-backend-crud
**Areas discussed:** Plan grouping, Lote calculations, Audit automation

---

## Plan Grouping

| Option | Description | Selected |
|--------|-------------|----------|
| By complexity | Plan 1: Lotes + Dashboard. Plan 2: Financas + Consumo + Pesagem + Sanitario. Plan 3: Sensores + Estoque + Auditoria + Profile. | ✓ |
| By dependency | Plan 1: Lotes + Consumo + Pesagem + Sanitario. Plan 2: Financas + Dashboard. Plan 3: Sensores + Estoque + Auditoria + Profile. | |
| Heavy + light + light | Plan 1: Lotes + Dashboard + Financas. Plan 2: Consumo + Pesagem + Sanitario + Sensores. Plan 3: Estoque + Auditoria + Profile. | |

**User's choice:** By complexity (Recommended)
**Notes:** Groups the most complex domain (Lotes with calculated properties) with Dashboard aggregation in Plan 1, standard lote-dependent CRUD in Plan 2, and independent/simple domains in Plan 3.

---

## Lote Calculations

| Option | Description | Selected |
|--------|-------------|----------|
| Rust code | Compute in Rust after fetching raw data, matching .NET pattern with calculated properties. | ✓ |
| SQL queries | Compute in SQL with expressions in SELECT. | |
| You decide | Claude picks based on .NET implementation. | |

**User's choice:** Rust code (Recommended)
**Notes:** Matches the .NET Lote entity pattern where CalcularIEP(), Viabilidade, and ConversaoAlimentar are computed properties on the entity struct.

---

## Audit Automation

| Option | Description | Selected |
|--------|-------------|----------|
| Manual calls | Explicit AuditoriaService::registrar_log() calls in each service method after CUD ops. | ✓ |
| Helper trait | Create a trait/macro wrapping CUD operations with auto-logging. | |
| You decide | Claude picks best balance of reuse and simplicity. | |

**User's choice:** Manual calls (Recommended)
**Notes:** Same pattern as Phase 1 GranjaService. Simple, predictable, matches .NET exactly.

---

## Claude's Discretion

- Internal service method organization
- Query optimization for Dashboard aggregation
- DTO field ordering
- Handler route grouping in main.rs

## Deferred Ideas

None -- discussion stayed within phase scope.
