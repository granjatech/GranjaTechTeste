---
phase: 05-vue-crud-views
reviewed: 2026-04-08T14:30:00Z
depth: standard
files_reviewed: 17
files_reviewed_list:
  - granjatech-frontend/src/composables/useExport.ts
  - granjatech-frontend/src/router/index.ts
  - granjatech-frontend/src/views/AuditoriaView.vue
  - granjatech-frontend/src/views/AviculturaView.vue
  - granjatech-frontend/src/views/ConsumoView.vue
  - granjatech-frontend/src/views/DashboardView.vue
  - granjatech-frontend/src/views/EstoqueView.vue
  - granjatech-frontend/src/views/FinanceiroView.vue
  - granjatech-frontend/src/views/GranjasView.vue
  - granjatech-frontend/src/views/LotesView.vue
  - granjatech-frontend/src/views/PesagemView.vue
  - granjatech-frontend/src/views/ProfileView.vue
  - granjatech-frontend/src/views/RelatoriosView.vue
  - granjatech-frontend/src/views/SanitarioView.vue
  - granjatech-frontend/src/views/SensoresView.vue
  - granjatech-frontend/src/views/UsuariosView.vue
  - granjatech-frontend/package.json
findings:
  critical: 1
  warning: 7
  info: 5
  total: 13
status: issues_found
---

# Phase 5: Code Review Report

**Reviewed:** 2026-04-08T14:30:00Z
**Depth:** standard
**Files Reviewed:** 17
**Status:** issues_found

## Summary

Reviewed 17 files comprising the Vue 3 frontend views, router, export composable, and package.json for the GranjaTech migration. The views follow a consistent pattern with good TypeScript typing, proper error handling, and appropriate use of Vuetify components. The code is well-structured and maintains parity with the original React frontend.

Key concerns: one route authorization gap (Usuarios/Auditoria accessible to all roles), a refresh button that is a no-op, and several `console.error` calls that should be removed for production. No hardcoded secrets or injection vulnerabilities were found.

## Critical Issues

### CR-01: Missing Route-Level Authorization for Admin-Only Pages

**File:** `granjatech-frontend/src/router/index.ts:77-88`
**Issue:** The `/usuarios` and `/auditoria` routes only check `requiresAuth: true` but have no role-based guard. In the original .NET backend, these endpoints are restricted to `Administrador` role. While `UsuariosView.vue` has a client-side `isAdmin` check (line 17), the route itself allows any authenticated user to navigate there, and `AuditoriaView.vue` has no role check at all. A non-admin user can access the auditoria page and see all audit logs, which is an authorization gap.
**Fix:** Add role-based meta to these routes and enforce it in the `beforeEach` guard:
```typescript
{
  path: '/usuarios',
  name: 'Usuarios',
  component: () => import('@/views/UsuariosView.vue'),
  meta: { requiresAuth: true, roles: ['Administrador'] },
},
{
  path: '/auditoria',
  name: 'Auditoria',
  component: () => import('@/views/AuditoriaView.vue'),
  meta: { requiresAuth: true, roles: ['Administrador'] },
},
```
Then in the guard:
```typescript
router.beforeEach((to) => {
  const auth = useAuthStore()
  if (to.meta.requiresAuth !== false && !auth.isAuthenticated) {
    return { name: 'Login' }
  }
  if (to.meta.roles && !to.meta.roles.includes(auth.user?.role)) {
    return { path: '/' }
  }
  if (to.name === 'Login' && auth.isAuthenticated) {
    return { path: '/' }
  }
})
```

## Warnings

### WR-01: Refresh Button is a No-Op

**File:** `granjatech-frontend/src/views/AviculturaView.vue:333`
**Issue:** The refresh button executes `selectedLoteId && (selectedLoteId = selectedLoteId)` which reassigns the same value. Vue's `watch` on `selectedLoteId` will not trigger because the value has not changed (Vue uses `Object.is` comparison). The refresh button does nothing.
**Fix:** Extract the fetch logic into a named function and call it directly:
```typescript
async function refreshDashboard() {
  if (selectedLoteId.value) {
    // Trigger the watch by temporarily clearing and resetting,
    // or better: call the fetch function directly
    await fetchDashboardData(selectedLoteId.value)
  }
}
```

### WR-02: Missing `await` on `fetchData()` After Mutations

**File:** `granjatech-frontend/src/views/EstoqueView.vue:140`, `granjatech-frontend/src/views/FinanceiroView.vue:153`, `granjatech-frontend/src/views/GranjasView.vue:120`, `granjatech-frontend/src/views/LotesView.vue:219`
**Issue:** After successful create/update operations, `fetchData()` is called without `await`. This means the loading state and UI update happen asynchronously, and any error from `fetchData` will become an unhandled promise rejection since the surrounding `try/catch` has already exited.
**Fix:** Add `await` before `fetchData()`:
```typescript
dialogOpen.value = false
await fetchData()
```

### WR-03: Password Validation Too Weak

**File:** `granjatech-frontend/src/views/UsuariosView.vue:61`
**Issue:** The password creation rule only checks that the field is non-empty (`!!v || 'Senha obrigatoria'`). There is no minimum length or complexity requirement. The original .NET system may enforce password rules server-side, but the frontend should provide immediate feedback to the user.
**Fix:** Add minimum length validation:
```typescript
const senhaCreateRule = [
  (v: string) => !!v || 'Senha obrigatoria',
  (v: string) => v.length >= 6 || 'Senha deve ter no minimo 6 caracteres',
]
```

### WR-04: Email Validation Regex Too Permissive

**File:** `granjatech-frontend/src/views/ProfileView.vue:37`, `granjatech-frontend/src/views/UsuariosView.vue:58`
**Issue:** The email regex `/.+@.+\..+/` is very loose and accepts invalid emails like `a@b.c` or strings with spaces. While server-side validation should be the ultimate gate, client-side validation should be reasonably strict.
**Fix:** Use a more standard pattern:
```typescript
(v: string) => /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/.test(v) || 'Email invalido'
```

### WR-05: Unhandled Edge Case in `gerarRelatorio` -- Missing `finally` Block Reset

**File:** `granjatech-frontend/src/views/RelatoriosView.vue:100-101`
**Issue:** When validation fails (e.g., `error.value = 'Selecione uma granja'; return`), the `loading.value` is set to `true` on line 92 but never reset to `false` because the early `return` bypasses the `finally` block. The UI will show an infinite loading state.
**Fix:** Move the validation checks before setting `loading.value = true`, or reset loading in the early returns:
```typescript
async function gerarRelatorio() {
  reportData.value = null
  error.value = ''

  // Validate before setting loading
  switch (activeTab.value) {
    case 0:
      if (!granjaId.value) { error.value = 'Selecione uma granja'; return }
      break
    // ... other cases
  }

  loading.value = true
  try {
    // ... fetch logic only
  } catch (e: any) {
    error.value = `Erro ao gerar relatorio: ${e.response?.data?.message || e.message}`
  } finally {
    loading.value = false
  }
}
```

### WR-06: GranjasView Edit/Delete Buttons Shown to All Roles

**File:** `granjatech-frontend/src/views/GranjasView.vue:221-236`
**Issue:** The "Nova Granja" button correctly checks `canCreate` (line 169, filtering out `Financeiro` role), but the edit and delete action buttons in the table are always rendered for all users regardless of role. A Financeiro user will see edit/delete buttons that will fail server-side with 403.
**Fix:** Guard the action buttons with the same role check:
```html
<template #item.actions="{ item }">
  <template v-if="canCreate">
    <v-btn icon="mdi-pencil" ... @click="openEdit(item)" />
    <v-btn icon="mdi-delete" ... @click="openDelete(item)" />
  </template>
</template>
```

### WR-07: `formRef.value.validate()` Called Without Null Check

**File:** `granjatech-frontend/src/views/EstoqueView.vue:116`, `granjatech-frontend/src/views/FinanceiroView.vue:131`, `granjatech-frontend/src/views/GranjasView.vue:101`, `granjatech-frontend/src/views/LotesView.vue:198`, `granjatech-frontend/src/views/UsuariosView.vue:108`
**Issue:** `formRef.value.validate()` is called without checking if `formRef.value` is defined. If the ref is not yet attached (e.g., due to a rendering race), this will throw a runtime error.
**Fix:** Add a guard:
```typescript
if (!formRef.value) return
const { valid } = await formRef.value.validate()
```

## Info

### IN-01: Console.error Calls Should Be Removed for Production

**File:** Multiple views (`AuditoriaView.vue:47`, `ConsumoView.vue:88,109,138,159`, `DashboardView.vue:113`, `EstoqueView.vue:173`, `GranjasView.vue:155`, `LotesView.vue:162,257`, `PesagemView.vue:80,99,135`, `ProfileView.vue:47`, `SanitarioView.vue:104,135,169`, `SensoresView.vue:103,118,146,163,179`, `UsuariosView.vue:163`)
**Issue:** Many `console.error` calls are present throughout the views. These should be removed or replaced with a proper logging service for production.
**Fix:** Remove `console.error` calls and rely on the error state variables already in use, or use a structured logging composable.

### IN-02: Duplicated `formatDate` and `formatCurrency` Functions

**File:** Multiple views (at least 10 views define their own `formatDate`)
**Issue:** `formatDate`, `formatCurrency`, and similar utility functions are duplicated across nearly every view. This creates maintenance burden.
**Fix:** Extract shared utility functions into a composable or utility module:
```typescript
// src/composables/useFormatters.ts
export function useFormatters() {
  function formatDate(dateStr: string): string { ... }
  function formatCurrency(value: number): string { ... }
  return { formatDate, formatCurrency }
}
```

### IN-03: TypeScript `any` Casts Used in Multiple Places

**File:** `granjatech-frontend/src/views/AviculturaView.vue:115,239-253`, `granjatech-frontend/src/views/LotesView.vue:203`, `granjatech-frontend/src/views/RelatoriosView.vue:97,452-463`
**Issue:** Several `any` type assertions are used, particularly in AviculturaView chart data functions and in LotesView's payload construction. This weakens type safety.
**Fix:** Define proper interfaces for the chart data structures and API payloads instead of using `any`.

### IN-04: Unused Import -- `computed` in ConsumoView

**File:** `granjatech-frontend/src/views/ConsumoView.vue:2`
**Issue:** `computed` is imported from Vue but `racaoChartData` and `aguaChartData` are the only computed usages. However, `watch` is also imported and used. This is not a bug but `computed` is correctly used here. Disregard -- no issue.

### IN-05: Hardcoded Profile ID Values

**File:** `granjatech-frontend/src/views/UsuariosView.vue:49-53`
**Issue:** Profile IDs are hardcoded (`Administrador: 1, Produtor: 2, Financeiro: 3`). If the database seed data changes, these will silently break.
**Fix:** Fetch profile options from the API endpoint instead of hardcoding, or define them as a shared constant with the backend contract.

---

_Reviewed: 2026-04-08T14:30:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
