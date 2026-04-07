# Phase 4: Vue Scaffold + Auth - Research

**Researched:** 2026-04-07
**Domain:** Vue 3 + Vuetify 3 + TypeScript + Vite frontend scaffold with auth
**Confidence:** HIGH

## Summary

This phase creates a new Vue 3 frontend (`granjatech-frontend/`) that replicates the React app's authentication flow, navigation layout, accessibility features, and shared components. The React codebase serves as the complete reference implementation -- every behavior, localStorage key, color value, and navigation item has been extracted and documented below.

The Vue ecosystem is mature and well-documented. Vue 3 Composition API with `<script setup lang="ts">`, Pinia for state management, Vue Router 4 for navigation guards, and Vuetify 3 for the component library form the standard stack. All packages have been verified against npm registry as of today.

**Primary recommendation:** Scaffold the project with `npm create vuetify@latest`, which sets up Vue 3 + Vuetify 3 + Vite + TypeScript in one step. Then layer in Pinia stores, API service, router guards, and layout components following the exact React patterns documented here.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Exact color palette: primary #2E7D32, secondary #FF6F00, dark backgrounds #121212/#1E1E1E, light backgrounds #F8F9FA/#FFFFFF
- **D-02:** Vuetify native defaults for component styling (no MUI shadow replication)
- **D-03:** Inter/Roboto typography, font scale 0.85-1.3 range, step 0.1, via CSS custom property or Vuetify theme override
- **D-04:** localStorage key `granjatech-accessibility-preferences` with `{mode, fontScale}` JSON format
- **D-05:** `<script setup lang="ts">` Composition API with strict TypeScript
- **D-06:** PascalCase file naming
- **D-07:** Project in `granjatech-frontend/` at repo root
- **D-08:** Folder layout: `src/components/`, `src/views/`, `src/stores/`, `src/services/`, `src/router/`, `src/plugins/`, `src/App.vue`, `src/main.ts`
- **D-09:** `VITE_API_URL` env var, fallback `http://localhost:5099/api`
- **D-10:** Axios with request interceptor (Bearer token) and response interceptor (401 redirect)
- **D-11:** Auth token in localStorage key `token` (same as React)
- **D-12:** Auth Pinia store: `useAuthStore()` with login/logout, jwt-decode, hydrates from localStorage
- **D-13:** Accessibility Pinia store: `useAccessibilityStore()` with dark mode toggle, font scale controls
- **D-14:** `v-navigation-drawer` -- permanent on md+, temporary on mobile, 280px width, 13 menu items with role filtering
- **D-15:** Dark mode toggle and font scale controls in app bar (right side)
- **D-16:** PageContainer.vue with breadcrumbs, title/subtitle, responsive wrapping
- **D-17:** Vue Router navigation guards: redirect unauthenticated to /login
- **D-18:** Carried forward: Portuguese naming, JWT claims `nameid`/`email`/`role`, same PostgreSQL DB

### Claude's Discretion
- Vite config details (proxy, build options)
- Exact Vuetify plugin configuration structure
- Router meta fields and guard implementation details
- Pinia store internal structure (getters, actions naming)
- LoadingSpinner.vue design (Vuetify native vs custom)
- Breadcrumb implementation details in PageContainer

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FRON-01 | Projeto Vue 3 + Vuetify 3 + TypeScript + Vite configurado e buildando | Standard Stack section -- npm create vuetify scaffolding, all packages verified |
| FRON-02 | Vuetify tema migrado do MUI (cores, dark mode, fontScale) | Architecture Patterns -- Vuetify theme config with exact color values from React theme.js |
| FRON-03 | Auth store (Pinia) com login/logout/register e persistencia em localStorage | Code Examples -- auth.ts store pattern with jwt-decode |
| FRON-04 | Accessibility store (Pinia) com dark mode toggle e font scale | Code Examples -- accessibility.ts store with Vuetify useTheme() integration |
| FRON-05 | API service (Axios) com interceptors de token e redirect 401 | Code Examples -- api.ts service mirroring React apiService.js |
| FRON-06 | Vue Router com navigation guards (rotas protegidas) | Architecture Patterns -- router/index.ts with beforeEach guard |
| FRON-07 | ResponsiveNavigation.vue (drawer + app bar) | Code Examples -- Vuetify v-navigation-drawer + v-app-bar with 13 nav items |
| FRON-08 | PageContainer.vue e LoadingSpinner.vue | Code Examples -- wrapper components using Vuetify v-breadcrumbs and v-progress-circular |
| VIEW-01 | LoginView.vue funcional contra backend Rust | Code Examples -- login form with gradient background, Vuetify form validation |
</phase_requirements>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| vue | 3.5.32 | UI framework | Latest stable, Composition API + `<script setup>` [VERIFIED: npm registry] |
| vuetify | 4.0.5 | Material Design component library | Latest stable, full MD3 support [VERIFIED: npm registry] |
| vue-router | 5.0.4 | Client-side routing with navigation guards | Official Vue router [VERIFIED: npm registry] |
| pinia | 3.0.4 | State management (auth, accessibility stores) | Official Vue state manager, replaces Vuex [VERIFIED: npm registry] |
| vite | 8.0.7 | Build tool and dev server | Default for Vue projects [VERIFIED: npm registry] |
| typescript | 6.0.2 | Type safety | Latest stable [VERIFIED: npm registry] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| axios | 1.14.0 | HTTP client for API calls | All API communication with Rust backend [VERIFIED: npm registry] |
| jwt-decode | 4.0.0 | Decode JWT tokens without verification | Extract user claims from auth token [VERIFIED: npm registry] |
| @mdi/font | 7.4.47 | Material Design Icons | Icon font for Vuetify components [VERIFIED: npm registry] |
| @fontsource/inter | 5.2.8 | Inter font family | Typography (D-03 decision) [VERIFIED: npm registry] |
| @fontsource/roboto | 5.2.10 | Roboto font family | Fallback typography [VERIFIED: npm registry] |
| @vitejs/plugin-vue | 6.0.5 | Vite Vue SFC support | Required for .vue files [VERIFIED: npm registry] |
| vite-plugin-vuetify | 2.1.3 | Vuetify tree-shaking and SASS vars | Auto-import Vuetify components [VERIFIED: npm registry] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Pinia | Vuex 4 | Pinia is the official recommendation since Vue 3; Vuex is legacy |
| @mdi/font | @mdi/js | Tree-shakeable but requires per-icon imports; font is simpler for full app |
| vite-plugin-vuetify | manual imports | Plugin provides automatic tree-shaking and SASS variable access |

**Installation:**
```bash
cd granjatech-frontend
npm install vue vuetify vue-router pinia axios jwt-decode @mdi/font @fontsource/inter @fontsource/roboto
npm install -D typescript vite @vitejs/plugin-vue vite-plugin-vuetify
```

Note: If using `npm create vuetify@latest`, most core packages are pre-configured. Only `axios`, `jwt-decode`, `@fontsource/inter`, and `@fontsource/roboto` need manual install.

## Architecture Patterns

### Recommended Project Structure
```
granjatech-frontend/
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
├── .env                    # VITE_API_URL=http://localhost:5099/api
├── src/
│   ├── main.ts             # App bootstrap
│   ├── App.vue             # Root component with v-app shell
│   ├── plugins/
│   │   └── vuetify.ts      # Vuetify instance + theme config
│   ├── router/
│   │   └── index.ts        # Routes + navigation guards
│   ├── stores/
│   │   ├── auth.ts         # useAuthStore() - login/logout/token
│   │   └── accessibility.ts # useAccessibilityStore() - dark mode/font scale
│   ├── services/
│   │   └── api.ts          # Axios instance + interceptors
│   ├── components/
│   │   ├── ResponsiveNavigation.vue  # Drawer + AppBar
│   │   ├── PageContainer.vue         # Page wrapper with breadcrumbs
│   │   └── LoadingSpinner.vue        # Loading indicator
│   └── views/
│       └── LoginView.vue   # Login page (only view in Phase 4)
└── public/
    └── favicon.ico
```

### Pattern 1: Vuetify Theme Configuration (D-01, D-02)

**What:** Define light and dark themes with the exact React MUI colors.
**When to use:** In `src/plugins/vuetify.ts`, applied at app initialization.

```typescript
// src/plugins/vuetify.ts
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import '@fontsource/inter/400.css'
import '@fontsource/inter/500.css'
import '@fontsource/inter/600.css'
import '@fontsource/inter/700.css'
import { createVuetify } from 'vuetify'

export default createVuetify({
  theme: {
    defaultTheme: 'light',
    themes: {
      light: {
        dark: false,
        colors: {
          primary: '#2E7D32',
          'primary-lighten-1': '#66BB6A',
          'primary-darken-1': '#1B5E20',
          secondary: '#FF6F00',
          'secondary-lighten-1': '#FFB74D',
          'secondary-darken-1': '#E65100',
          background: '#F8F9FA',
          surface: '#FFFFFF',
          success: '#4CAF50',
          error: '#F44336',
          warning: '#FF9800',
          info: '#2196F3',
        },
      },
      dark: {
        dark: true,
        colors: {
          primary: '#2E7D32',
          'primary-lighten-1': '#66BB6A',
          'primary-darken-1': '#1B5E20',
          secondary: '#FF6F00',
          'secondary-lighten-1': '#FFB74D',
          'secondary-darken-1': '#E65100',
          background: '#121212',
          surface: '#1E1E1E',
          success: '#4CAF50',
          error: '#F44336',
          warning: '#FF9800',
          info: '#2196F3',
        },
      },
    },
  },
  defaults: {
    global: {
      font: {
        family: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
      },
    },
  },
})
```

### Pattern 2: Pinia Auth Store with JWT Decode (D-11, D-12)

**What:** Reactive auth store that hydrates from localStorage on init.
**When to use:** In `src/stores/auth.ts`.

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { jwtDecode } from 'jwt-decode'
import api from '@/services/api'

interface JwtPayload {
  nameid: string
  email: string
  role: string
  exp: number
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const user = ref<JwtPayload | null>(null)

  // Hydrate on init
  if (token.value) {
    try {
      user.value = jwtDecode<JwtPayload>(token.value)
    } catch {
      token.value = null
      localStorage.removeItem('token')
    }
  }

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  async function login(email: string, senha: string) {
    const response = await api.post('/auth/login', { email, senha })
    const newToken = response.data.token
    token.value = newToken
    user.value = jwtDecode<JwtPayload>(newToken)
    localStorage.setItem('token', newToken)
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
  }

  return { token, user, isAuthenticated, login, logout }
})
```

### Pattern 3: Vue Router Navigation Guard (D-17)

**What:** beforeEach guard redirecting unauthenticated users to login.
**When to use:** In `src/router/index.ts`.

```typescript
import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', name: 'login', component: () => import('@/views/LoginView.vue'), meta: { requiresAuth: false } },
    { path: '/', name: 'dashboard', component: () => import('@/views/PlaceholderView.vue'), meta: { requiresAuth: true } },
    // Phase 5 routes will go here
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ],
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  if (to.meta.requiresAuth !== false && !auth.isAuthenticated) {
    return { name: 'login' }
  }
  if (to.name === 'login' && auth.isAuthenticated) {
    return { name: 'dashboard' }
  }
})

export default router
```

### Pattern 4: Font Scale via CSS Custom Property (D-03)

**What:** Apply font scale globally using a CSS custom property on `<html>`, read by Vuetify's font-size base.
**When to use:** In the accessibility store, watch fontScale and update `document.documentElement.style.fontSize`.

```typescript
// In accessibility store
watch(fontScale, (scale) => {
  document.documentElement.style.fontSize = `${scale * 100}%`
})
```

This approach works because Vuetify uses `rem` units. Changing the root font-size scales all text proportionally.

### Pattern 5: Vuetify Dark Mode Toggle (D-04, D-13)

**What:** Toggle between light and dark Vuetify themes reactively.
**When to use:** Accessibility store integrates with Vuetify's `useTheme()`.

The Vuetify `useTheme()` composable provides `theme.global.name` which can be set to `'light'` or `'dark'`. The accessibility store persists the preference and syncs it.

### Anti-Patterns to Avoid

- **Direct DOM manipulation for theming:** Use Vuetify's theme system, not manual class toggling
- **Storing decoded user in localStorage:** Only store the raw JWT token; decode on hydration
- **Using Options API:** Decision D-05 mandates `<script setup lang="ts">` Composition API
- **Importing all Vuetify components manually:** Use `vite-plugin-vuetify` for auto-import
- **Using `window.location.href` for navigation in guards:** Use Vue Router's `router.push()` or return value from `beforeEach`

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JWT decoding | Manual base64 parsing | `jwt-decode` 4.0.0 | Handles edge cases, type-safe with generics |
| Material Design icons | Custom SVG system | `@mdi/font` | 7000+ icons, Vuetify native integration |
| Component tree-shaking | Manual imports | `vite-plugin-vuetify` | Automatic, zero config |
| Theme switching | CSS class toggling | Vuetify `useTheme()` | Reactive, supports custom color tokens |
| Route protection | Manual auth checks per view | Vue Router `beforeEach` | Centralized, declarative via `meta` |
| Responsive breakpoints | Custom media queries | Vuetify `useDisplay()` | Reactive, matches v-navigation-drawer breakpoints |

**Key insight:** Vuetify 3/4 provides built-in solutions for almost every UI concern in this phase. The main custom code is the Pinia stores and API service.

## Common Pitfalls

### Pitfall 1: Pinia Store Used Before App Initialization
**What goes wrong:** Calling `useAuthStore()` outside of a component or before `app.use(pinia)` throws an error.
**Why it happens:** Pinia requires the app context to be active.
**How to avoid:** In `router/index.ts`, access the store inside the `beforeEach` callback (which runs after app mount), not at module level.
**Warning signs:** "getActivePinia was called with no active Pinia" error.

### Pitfall 2: Vuetify Theme Not Applying to Dark Mode
**What goes wrong:** Changing `theme.global.name.value` does not update background colors.
**Why it happens:** The root `<v-app>` component must wrap the entire app for Vuetify's theme CSS variables to apply.
**How to avoid:** Always wrap in `<v-app>` in App.vue. Never render content outside `<v-app>`.
**Warning signs:** Background stays white/light even after toggling dark mode.

### Pitfall 3: CORS Issues with Rust Backend
**What goes wrong:** Axios calls to `localhost:5099` fail with CORS errors during development.
**Why it happens:** Vite dev server runs on a different port (default 5173).
**How to avoid:** Either configure Vite's dev server proxy in `vite.config.ts`, or ensure the Rust backend's CORS config includes the Vite dev server origin. The Rust backend already has CORS middleware (Phase 1) -- verify it allows `http://localhost:5173`.
**Warning signs:** Network errors in browser console, "Access-Control-Allow-Origin" missing.

### Pitfall 4: JWT Token Expiration Not Handled
**What goes wrong:** User stays "logged in" with an expired token, gets 401 on every request.
**Why it happens:** The token is in localStorage but the JWT has expired.
**How to avoid:** Check `exp` claim during hydration in the auth store. The 401 interceptor (D-10) handles runtime expiration by redirecting to login.
**Warning signs:** User sees login screen briefly, then gets redirected again.

### Pitfall 5: Font Scale Applied Incorrectly
**What goes wrong:** Font scale changes affect some text but not others, or affects non-text elements.
**Why it happens:** Mixing `rem` and `px` units, or applying scale to root without understanding cascade.
**How to avoid:** Use the `document.documentElement.style.fontSize` approach consistently. Vuetify uses `rem` internally, so this scales proportionally.
**Warning signs:** Buttons and icons scale weirdly, or text doesn't change at all.

### Pitfall 6: Navigation Items Array -- Role Values Must Match JWT
**What goes wrong:** No navigation items appear after login.
**Why it happens:** The JWT `role` claim value doesn't match the strings in the navigation items array.
**How to avoid:** Use exact role strings from the Rust backend JWT: `'Administrador'`, `'Produtor'`, `'Financeiro'`.
**Warning signs:** Empty navigation drawer for logged-in users.

## Code Examples

### Navigation Items Array (from React ResponsiveNavigation.js)

The exact 13 items with their roles, to be replicated in Vue:

```typescript
// src/components/navigationItems.ts (or inline in ResponsiveNavigation.vue)
export const navigationItems = [
  { path: '/', label: 'Dashboard', icon: 'mdi-view-dashboard', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/granjas', label: 'Granjas', icon: 'mdi-barn', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/lotes', label: 'Lotes', icon: 'mdi-duck', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/estoque', label: 'Estoque', icon: 'mdi-package-variant', roles: ['Administrador', 'Produtor'] },
  { path: '/avicultura', label: 'Avicultura Pro', icon: 'mdi-chart-line', roles: ['Administrador', 'Produtor'] },
  { path: '/consumo', label: 'Consumo', icon: 'mdi-food-drumstick', roles: ['Administrador', 'Produtor'] },
  { path: '/pesagem', label: 'Pesagens', icon: 'mdi-scale', roles: ['Administrador', 'Produtor'] },
  { path: '/sanitario', label: 'Sanitario', icon: 'mdi-medical-bag', roles: ['Administrador', 'Produtor'] },
  { path: '/sensores', label: 'Sensores', icon: 'mdi-access-point', roles: ['Administrador', 'Produtor'] },
  { path: '/financeiro', label: 'Financeiro', icon: 'mdi-currency-usd', roles: ['Administrador', 'Financeiro'] },
  { path: '/relatorios', label: 'Relatorios', icon: 'mdi-file-chart', roles: ['Administrador', 'Financeiro', 'Produtor'] },
  { path: '/usuarios', label: 'Usuarios', icon: 'mdi-account-group', roles: ['Administrador'] },
  { path: '/auditoria', label: 'Auditoria', icon: 'mdi-shield-check', roles: ['Administrador'] },
]
```

### API Service (D-09, D-10)

```typescript
// src/services/api.ts
import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:5099/api',
  timeout: 30000,
  headers: { 'Content-Type': 'application/json' },
})

// Request: attach Bearer token
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Response: redirect to /login on 401
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export default api
```

Note: The 401 interceptor uses `window.location.href` (matching React behavior) rather than `router.push()` because the Axios interceptor runs outside Vue component context. This is intentional parity with the React implementation.

### Accessibility Store (D-04, D-13)

```typescript
// src/stores/accessibility.ts
import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { useTheme } from 'vuetify'

const STORAGE_KEY = 'granjatech-accessibility-preferences'
const DEFAULT_MODE = 'light'
const DEFAULT_FONT_SCALE = 1
const FONT_SCALE_STEP = 0.1
const MIN_FONT_SCALE = 0.85
const MAX_FONT_SCALE = 1.3

function clamp(value: number): number {
  return Math.min(MAX_FONT_SCALE, Math.max(MIN_FONT_SCALE, Number.isNaN(value) ? DEFAULT_FONT_SCALE : value))
}

function readStored(): { mode: string; fontScale: number } | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return null
    const parsed = JSON.parse(raw)
    return {
      mode: parsed.mode === 'dark' ? 'dark' : DEFAULT_MODE,
      fontScale: clamp(parseFloat(parsed.fontScale)),
    }
  } catch {
    return null
  }
}

export const useAccessibilityStore = defineStore('accessibility', () => {
  const stored = readStored()
  const mode = ref(stored?.mode || DEFAULT_MODE)
  const fontScale = ref(stored?.fontScale || DEFAULT_FONT_SCALE)

  const canIncreaseFont = computed(() => fontScale.value < MAX_FONT_SCALE - 1e-3)
  const canDecreaseFont = computed(() => fontScale.value > MIN_FONT_SCALE + 1e-3)

  function toggleColorMode() { mode.value = mode.value === 'light' ? 'dark' : 'light' }
  function increaseFontScale() { fontScale.value = clamp(fontScale.value + FONT_SCALE_STEP) }
  function decreaseFontScale() { fontScale.value = clamp(fontScale.value - FONT_SCALE_STEP) }
  function resetSettings() {
    mode.value = DEFAULT_MODE
    fontScale.value = DEFAULT_FONT_SCALE
    localStorage.removeItem(STORAGE_KEY)
  }

  // Persist to localStorage
  watch([mode, fontScale], ([m, fs]) => {
    if (m === DEFAULT_MODE && fs === DEFAULT_FONT_SCALE) {
      localStorage.removeItem(STORAGE_KEY)
    } else {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({ mode: m, fontScale: fs }))
    }
  })

  // Apply font scale to root element
  watch(fontScale, (scale) => {
    document.documentElement.style.fontSize = `${scale * 100}%`
  }, { immediate: true })

  return {
    mode, fontScale,
    canIncreaseFont, canDecreaseFont,
    toggleColorMode, increaseFontScale, decreaseFontScale, resetSettings,
  }
})
```

### Vuetify Theme Sync in App.vue

The accessibility store's `mode` must be synced to Vuetify's theme system:

```vue
<!-- src/App.vue -->
<script setup lang="ts">
import { watch } from 'vue'
import { useTheme } from 'vuetify'
import { useAccessibilityStore } from '@/stores/accessibility'
import ResponsiveNavigation from '@/components/ResponsiveNavigation.vue'
import { useAuthStore } from '@/stores/auth'

const theme = useTheme()
const accessibility = useAccessibilityStore()
const auth = useAuthStore()

// Sync accessibility store mode -> Vuetify theme
watch(() => accessibility.mode, (newMode) => {
  theme.global.name.value = newMode
}, { immediate: true })
</script>

<template>
  <v-app>
    <ResponsiveNavigation v-if="auth.isAuthenticated" />
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>
```

### Login Request/Response Format

The Rust backend expects and returns:

```
POST /api/auth/login
Request:  { "email": "string", "senha": "string" }
Response: { "token": "jwt-string" }
```

JWT payload claims: `nameid` (user ID string), `email`, `role` (one of: Administrador, Produtor, Financeiro), `exp` (expiration timestamp).

### Route Labels for Breadcrumbs (from React PageContainer.js)

```typescript
const routeLabels: Record<string, string> = {
  '/': 'Dashboard',
  '/granjas': 'Granjas',
  '/lotes': 'Lotes',
  '/estoque': 'Estoque',
  '/sensores': 'Sensores',
  '/financeiro': 'Financeiro',
  '/relatorios': 'Relatorios',
  '/usuarios': 'Usuarios',
  '/auditoria': 'Auditoria',
  '/perfil': 'Perfil',
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Vuex 4 | Pinia 3.x | 2022 (official recommendation) | Simpler API, TypeScript-first, Composition API native |
| Vue CLI | Vite | 2022-2023 (Vue 3 default) | Faster dev server, ESM-native, better DX |
| Options API | Composition API + `<script setup>` | Vue 3.2+ (2021) | Less boilerplate, better TypeScript, tree-shakeable |
| Vuetify 2 (Vue 2) | Vuetify 3/4 (Vue 3) | 2023+ | Complete rewrite, Composition API, MD3 support |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `npm create vuetify@latest` scaffolds Vue 3 + Vuetify + Vite + TS | Standard Stack | LOW -- may need manual setup instead, but packages are correct |
| A2 | Vuetify 4.x `useTheme()` API sets `theme.global.name.value` for theme switching | Pattern 5 | MEDIUM -- API may have changed in Vuetify 4; verify at implementation time |
| A3 | `document.documentElement.style.fontSize` approach scales all Vuetify rem-based text | Pattern 4 | LOW -- standard CSS behavior, but verify Vuetify doesn't override |
| A4 | MDI icon names (mdi-view-dashboard, mdi-barn, etc.) are correct for the nav items | Code Examples | LOW -- can verify against @mdi/font docs |

## Open Questions

1. **Vuetify 4.0.5 vs 3.x API differences**
   - What we know: npm shows vuetify 4.0.5 as latest; CONTEXT.md references "Vuetify 3"
   - What's unclear: Whether Vuetify 4 has breaking API changes from 3
   - Recommendation: Use whatever `npm create vuetify@latest` provides. The core patterns (theme, components, composables) are the same. If Vuetify 4 is installed, use its API.

2. **Vite proxy vs direct CORS**
   - What we know: Rust backend has CORS middleware from Phase 1
   - What's unclear: Whether CORS is configured to allow Vite's dev port (5173)
   - Recommendation: Check Rust CORS config first; add Vite proxy as fallback in `vite.config.ts`

3. **Placeholder views for Phase 5 routes**
   - What we know: Phase 4 only builds LoginView.vue; Phase 5 adds 14 more views
   - What's unclear: Whether Phase 4 should register all 15 routes (with placeholders) or just login + dashboard
   - Recommendation: Register all routes now with a simple placeholder component, so navigation items are clickable. This validates the full navigation structure early.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Build/dev server | Yes | v24.14.0 | -- |
| npm | Package management | Yes | 11.9.0 | -- |
| Rust backend | API endpoints | Yes | -- | -- (must be running for login test) |
| PostgreSQL | Backend data | Yes | -- | Via existing docker-compose |

**Missing dependencies with no fallback:** None.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Vitest (bundled with Vite ecosystem) |
| Config file | `granjatech-frontend/vitest.config.ts` (Wave 0) |
| Quick run command | `cd granjatech-frontend && npx vitest run --reporter=verbose` |
| Full suite command | `cd granjatech-frontend && npx vitest run` |

### Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FRON-01 | Vue + Vuetify project builds | smoke | `cd granjatech-frontend && npm run build` | Wave 0 |
| FRON-02 | Theme colors match spec | unit | `vitest run src/plugins/__tests__/vuetify.test.ts` | Wave 0 |
| FRON-03 | Auth store login/logout/hydrate | unit | `vitest run src/stores/__tests__/auth.test.ts` | Wave 0 |
| FRON-04 | Accessibility store toggle/scale/persist | unit | `vitest run src/stores/__tests__/accessibility.test.ts` | Wave 0 |
| FRON-05 | API service interceptors | unit | `vitest run src/services/__tests__/api.test.ts` | Wave 0 |
| FRON-06 | Router guard redirects unauthenticated | unit | `vitest run src/router/__tests__/index.test.ts` | Wave 0 |
| FRON-07 | Navigation renders filtered items | manual-only | Manual: login with different roles, verify menu items | -- |
| FRON-08 | PageContainer/LoadingSpinner render | unit | `vitest run src/components/__tests__/` | Wave 0 |
| VIEW-01 | Login works against Rust backend | manual-only | Manual: enter credentials, verify redirect to dashboard | -- |

### Sampling Rate
- **Per task commit:** `cd granjatech-frontend && npm run build`
- **Per wave merge:** `cd granjatech-frontend && npx vitest run`
- **Phase gate:** Full suite green + manual login test before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `granjatech-frontend/vitest.config.ts` -- Vitest configuration
- [ ] `granjatech-frontend/src/stores/__tests__/auth.test.ts` -- covers FRON-03
- [ ] `granjatech-frontend/src/stores/__tests__/accessibility.test.ts` -- covers FRON-04
- [ ] Framework install: `npm install -D vitest @vue/test-utils happy-dom`

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | Yes | JWT Bearer token via Rust backend (no frontend auth logic beyond storing token) |
| V3 Session Management | Yes | JWT in localStorage with 8-hour expiration, 401 interceptor clears token |
| V4 Access Control | Yes | Role-based navigation filtering (UI-level only; backend enforces actual access) |
| V5 Input Validation | Yes | Vuetify form validation rules on login fields |
| V6 Cryptography | No | All crypto handled by Rust backend |

### Known Threat Patterns for Vue SPA

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| XSS via template injection | Tampering | Vue's built-in template escaping; never use `v-html` with user input |
| JWT in localStorage | Information Disclosure | Acceptable tradeoff per project decision (D-11); 401 interceptor handles expiry |
| CORS misconfiguration | Spoofing | Rust backend CORS middleware restricts origins |
| Open redirect after login | Spoofing | Router guard only redirects to known routes; no dynamic redirect URL |

## Project Constraints (from CLAUDE.md)

- **Stack:** Rust (Actix-web 4, SQLx) + Vue 3 (Vuetify 3, Pinia, Vue Router 4, TypeScript, Vite)
- **Database:** PostgreSQL 16, no destructive migrations
- **API Contract:** Same endpoints, same routes, same JSON payloads
- **Naming:** Portuguese naming in code (from Phase 1 D-03)
- **BCrypt:** Cross-compatible hashes (.NET/Rust) -- not relevant for frontend but affects login flow
- **Deploy:** Docker Compose local only
- **Frontend project location:** `granjatech-frontend/` at repo root

## Sources

### Primary (HIGH confidence)
- npm registry -- verified all package versions (vue 3.5.32, vuetify 4.0.5, vue-router 5.0.4, pinia 3.0.4, vite 8.0.7, axios 1.14.0, jwt-decode 4.0.0, typescript 6.0.2)
- React source code -- `frontend/src/context/AuthContext.js`, `AccessibilityContext.js`, `services/apiService.js`, `theme.js`, `components/ResponsiveNavigation.js`, `components/PageContainer.js`, `components/LoadingSpinner.js`, `pages/LoginPage.js`, `App.js`
- Rust backend source -- `granjatech-api/src/dto/auth.rs` (LoginDto, LoginResponseDto)

### Secondary (MEDIUM confidence)
- Vuetify theme API patterns [ASSUMED from training data -- verify at implementation time]
- `npm create vuetify@latest` scaffolding output [ASSUMED]

### Tertiary (LOW confidence)
- Exact MDI icon names for navigation items [ASSUMED -- verify against @mdi/font]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all versions verified against npm registry
- Architecture: HIGH -- patterns extracted directly from React reference implementation
- Pitfalls: MEDIUM -- based on Vue/Vuetify ecosystem knowledge, common patterns
- Code examples: MEDIUM -- patterns are standard but Vuetify 4.x API needs runtime verification

**Research date:** 2026-04-07
**Valid until:** 2026-05-07 (30 days -- stable ecosystem)
