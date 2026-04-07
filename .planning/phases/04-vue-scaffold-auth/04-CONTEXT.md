# Phase 4: Vue Scaffold + Auth - Context

**Gathered:** 2026-04-07
**Status:** Ready for planning

<domain>
## Phase Boundary

Vue 3 + Vuetify 3 + TypeScript + Vite project setup with Pinia stores (auth and accessibility), Vue Router with navigation guards, layout components (ResponsiveNavigation, PageContainer, LoadingSpinner), and a functional LoginView that authenticates against the Rust backend. Users can log in, navigate protected routes, toggle dark mode, and adjust font scale.

</domain>

<decisions>
## Implementation Decisions

### Vuetify Theme
- **D-01:** Exact color palette from React MUI theme: primary #2E7D32, secondary #FF6F00, dark backgrounds #121212/#1E1E1E, light backgrounds #F8F9FA/#FFFFFF. All semantic colors (success, error, warning, info) match.
- **D-02:** Component styling uses Vuetify native defaults (elevation, border-radius, transitions, scrollbar). No pixel-perfect replication of MUI's custom shadow levels or component overrides.
- **D-03:** Typography uses Inter/Roboto family with Vuetify's default sizing. Font scale (0.85-1.3 range, step 0.1) applied via CSS custom property or Vuetify theme override.
- **D-04:** Same localStorage key `granjatech-accessibility-preferences` with same JSON format `{mode, fontScale}` for dark mode and font scale persistence.

### Vue Project Structure
- **D-05:** `<script setup lang="ts">` Composition API with strict TypeScript (`strict: true` in tsconfig).
- **D-06:** PascalCase file naming: LoginView.vue, PageContainer.vue, ResponsiveNavigation.vue.
- **D-07:** Project lives in `granjatech-frontend/` at repo root (alongside `granjatech-api/` and `frontend/`).
- **D-08:** Folder layout mirrors React structure:
  - `src/components/` — PageContainer.vue, LoadingSpinner.vue, ResponsiveNavigation.vue
  - `src/views/` — LoginView.vue (Phase 4), remaining 14 views in Phase 5
  - `src/stores/` — auth.ts, accessibility.ts (Pinia)
  - `src/services/` — api.ts (Axios)
  - `src/router/` — index.ts (Vue Router)
  - `src/plugins/` — vuetify.ts (Vuetify config + theme)
  - `src/App.vue`, `src/main.ts`

### API Service & Auth Flow
- **D-09:** API base URL configured via `VITE_API_URL` env var (Vite built-in). Fallback to `http://localhost:5099/api` (Rust backend port). Accessed via `import.meta.env.VITE_API_URL`.
- **D-10:** Axios instance with request interceptor (attach Bearer token from localStorage) and response interceptor (redirect to /login on 401).
- **D-11:** Auth token stored in localStorage under key `token` — same as React for interop. Users logged in via React stay logged in on Vue.
- **D-12:** Auth Pinia store: `useAuthStore()` with login/logout actions, decoded JWT user data (via jwt-decode), reactive token/user state. Hydrates from localStorage on app init.
- **D-13:** Accessibility Pinia store: `useAccessibilityStore()` with dark mode toggle, font scale increase/decrease/reset. Persists to same localStorage key as React (D-04).

### Navigation & Layout
- **D-14:** Vuetify `v-navigation-drawer` — permanent on desktop (md+ breakpoint), temporary/overlay on mobile. 280px width. Same 13 menu items with role-based filtering matching React's `navigationItems` array.
- **D-15:** Dark mode toggle and font scale controls in the app bar (right side), same placement as React.
- **D-16:** PageContainer.vue with breadcrumbs, title/subtitle header, responsive wrapping — same behavior as React PageContainer.
- **D-17:** Vue Router navigation guards: unauthenticated users redirected to /login, authenticated users see full navigation.

### Carried Forward from Phases 1-3
- **D-18:** All prior phase decisions carry forward:
  - Portuguese naming in code (Phase 1 D-03)
  - JWT claims: `nameid`, `email`, `role` (Phase 1 D-05)
  - Normalized API responses from Rust backend (Phase 1 D-07)
  - Same PostgreSQL database, no migrations

### Claude's Discretion
- Vite config details (proxy, build options)
- Exact Vuetify plugin configuration structure
- Router meta fields and guard implementation details
- Pinia store internal structure (getters, actions naming)
- LoadingSpinner.vue design (Vuetify native vs custom)
- Breadcrumb implementation details in PageContainer

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan
- `plano-migracao-granjatech.md` — Complete migration plan with frontend component mapping (React -> Vue equivalences)

### React frontend (reference implementation to replicate)
- `frontend/src/context/AuthContext.js` — Auth logic: JWT decode, localStorage token, login/logout flow
- `frontend/src/context/AccessibilityContext.js` — Dark mode + font scale: localStorage key, range constants, toggle logic
- `frontend/src/services/apiService.js` — Axios setup: base URL, interceptors (token, 401 redirect), API methods
- `frontend/src/theme.js` — MUI theme: color palette, typography factory, component overrides (reference for Vuetify theme)
- `frontend/src/components/ResponsiveNavigation.js` — Navigation drawer: 13 menu items with roles, drawer width, app bar controls
- `frontend/src/components/PageContainer.js` — Page wrapper: breadcrumbs, title/subtitle, responsive layout
- `frontend/src/components/LoadingSpinner.js` — Loading indicator component
- `frontend/src/components/ProtectedRoute.js` — Route guard pattern (reference for Vue Router guards)
- `frontend/src/pages/LoginPage.js` — Login form: gradient background, card layout, error handling, loading state
- `frontend/src/App.js` — App shell: routing, theme provider, accessibility provider

### Existing Rust backend (API to connect to)
- `granjatech-api/src/handlers/auth.rs` — Auth endpoints (POST /api/auth/login, POST /api/auth/register)
- `granjatech-api/src/dto/auth.rs` — Login request/response DTOs (LoginDto, LoginResponseDto with token field)
- `granjatech-api/src/middleware/jwt.rs` — JWT Claims structure (nameid, email, role)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- React frontend serves as complete reference for all UI behavior, navigation structure, and auth flow
- Rust backend already running with all 60+ endpoints — no backend changes needed
- Migration plan maps every React component to its Vue equivalent

### Established Patterns
- **Auth flow:** Login POST -> receive JWT -> decode with jwt-decode -> store in localStorage -> attach to subsequent requests
- **Role filtering:** `navigationItems` array with `roles` field per item, filtered against decoded user's role claim
- **Accessibility:** localStorage persistence with same key/format, mode toggle, font scale with min/max/step
- **Page layout:** AppBar + Drawer shell, pages rendered in main content area offset by drawer width on desktop

### Integration Points
- Rust backend at `http://localhost:5099/api` — all endpoints ready
- POST `/api/auth/login` with `{email, senha}` returns `{token}` (JWT)
- JWT claims: `nameid` (user ID as string), `email`, `role` (one of: Administrador, Produtor, Financeiro)
- localStorage keys: `token` (JWT), `granjatech-accessibility-preferences` (dark mode + font scale)

</code_context>

<specifics>
## Specific Ideas

No specific requirements — replicate React behavior using Vuetify 3 native components and patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 04-vue-scaffold-auth*
*Context gathered: 2026-04-07*
