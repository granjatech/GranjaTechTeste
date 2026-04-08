---
phase: 05-vue-crud-views
plan: 02
subsystem: frontend-crud-views
tags: [vue, vuetify, crud, v-data-table, v-dialog]
dependency_graph:
  requires: [05-01]
  provides: [granjas-view, lotes-view, estoque-view, financeiro-view]
  affects: [router]
tech_stack:
  added: []
  patterns: [vue-sfc-crud-dialog, v-data-table-slots, computed-summaries]
key_files:
  created:
    - granjatech-frontend/src/views/GranjasView.vue
    - granjatech-frontend/src/views/LotesView.vue
    - granjatech-frontend/src/views/EstoqueView.vue
    - granjatech-frontend/src/views/FinanceiroView.vue
  modified:
    - granjatech-frontend/src/router/index.ts
decisions:
  - Used individual ref fields (not reactive object) for form state, matching UsuariosView pattern from plan 01
  - Mortalidade uses inline sub-dialog with embedded table + form, not a separate view
metrics:
  duration: 292s
  completed: "2026-04-08T11:04:06Z"
  tasks: 2
  files: 5
---

# Phase 05 Plan 02: CRUD Views (Granjas, Lotes, Estoque, Financeiro) Summary

Four standard CRUD views with v-data-table, v-dialog forms, delete confirmation, and snackbar feedback following the established pattern from plan 01's UsuariosView.

## One-liner

Four CRUD views with table+dialog pattern: granjas (role-gated), lotes (mortalidade sub-dialog), estoque (low-stock alerts), financeiro (summary cards)

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | GranjasView + LotesView | 764b3c0 | GranjasView.vue, LotesView.vue |
| 2 | EstoqueView + FinanceiroView + router | 11211ce | EstoqueView.vue, FinanceiroView.vue, router/index.ts |

## Implementation Details

### GranjasView
- Full CRUD via `/granjas` API endpoints
- `canCreate` computed hides create button for Financeiro role (T-05-05 mitigation)
- v-data-table with sortable columns: Codigo, Nome, Localizacao, Tipo Producao, Status
- v-dialog form with nome validation rule
- v-select for tipoProducao (Corte, Postura, Matrizes)

### LotesView
- Full CRUD via `/lotes` API endpoints
- Mortalidade sub-dialog: lists existing mortalidades + form to register new ones via `/lotes/{id}/mortalidades`
- v-data-table with status chip (Ativo=success), viabilidade formatted as percentage
- Granja dropdown populated from `/granjas` endpoint
- Quantity validation min 1 (T-05-07 mitigation)
- Status field only shown in edit mode

### EstoqueView
- Full CRUD via `/estoque` API endpoints
- Low-stock highlighting: red bold text when quantidadeAtual <= quantidadeMinima
- Price formatted as R$ currency
- Category and unit dropdowns with predefined options
- Optional fornecedor and observacoes fields

### FinanceiroView
- Full CRUD via `/financas` API endpoints
- Summary cards: Total Entradas (green), Total Saidas (red), Saldo (blue)
- v-chip color coding: Entrada=success, Saida=error
- Valor validation min R$ 0.01 (T-05-06 mitigation)
- Granja dropdown (optional) populated from `/granjas`
- Currency formatting with pt-BR locale

### Router
- Updated 4 routes from PlaceholderView to actual CRUD views
- Remaining 6 routes (sensores, consumo, pesagem, sanitario, avicultura, relatorios) still use PlaceholderView

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Installed chart.js and vue-chartjs dependencies**
- **Found during:** Task 1 verification
- **Issue:** DashboardView from plan 01 imports chart.js/vue-chartjs but packages were not installed, causing vite build failure
- **Fix:** `npm install chart.js vue-chartjs` -- packages were already in package.json from plan 01 scaffolding
- **Files modified:** None (deps already declared)

**2. [Rule 1 - Bug] Removed unused LoteForm interface**
- **Found during:** Task 1 verification (vue-tsc TS6196)
- **Issue:** LoteForm interface declared but never used (form fields use individual refs)
- **Fix:** Replaced with comment explaining inline payload construction
- **Files modified:** LotesView.vue

## Known Stubs

None -- all views are fully wired to API endpoints with complete CRUD operations.

## Threat Mitigations Applied

| Threat ID | Mitigation |
|-----------|------------|
| T-05-05 | GranjasView `canCreate` computed excludes Financeiro from create button |
| T-05-06 | FinanceiroView valor field has `:rules` validating min 0.01 |
| T-05-07 | LotesView quantidadeInicial has `:rules` validating required + min 1 |
| T-05-08 | Accepted -- summary shows aggregates visible to authenticated user |

## Self-Check: PASSED

- All 5 files exist on disk
- Commits 764b3c0 and 11211ce verified in git log
- vite build exits 0
- vue-tsc --noEmit passes (no errors from plan 02 files)
