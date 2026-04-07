# Phase 5: Vue CRUD Views - Context

**Gathered:** 2026-04-07
**Status:** Ready for planning

<domain>
## Phase Boundary

All 14 remaining Vue views functional with CRUD operations, charts (vue-chartjs), and data display matching the React frontend. Includes Dashboard KPIs, 6 CRUD views with dialog forms, 4 data-intensive views with charts, Avicultura analytics dashboard, and Relatorios with PDF/Excel export. Every view must be responsive and respect dark mode/font scale.

</domain>

<decisions>
## Implementation Decisions

### CRUD Pattern & Dialogs
- **D-01:** Create/Edit forms use Vuetify `v-dialog` modals over the data table — same pattern as React MUI Dialog. Single dialog per view toggling between create/edit mode.
- **D-02:** Delete operations use a confirmation `v-dialog` before executing.
- **D-03:** All data tables use Vuetify `v-data-table` with built-in sort, pagination, search, and loading state. Equivalent to MUI DataGrid in React.
- **D-04:** Form validation uses Vuetify native `:rules` prop on form fields (required, email, minLength, etc.). No external validation library.

### Charts & Visualization
- **D-05:** Chart library is `vue-chartjs` + `chart.js` — direct equivalent of Recharts. Supports all chart types used in React: Line (growth curves, consumption, weights), Bar (monthly revenue, industry comparison), Pie/Doughnut where needed.
- **D-06:** Chart types by view:
  - Dashboard: Bar (receita mensal), Line (producao)
  - Consumo: Line (racao/agua over time)
  - Pesagem: Line (peso medio over time)
  - Avicultura: Line (crescimento), Bar (comparacao industria)
  - Sensores: Line (leituras over time)

### Export (PDF/Excel)
- **D-07:** PDF export uses `jsPDF` + `jspdf-autotable` — same libs as React. Excel export uses `xlsx` (SheetJS) — same as React. Full parity.
- **D-08:** Export logic centralized in a Vue composable `src/composables/useExport.ts` with `exportToPdf(title, columns, data)` and `exportToExcel(filename, columns, data)`. Views import and call — no duplicated export logic.

### Plan Grouping (4 plans by complexity)
- **D-09:** Plan 1 — Simple + Setup (~4 views): DashboardView, ProfileView, AuditoriaView, UsuariosView. Includes installing vue-chartjs + chart.js.
- **D-10:** Plan 2 — Medium CRUD (~4 views): GranjasView, LotesView, EstoqueView, FinanceiroView. Standard CRUD with v-dialog + v-data-table.
- **D-11:** Plan 3 — Data + Charts (~4 views): ConsumoView, PesagemView, SensoresView, SanitarioView. Data display with chart visualizations.
- **D-12:** Plan 4 — Complex (~2 views): AviculturaView (full lote analytics dashboard), RelatoriosView (6 report types + PDF/Excel export). Includes creating useExport composable.

### Carried Forward from Phase 4
- **D-13:** All Phase 4 decisions carry forward:
  - `<script setup lang="ts">` Composition API with strict TypeScript (D-05)
  - PascalCase file naming (D-06)
  - Folder layout: views/ components/ stores/ services/ composables/ (D-08)
  - Vuetify theme with exact React palette (D-01)
  - API service with Bearer token + 401 interceptor (D-10)
  - Auth store with JWT decode (D-12)
  - Accessibility store with dark mode + font scale (D-13)
  - PageContainer.vue with breadcrumbs (D-16)
  - LoadingSpinner.vue for loading states (D-16)

### Claude's Discretion
- Exact v-data-table column definitions and slot customizations per view
- Chart configuration details (colors, tooltips, responsive options)
- v-dialog width/max-width per view based on form complexity
- Snackbar/notification pattern for CRUD success/error feedback
- Composable structure for shared CRUD logic (if beneficial)
- How to handle role-based UI differences within views (show/hide buttons, filter data)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan
- `plano-migracao-granjatech.md` — Complete migration plan with frontend component mapping (React -> Vue equivalences), endpoint list

### React frontend (reference implementation — replicate these)
- `frontend/src/pages/DashboardPage.js` — KPIs + charts layout (176 lines)
- `frontend/src/pages/GranjasPage.js` — CRUD pattern template (312 lines)
- `frontend/src/pages/LotesPage.js` — CRUD with extra fields + mortalidade (375 lines)
- `frontend/src/pages/UsuariosPage.js` — Admin CRUD (335 lines)
- `frontend/src/pages/FinanceiroPage.js` — Transactions CRUD + summary (362 lines)
- `frontend/src/pages/EstoquePage.js` — Products CRUD (334 lines)
- `frontend/src/pages/ProfilePage.js` — Profile + password change (132 lines)
- `frontend/src/pages/AuditoriaPage.js` — Read-only audit log table (136 lines)
- `frontend/src/pages/ConsumoPage.js` — Feed/water consumption + charts (635 lines)
- `frontend/src/pages/PesagemPage.js` — Weighings + charts (596 lines)
- `frontend/src/pages/SensoresPage.js` — Sensors + readings + charts (489 lines)
- `frontend/src/pages/SanitarioPage.js` — Health events + vaccination schedule (689 lines)
- `frontend/src/pages/AviculturaPage.js` — Lote analytics dashboard (479 lines)
- `frontend/src/pages/RelatoriosPage.js` — 6 report types + PDF/Excel export (973 lines)

### Existing Vue scaffold (built in Phase 4)
- `granjatech-frontend/src/services/api.ts` — Axios instance, base URL, interceptors
- `granjatech-frontend/src/stores/auth.ts` — Auth Pinia store (login, logout, hydrate, user/token state)
- `granjatech-frontend/src/stores/accessibility.ts` — Dark mode + font scale store
- `granjatech-frontend/src/components/PageContainer.vue` — Page wrapper with breadcrumbs
- `granjatech-frontend/src/components/LoadingSpinner.vue` — Loading indicator
- `granjatech-frontend/src/router/index.ts` — All 16 routes with auth guards

### Existing Rust backend (API endpoints)
- `granjatech-api/src/handlers/` — All handler files define the API endpoints each view consumes
- `granjatech-api/src/dto/` — All DTOs define request/response shapes

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **PageContainer.vue**: Already built — all views wrap in this with title/subtitle
- **LoadingSpinner.vue**: Already built — use during data fetching
- **api.ts**: Axios service ready with token injection and 401 redirect
- **auth.ts store**: User role available via `useAuthStore().user?.role` for role-based UI filtering
- **PlaceholderView.vue**: Stub currently used for all 14 routes — will be replaced by real views
- **Vue Router**: All 16 routes already defined in `router/index.ts` — just need to update imports from PlaceholderView to real views

### Established Patterns
- **Composition API**: All code uses `<script setup lang="ts">` with imports from vue/vue-router/pinia
- **Vuetify components**: Layout uses v-app, v-navigation-drawer, v-app-bar, v-main, v-container
- **API calls**: `import api from '@/services/api'` then `api.get('/endpoint')`, `api.post('/endpoint', data)`
- **Pinia stores**: Composition API style with `defineStore('name', () => { ... })`
- **Theme**: Vuetify theme with exact React colors, dark/light mode toggle

### Integration Points
- **Router**: Update `router/index.ts` to import real views instead of PlaceholderView
- **API endpoints**: All 60+ endpoints on Rust backend at `http://localhost:5099/api`
- **Role filtering**: Views must filter data based on user role (Admin sees all, Produtor sees own, Financeiro sees associated)

</code_context>

<specifics>
## Specific Ideas

No specific requirements beyond total parity with React frontend. Each React page is the definitive reference for its Vue equivalent.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 05-vue-crud-views*
*Context gathered: 2026-04-07*
