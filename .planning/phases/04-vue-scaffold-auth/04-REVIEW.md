---
phase: 04-vue-scaffold-auth
reviewed: 2026-04-07T12:00:00Z
depth: standard
files_reviewed: 13
files_reviewed_list:
  - granjatech-frontend/src/App.vue
  - granjatech-frontend/src/components/LoadingSpinner.vue
  - granjatech-frontend/src/components/PageContainer.vue
  - granjatech-frontend/src/components/ResponsiveNavigation.vue
  - granjatech-frontend/src/main.ts
  - granjatech-frontend/src/plugins/vuetify.ts
  - granjatech-frontend/src/router/index.ts
  - granjatech-frontend/src/services/api.ts
  - granjatech-frontend/src/stores/accessibility.ts
  - granjatech-frontend/src/stores/auth.ts
  - granjatech-frontend/src/views/LoginView.vue
  - granjatech-frontend/src/views/PlaceholderView.vue
  - granjatech-frontend/src/vite-env.d.ts
findings:
  critical: 1
  warning: 4
  info: 2
  total: 7
status: issues_found
---

# Phase 04: Code Review Report

**Reviewed:** 2026-04-07T12:00:00Z
**Depth:** standard
**Files Reviewed:** 13
**Status:** issues_found

## Summary

The Vue 3 scaffold with authentication is well-structured overall. The code follows Vue 3 Composition API conventions correctly, Pinia stores are properly defined, and the Vuetify theming matches the project color spec. However, there is a critical authorization gap in the router guard (routes are not role-protected, only auth-protected), and several warnings around token lifecycle management and the API error interceptor that could cause bugs in production.

## Critical Issues

### CR-01: Router guard lacks role-based route protection

**File:** `granjatech-frontend/src/router/index.ts:106-118`
**Issue:** The `beforeEach` guard only checks `requiresAuth` but performs no role-based access control. Any authenticated user (including `Financeiro` or `Produtor`) can navigate directly to `/usuarios` or `/auditoria` by typing the URL, bypassing the navigation drawer's role filtering. The original .NET system enforces role checks at both controller and UI levels -- the Vue migration must do the same.
**Fix:** Add a `roles` meta field to routes and enforce it in the guard:
```typescript
// In route definitions:
{
  path: '/usuarios',
  name: 'Usuarios',
  component: () => import('@/views/PlaceholderView.vue'),
  meta: { requiresAuth: true, roles: ['Administrador'] },
},

// In beforeEach:
router.beforeEach((to) => {
  const auth = useAuthStore()

  if (to.meta.requiresAuth !== false && !auth.isAuthenticated) {
    return { name: 'Login' }
  }

  if (to.name === 'Login' && auth.isAuthenticated) {
    return { path: '/' }
  }

  // Role-based guard
  const allowedRoles = to.meta.roles as string[] | undefined
  if (allowedRoles && auth.user?.role && !allowedRoles.includes(auth.user.role)) {
    return { path: '/' }
  }
})
```

## Warnings

### WR-01: Token expiry not rechecked after initial hydration

**File:** `granjatech-frontend/src/stores/auth.ts:34`
**Issue:** `isAuthenticated` is computed as `!!token.value && !!user.value` but never re-checks `user.value.exp` against current time. A token with an 8-hour expiry could expire mid-session, and the user would remain "authenticated" in the client until a 401 triggers the API interceptor. This creates a window where the UI shows authenticated state but API calls will fail.
**Fix:** Include an expiry check in the computed property:
```typescript
const isAuthenticated = computed(() => {
  if (!token.value || !user.value) return false
  return user.value.exp * 1000 > Date.now()
})
```

### WR-02: Login does not handle malformed token from server

**File:** `granjatech-frontend/src/stores/auth.ts:38-42`
**Issue:** The `login()` function calls `jwtDecode()` on the server response without try/catch. If the server returns an unexpected response shape (missing `token` field) or a malformed token, `jwtDecode` will throw, and the token will already be set in `token.value` and `localStorage`, leaving the store in an inconsistent state (token saved, but user is null).
**Fix:** Wrap the decode and validate before persisting:
```typescript
async function login(email: string, senha: string) {
  const response = await api.post('/auth/login', { email, senha })
  const newToken: string = response.data.token
  const decoded = jwtDecode<JwtPayload>(newToken) // throws if malformed
  token.value = newToken
  user.value = decoded
  localStorage.setItem('token', newToken)
}
```
Note: The current code already has this order (decode after assignment on line 40), so the fix is to move `jwtDecode` before the assignments:
```typescript
async function login(email: string, senha: string) {
  const response = await api.post('/auth/login', { email, senha })
  const newToken: string = response.data.token
  const decoded = jwtDecode<JwtPayload>(newToken)
  // Only persist after successful decode
  token.value = newToken
  user.value = decoded
  localStorage.setItem('token', newToken)
}
```

### WR-03: API 401 interceptor bypasses Vue router and leaves stale auth state

**File:** `granjatech-frontend/src/services/api.ts:22-25`
**Issue:** On a 401 response, the interceptor removes the token from localStorage and does a hard `window.location.href = '/login'` redirect. This bypasses the Vue router, causes a full page reload losing all Pinia store state, and does not call `auth.logout()`. The auth store's `user` ref retains stale data until the page reloads. This also prevents showing user-friendly error messages.
**Fix:** Use the router and auth store for a clean logout. Since circular imports can be tricky (api imports auth, auth imports api), consider an event-based approach:
```typescript
// Option A: Import router directly (router doesn't import api)
import router from '@/router'

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      router.push('/login')
    }
    return Promise.reject(error)
  }
)
```

### WR-04: Login form lacks email format validation

**File:** `granjatech-frontend/src/views/LoginView.vue:14-16`
**Issue:** `isFormValid` only checks that email and senha are non-empty strings. The `type="email"` attribute on the input provides browser-level validation, but since `@submit.prevent` is used, browser validation may not trigger in all cases. The original React app uses the same pattern, but adding basic email format validation prevents unnecessary API calls with obviously invalid input.
**Fix:** Add a minimal email pattern check:
```typescript
const isFormValid = computed(() => {
  return email.value.trim() !== '' 
    && senha.value.trim() !== '' 
    && email.value.includes('@')
})
```

## Info

### IN-01: Accessibility store uses `||` instead of `??` for nullish coalescing

**File:** `granjatech-frontend/src/stores/accessibility.ts:33`
**Issue:** `stored?.fontScale || DEFAULT_FONT_SCALE` uses logical OR, which would treat `0` as falsy. While the `clamp()` function prevents `0` from being stored, the nullish coalescing operator (`??`) expresses the intent more accurately and is safer against future changes to the clamp range.
**Fix:**
```typescript
const fontScale = ref(stored?.fontScale ?? DEFAULT_FONT_SCALE)
```

### IN-02: Catch-all route silently redirects to dashboard

**File:** `granjatech-frontend/src/router/index.ts:96-98`
**Issue:** The catch-all route `/:pathMatch(.*)*` redirects to `/` without any notification. Users who mistype a URL will silently land on the dashboard with no indication that their requested page was not found. Consider a 404 page for better UX.
**Fix:** Replace the redirect with a dedicated NotFoundView:
```typescript
{
  path: '/:pathMatch(.*)*',
  name: 'NotFound',
  component: () => import('@/views/NotFoundView.vue'),
  meta: { requiresAuth: false },
}
```

---

_Reviewed: 2026-04-07T12:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
