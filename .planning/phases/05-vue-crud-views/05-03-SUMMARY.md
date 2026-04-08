---
phase: 05-vue-crud-views
plan: 03
subsystem: frontend-views
tags: [vue, vuetify, vue-chartjs, crud, charts, consumo, pesagem, sensores, sanitario]
dependency_graph:
  requires: [05-02]
  provides: [ConsumoView, PesagemView, SensoresView, SanitarioView]
  affects: [router]
tech_stack:
  added: []
  patterns: [vue-chartjs-line, lote-selector-watch, sensor-crud-readings]
key_files:
  created:
    - granjatech-frontend/src/views/ConsumoView.vue
    - granjatech-frontend/src/views/PesagemView.vue
    - granjatech-frontend/src/views/SensoresView.vue
    - granjatech-frontend/src/views/SanitarioView.vue
  modified:
    - granjatech-frontend/src/router/index.ts
decisions:
  - Used vue-chartjs Line component with Chart.js registration pattern for all chart views
  - SensoresView uses click:row handler for sensor selection instead of separate button
  - SanitarioView vaccination schedule fetched once on mount (not lote-dependent)
metrics:
  duration: 5min
  completed: 2026-04-08
---

# Phase 05 Plan 03: Data-Intensive Views with Charts Summary

Four data-intensive views with vue-chartjs Line charts: ConsumoView (feed/water consumption with dual line charts), PesagemView (weighings with weight curve chart), SensoresView (sensors CRUD with readings line chart), and SanitarioView (sanitary events with vaccination schedule).

## Completed Tasks

| # | Task | Commit | Key Files |
|---|------|--------|-----------|
| 1 | ConsumoView + PesagemView (lote selector + line charts) | b1c6bb0 | ConsumoView.vue, PesagemView.vue |
| 2 | SensoresView + SanitarioView + router update for 4 routes | 22a470f | SensoresView.vue, SanitarioView.vue, router/index.ts |

## What Was Built

### ConsumoView.vue
- Lote selector (v-select) with watch-based data refresh
- 4 summary cards: Total Racao, Total Agua, Media Racao/Dia, Media Agua/Dia
- Side-by-side data tables for racao and agua consumption
- Two registration dialogs (one for racao, one for agua) with validation rules
- Two vue-chartjs Line charts: racao (green) and agua (blue) with fill

### PesagemView.vue
- Lote selector with watch-based data refresh
- 3 summary cards: Total Pesagens, Peso Medio Atual, Ganho Medio Diario
- Data table with columns: Data, Peso Medio, Qtd Amostras, Observacoes
- Registration dialog with peso and amostras validation
- vue-chartjs Line chart for weight curve (orange) with fill

### SensoresView.vue
- Sensors CRUD: create dialog (nome, tipo, localizacao, granjaId) and delete with confirmation
- Clickable sensor rows to view readings
- Readings data table and registration dialog
- vue-chartjs Line chart for readings over time (purple)
- Granjas fetched on mount for dropdown in create dialog

### SanitarioView.vue
- Lote selector with watch-based data refresh
- 4 summary cards: Total Eventos, Ultimo Evento, Vacinas Aplicadas, Medicamentos
- Events data table with v-chip color-coded tipo (Vacinacao=success, Medicacao=warning, Exame=info)
- Event registration dialog with tipo selector
- Vaccination schedule table (static reference, fetched once on mount)

### Router Updates
- /consumo -> ConsumoView.vue
- /pesagem -> PesagemView.vue
- /sensores -> SensoresView.vue
- /sanitario -> SanitarioView.vue
- Remaining routes (avicultura, relatorios) still use PlaceholderView

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed TypeScript error in SensoresView click:row handler**
- **Found during:** Task 2
- **Issue:** Inline TypeScript type annotations in Vue template @click:row caused TS1005 parse errors
- **Fix:** Changed to `any` typed parameters in template, keeping type safety in the handler logic
- **Files modified:** granjatech-frontend/src/views/SensoresView.vue
- **Commit:** 22a470f

## Threat Mitigations Applied

- T-05-09: Vuetify `:rules` on ConsumoView quantidade fields validates min 0.01
- T-05-10: Vuetify `:rules` on SensoresView leitura valor validates as number
- T-05-12: SanitarioView loteId comes from v-select populated by authenticated API call

## Verification

- `npx vue-tsc --noEmit` passes with 0 errors
- `npx vite build` exits 0 successfully
- All 4 view files contain `<script setup lang="ts">`
- ConsumoView and PesagemView import and use vue-chartjs Line component
- SensoresView has sensor CRUD + readings chart
- SanitarioView has events + vaccination schedule
- Router updated for 4 routes, remaining 2 still use PlaceholderView
