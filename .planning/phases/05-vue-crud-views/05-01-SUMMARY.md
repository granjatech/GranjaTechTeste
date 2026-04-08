---
phase: 05-vue-crud-views
plan: 01
subsystem: frontend-views
tags: [vue, vuetify, chartjs, crud, dashboard, profile, auditoria, usuarios]
dependency_graph:
  requires: [04-01, 04-02]
  provides: [DashboardView, ProfileView, AuditoriaView, UsuariosView, vue-chartjs-infra]
  affects: [router/index.ts, package.json]
tech_stack:
  added: [vue-chartjs, chart.js, jspdf, jspdf-autotable, xlsx]
  patterns: [KPI-cards, bar-chart, CRUD-dialog, role-gating, search-table, password-toggle]
key_files:
  created:
    - granjatech-frontend/src/views/DashboardView.vue
    - granjatech-frontend/src/views/ProfileView.vue
    - granjatech-frontend/src/views/AuditoriaView.vue
    - granjatech-frontend/src/views/UsuariosView.vue
  modified:
    - granjatech-frontend/package.json
    - granjatech-frontend/package-lock.json
    - granjatech-frontend/src/router/index.ts
decisions:
  - "Used vue-chartjs Bar component with ChartJS.register for tree-shaking"
  - "Kept LoadingSpinner for full-page loads, v-data-table :loading for table views"
  - "Used v-dialog with persistent prop for create/edit to prevent accidental close"
metrics:
  duration: 223s
  completed: "2026-04-08T10:54:52Z"
  tasks_completed: 2
  tasks_total: 2
  files_created: 4
  files_modified: 3
---

# Phase 05 Plan 01: Dashboard + Profile + Auditoria + Usuarios Summary

Installed chart/export dependencies (vue-chartjs, chart.js, jspdf, jspdf-autotable, xlsx) and implemented 4 views with full Vue 3 + Vuetify 3 parity to their React counterparts, including KPI cards with gradient backgrounds, bar chart for monthly revenue, profile edit with password change, read-only audit log table with action color chips, and admin-only user CRUD with dialog forms and delete confirmation.

## Task Completion

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Install chart/export deps + DashboardView + ProfileView + AuditoriaView | 9134793 | package.json, DashboardView.vue, ProfileView.vue, AuditoriaView.vue |
| 2 | UsuariosView + router update for all 4 views | b76af3b | UsuariosView.vue, router/index.ts |

## Implementation Details

### DashboardView
- 4 KPI cards (Total Granjas, Lotes Ativos, Total de Aves, Receita Total) with gradient backgrounds matching plan colors
- Bar chart using vue-chartjs with Entradas (green) and Saidas (red) datasets
- Promise.all for parallel API calls to /dashboard/kpis and /dashboard/resumo-mensal
- Hover effect on KPI cards with translateY transform

### ProfileView
- Two-column layout: profile edit (nome, email) and password change (senhaAtual, novaSenha, confirmaNovaSenha)
- Password visibility toggle icons on all password fields
- Vuetify form validation rules for required fields and email format
- Role info card showing perfil chip and associated granjas/produtores
- Success/error alerts for both forms

### AuditoriaView
- v-data-table with search bar, 25 items per page
- Color-coded action chips: criar=success, editar/atualizar=warning, excluir/deletar=error, login=info
- Date formatting with pt-BR locale
- Read-only -- no create/edit/delete operations

### UsuariosView
- Admin role gating: non-admin users see warning alert instead of table
- v-data-table with search, role color chips
- Create dialog: nome, email, senha (required), perfilId (v-select)
- Edit dialog: same fields minus senha
- Delete confirmation dialog with user name display
- Snackbar feedback for all CRUD operations
- API calls: GET /auth/usuarios, POST /auth/register, PUT /auth/usuarios/:id, DELETE /auth/usuarios/:id

### Router Updates
- 4 routes updated from PlaceholderView to real views: /, /usuarios, /auditoria, /perfil
- 10 routes remain on PlaceholderView for future plans

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed unused LoadingSpinner import from AuditoriaView**
- **Found during:** Task 1 verification
- **Issue:** vue-tsc flagged TS6133 unused import since v-data-table has built-in :loading
- **Fix:** Removed the import, kept v-data-table :loading instead
- **Files modified:** AuditoriaView.vue
- **Commit:** 9134793

## Verification

- `npx vue-tsc --noEmit` passes with 0 errors
- `npx vite build` succeeds in 1.83s
- All 4 view files contain `<script setup lang="ts">`
- Router has 4 real view imports and 10 PlaceholderView imports
- package.json contains vue-chartjs, chart.js, jspdf, jspdf-autotable, xlsx

## Known Stubs

None -- all views wire to real API endpoints. No placeholder data or TODO markers.

## Threat Mitigations Applied

| Threat ID | Mitigation |
|-----------|------------|
| T-05-01 | UsuariosView checks `auth.user?.role === 'Administrador'` client-side; backend enforces server-side |
| T-05-02 | ProfileView requires senhaAtual field; Vuetify rules validate matching passwords |
| T-05-04 | UsuariosView v-select constrains perfilId to [1,2,3]; backend validates on register/update |
