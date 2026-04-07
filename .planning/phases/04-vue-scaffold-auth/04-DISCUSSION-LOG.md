# Phase 4: Vue Scaffold + Auth - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-07
**Phase:** 04-vue-scaffold-auth
**Areas discussed:** Vuetify theme fidelity, Vue project structure, API service & auth flow, Navigation & layout

---

## Vuetify Theme Fidelity

| Option | Description | Selected |
|--------|-------------|----------|
| Same palette, Vuetify defaults | Keep exact color palette but use Vuetify's native component styling (borders, shadows, spacing) | ✓ |
| Pixel-perfect replication | Replicate MUI's exact border-radius, shadow levels, scrollbar styles, component overrides | |
| Vuetify Material Design 3 | Use Vuetify's MD3 defaults with the green/orange color scheme | |

**User's choice:** Same palette, Vuetify defaults
**Notes:** Faster to implement, looks native to Vuetify. Colors are exact match, component styling uses Vuetify defaults.

### Dark mode / font scale localStorage key

| Option | Description | Selected |
|--------|-------------|----------|
| Same key `granjatech-accessibility-preferences` | Reuse same key and JSON format for cross-frontend compatibility | ✓ |
| New key for Vue | Separate key `granjatech-vue-accessibility` | |

**User's choice:** Same key
**Notes:** Preferences carry over between React and Vue frontends.

---

## Vue Project Structure

### SFC Style & TypeScript

| Option | Description | Selected |
|--------|-------------|----------|
| `<script setup>` + strict TS | Composition API with strict TypeScript | ✓ |
| `<script setup>` + relaxed TS | Same but with strict: false | |
| Options API + TypeScript | Traditional defineComponent() | |

**User's choice:** `<script setup>` + strict TS

### Folder Organization

| Option | Description | Selected |
|--------|-------------|----------|
| Mirror React structure | src/views/, src/components/, src/stores/, src/services/, src/router/ | ✓ |
| Feature-based | src/features/auth/, src/features/granjas/, etc. | |
| Flat minimal | src/pages/, src/lib/ | |

**User's choice:** Mirror React structure

### File Naming

| Option | Description | Selected |
|--------|-------------|----------|
| PascalCase | LoginView.vue, PageContainer.vue | ✓ |
| kebab-case | login-view.vue, page-container.vue | |

**User's choice:** PascalCase

### Project Location

| Option | Description | Selected |
|--------|-------------|----------|
| granjatech-frontend/ | New top-level folder alongside granjatech-api/ | ✓ |
| vue-frontend/ | Generic name at repo root | |
| Replace frontend/ | Remove React, put Vue in frontend/ | |

**User's choice:** granjatech-frontend/

---

## API Service & Auth Flow

### API Base URL

| Option | Description | Selected |
|--------|-------------|----------|
| VITE_API_URL env var | Vite built-in env vars, fallback to localhost:5099 | ✓ |
| Hardcoded with dev/prod switch | Detect environment and switch URLs | |

**User's choice:** VITE_API_URL env var

### Token localStorage Key

| Option | Description | Selected |
|--------|-------------|----------|
| Same key 'token' | Reuse for React/Vue interop | ✓ |
| New key 'granjatech-token' | Clean break, requires re-login | |

**User's choice:** Same key 'token'

---

## Navigation & Layout

### Drawer Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Match React behavior | Permanent on desktop (md+), temporary on mobile, 280px width | ✓ |
| Rail + expand pattern | Mini drawer on desktop, full on mobile | |
| Always temporary | Hamburger menu on all screen sizes | |

**User's choice:** Match React behavior

### Accessibility Controls Placement

| Option | Description | Selected |
|--------|-------------|----------|
| App bar (same as React) | Dark mode toggle + font scale in top app bar, right side | ✓ |
| Settings drawer | Separate settings panel for accessibility controls | |
| Navigation drawer footer | Controls at bottom of navigation drawer | |

**User's choice:** App bar (same as React)

---

## Claude's Discretion

- Vite config details (proxy, build options)
- Exact Vuetify plugin configuration structure
- Router meta fields and guard implementation details
- Pinia store internal structure
- LoadingSpinner.vue design
- Breadcrumb implementation details

## Deferred Ideas

None — discussion stayed within phase scope.
