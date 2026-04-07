---
phase: 02-complete-backend-crud
plan: 02
subsystem: backend-services
tags: [rust, actix-web, sqlx, financas, consumo, pesagem, sanitario]
dependency_graph:
  requires: [02-01]
  provides: [financas-crud, consumo-crud, pesagem-crud, sanitario-crud]
  affects: [02-03]
tech_stack:
  added: []
  patterns: [service-struct-pattern, handler-role-check, utoipa-annotations, join-helper-row]
key_files:
  created:
    - granjatech-api/src/services/financas_service.rs
    - granjatech-api/src/services/consumo_service.rs
    - granjatech-api/src/services/pesagem_service.rs
    - granjatech-api/src/services/sanitario_service.rs
    - granjatech-api/src/handlers/financas.rs
    - granjatech-api/src/handlers/consumo.rs
    - granjatech-api/src/handlers/pesagem.rs
    - granjatech-api/src/handlers/sanitario.rs
  modified:
    - granjatech-api/src/services/mod.rs
    - granjatech-api/src/handlers/mod.rs
    - granjatech-api/src/main.rs
decisions:
  - Route ordering in Actix: static paths (/resumo, /cronograma-vacinacao) registered before dynamic paths (/{loteId}) to avoid route shadowing
  - Financeiro data filtering uses subquery pattern (IN SELECT) rather than JOIN for cleaner SQL
metrics:
  duration: 5m49s
  completed: 2026-04-07T14:03:38Z
  tasks_completed: 2
  tasks_total: 2
  files_created: 8
  files_modified: 3
---

# Phase 02 Plan 02: Financas/Consumo/Pesagem/Sanitario Summary

Financial transactions CRUD with 5-minute edit window and role hierarchy, feed/water consumption with per-bird computed metrics, weekly weighings with daily growth rate, and sanitary events with vaccination schedule -- all using parameterized sqlx queries with audit logging.

## Task Results

| Task | Name | Commit | Status |
|------|------|--------|--------|
| 1 | Implement FinancasService, ConsumoService, PesagemService, SanitarioService | 657b4f7 | Done |
| 2 | Implement handlers and register routes | 981c5a5 | Done |

## Implementation Details

### Task 1: Four Service Files

**FinancasService** (`financas_service.rs`):
- `get_all`: JOIN query across TransacoesFinanceiras/Lotes/Granjas/Usuarios with Financeiro filtering via FinanceiroProdutor subquery
- `create`: INSERT with LoteId normalization (0 -> NULL), returns full joined DTO
- `update`: 5-minute edit window check (Admin exempt), hierarchy check (Financeiro cannot edit Admin's transactions)
- `delete`: Admin-only with existence verification

**ConsumoService** (`consumo_service.rs`):
- `create_racao`/`create_agua`: INSERT RETURNING with computed per-bird metrics (gramas/ml)
- `list_racao`/`list_agua`: SELECT with computed metrics per row
- `resumo`: Aggregate queries for totals, averages, and last dates

**PesagemService** (`pesagem_service.rs`):
- `create`: INSERT RETURNING with computed `ganho_medio_diario` (ganho_semanal / 7)
- `list`: Ordered by SemanaVida ASC with computed daily gain
- `resumo`: Latest/earliest peso, ganho_total, uniformidade (CV), total count

**SanitarioService** (`sanitario_service.rs`):
- `create`: Full 15-column INSERT RETURNING
- `list`: Optional `tipo_evento` filter via conditional SQL
- `resumo`: Count by type, total cost, cost per bird, upcoming carencia actions
- `cronograma_vacinacao`: Static hardcoded 5-item vaccination schedule (Marek, Newcastle, Gumboro)

### Task 2: Handlers and Routes

16 new handler functions with utoipa path annotations:
- **Financas (4)**: GET/POST (Admin+Financeiro), PUT (Admin+Financeiro), DELETE (Admin only)
- **Consumo (5)**: All Admin+Produtor -- POST racao, POST agua, GET racao/{loteId}, GET agua/{loteId}, GET resumo/{loteId}
- **Pesagem (3)**: All Admin+Produtor -- POST, GET /{loteId}, GET /resumo/{loteId}
- **Sanitario (4)**: Admin+Produtor -- POST, GET /{loteId} (optional tipo_evento query), GET /resumo/{loteId}, GET /cronograma-vacinacao

Route registration uses static-before-dynamic ordering to prevent path shadowing.

## Deviations from Plan

None -- plan executed exactly as written.

## Threat Mitigations Applied

| Threat ID | Mitigation | Implementation |
|-----------|-----------|----------------|
| T-02-03 | 5-minute edit window + role hierarchy | `financas_service.rs` update method checks Duration::minutes(5) and Financeiro vs Admin creator |
| T-02-04 | Admin-only delete | `financas.rs` handler checks `claims.role == "Administrador"` |
| T-02-05 | Role check + validation | All consumo/pesagem handlers check Admin+Produtor role, validate input via validator crate |
| T-02-06 | Financeiro data filtering | `financas_service.rs` get_all uses FinanceiroProdutor subquery to limit visible transactions |
| T-02-07 | Parameterized SQL | All queries use `.bind()` parameters, no string interpolation in SQL |
| T-02-08 | Audit logging | Every CUD operation calls `AuditoriaService::registrar_log` |

## Self-Check: PASSED

All 8 created files exist. Both commit hashes (657b4f7, 981c5a5) verified in git log. Compilation passes with zero errors.
