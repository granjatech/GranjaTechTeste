---
phase: 05-vue-crud-views
fixed_at: 2026-04-08T15:45:00Z
review_path: .planning/phases/05-vue-crud-views/05-REVIEW.md
iteration: 2
findings_in_scope: 13
fixed: 12
skipped: 1
status: partial
---

# Phase 5: Code Review Fix Report

**Fixed at:** 2026-04-08T15:45:00Z
**Source review:** .planning/phases/05-vue-crud-views/05-REVIEW.md
**Iteration:** 2

**Summary:**
- Findings in scope: 13 (1 critical, 7 warnings, 5 info)
- Fixed: 12
- Skipped: 1

## Fixed Issues

### CR-01: Missing Route-Level Authorization for Admin-Only Pages

**Files modified:** `granjatech-frontend/src/router/index.ts`
**Commit:** 4ad4e7e
**Applied fix:** Added `roles: ['Administrador']` meta to `/usuarios` and `/auditoria` routes. Added role-based guard in `router.beforeEach` that redirects to `/` when the user's role is not in `to.meta.roles`.

### WR-01: Refresh Button is a No-Op

**Files modified:** `granjatech-frontend/src/views/AviculturaView.vue`
**Commit:** 655fd9f
**Applied fix:** Extracted the dashboard fetch logic from the `watch` callback into a named `fetchDashboardData(loteId)` function. Updated the refresh button click handler to call `fetchDashboardData(selectedLoteId)` directly instead of the no-op self-assignment.

### WR-02: Missing `await` on `fetchData()` After Mutations

**Files modified:** `granjatech-frontend/src/views/EstoqueView.vue`, `granjatech-frontend/src/views/FinanceiroView.vue`, `granjatech-frontend/src/views/GranjasView.vue`, `granjatech-frontend/src/views/LotesView.vue`
**Commit:** 4e70339
**Applied fix:** Added `await` before `fetchData()` calls in `handleSubmit` functions across all four views so errors from the refresh are caught by the surrounding try/catch.

### WR-03: Password Validation Too Weak

**Files modified:** `granjatech-frontend/src/views/UsuariosView.vue`
**Commit:** af0793f
**Applied fix:** Added minimum length validation rule (`v.length >= 6`) to `senhaCreateRule` array.

### WR-04: Email Validation Regex Too Permissive

**Files modified:** `granjatech-frontend/src/views/UsuariosView.vue`, `granjatech-frontend/src/views/ProfileView.vue`
**Commit:** ddd9023
**Applied fix:** Replaced loose `/.+@.+\..+/` regex with stricter `/^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/` pattern in both views.

### WR-05: Unhandled Edge Case in `gerarRelatorio` -- Missing `finally` Block Reset

**Files modified:** `granjatech-frontend/src/views/RelatoriosView.vue`
**Commit:** bdf9b60
**Applied fix:** Moved all validation checks (cases 0-5) into a separate switch before `loading.value = true`. The try block now only contains fetch logic, ensuring `finally` always runs when loading is set.

### WR-06: GranjasView Edit/Delete Buttons Shown to All Roles

**Files modified:** `granjatech-frontend/src/views/GranjasView.vue`
**Commit:** cc6d553
**Applied fix:** Wrapped edit and delete action buttons in `<template v-if="canCreate">` to hide them from Financeiro users who lack permission.

### WR-07: `formRef.value.validate()` Called Without Null Check

**Files modified:** `granjatech-frontend/src/views/EstoqueView.vue`, `granjatech-frontend/src/views/FinanceiroView.vue`, `granjatech-frontend/src/views/GranjasView.vue`, `granjatech-frontend/src/views/LotesView.vue`, `granjatech-frontend/src/views/UsuariosView.vue`
**Commit:** 6519f67
**Applied fix:** Added `if (!formRef.value) return` guard before `formRef.value.validate()` in all five views.

### IN-01: Console.error Calls Should Be Removed for Production

**Files modified:** `AuditoriaView.vue`, `AviculturaView.vue`, `ConsumoView.vue`, `DashboardView.vue`, `EstoqueView.vue`, `FinanceiroView.vue`, `GranjasView.vue`, `LotesView.vue`, `PesagemView.vue`, `ProfileView.vue`, `RelatoriosView.vue`, `SanitarioView.vue`, `SensoresView.vue`, `UsuariosView.vue`
**Commit:** 309a1b2
**Applied fix:** Removed all 25 `console.error` calls across 14 view files. Each catch block already sets an error state variable (`error.value` or calls `showSnackbar`), so the console.error calls were redundant. For the one catch block in SanitarioView that only had console.error (cronograma fetch), replaced with a comment noting it is supplementary data that fails silently.

### IN-02: Duplicated formatDate and formatCurrency Functions

**Files modified:** `granjatech-frontend/src/composables/useFormatters.ts` (new), `AuditoriaView.vue`, `AviculturaView.vue`, `ConsumoView.vue`, `DashboardView.vue`, `EstoqueView.vue`, `FinanceiroView.vue`, `LotesView.vue`, `PesagemView.vue`, `RelatoriosView.vue`, `SanitarioView.vue`, `SensoresView.vue`
**Commits:** 829b36b (composable creation), 309a1b2 (view updates)
**Applied fix:** Created `useFormatters` composable with `formatDate`, `formatDateTime`, and `formatCurrency` functions. Updated all 11 views that had local duplicates to import from the composable. The composable uses consistent pt-BR locale formatting with null-safety. AuditoriaView and SensoresView use `formatDateTime` (date+time), all others use `formatDate` (date only).

### IN-03: TypeScript any Casts Used in Multiple Places

**Files modified:** `AviculturaView.vue`, `LotesView.vue`, `RelatoriosView.vue`
**Commit:** ed9f626
**Applied fix:** In AviculturaView: replaced `any` type on `ComparacaoIndustria` fields with `ComparacaoMetrica`, added optional `metrica` and `valor` fields to `ComparacaoMetrica` interface for API response variants, replaced all `(m: any)` map callbacks with `(m: ComparacaoMetrica)`, replaced `catch (e: any)` with `catch`. In LotesView: replaced `payload: any` with `Record<string, string | number | null>`, replaced `catch (err: any)` with `catch (err: unknown)` using typed assertion. In RelatoriosView: replaced `response: any` with `{ data: unknown } | undefined`, replaced `catch (e: any)` with `catch (e: unknown)` using typed assertion, replaced `n(v: any)` with `n(v: unknown)`. Remaining `any` in Vuetify template slot destructuring left as-is since those require complex generic typing.

### IN-05: Hardcoded Profile ID Values

**Files modified:** `granjatech-frontend/src/constants/perfis.ts` (new), `UsuariosView.vue`
**Commit:** 6ef52eb
**Applied fix:** Created shared constants module `src/constants/perfis.ts` with `PERFIL_IDS` and `PERFIL_OPTIONS` exports. The constants document the backend contract (seeded database values) with clear comments. Updated UsuariosView to import `PERFIL_OPTIONS` instead of hardcoding the array.

## Skipped Issues

### IN-04: Unused Import -- computed in ConsumoView

**File:** `granjatech-frontend/src/views/ConsumoView.vue:2`
**Reason:** Reviewer explicitly marked this as "Disregard -- no issue" after determining `computed` is correctly used.
**Original issue:** `computed` import appeared potentially unused, but reviewer confirmed it is used.

---

_Fixed: 2026-04-08T15:45:00Z_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 2_
