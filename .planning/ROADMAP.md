# Roadmap: GranjaTech Migration (.NET 8 + React -> Rust + Vue.js)

## Overview

Migrate GranjaTech from .NET 8 (C#) + React 19 to Rust (Actix-web + SQLx) + Vue 3 (Vuetify 3), maintaining total feature parity with the existing system. The same PostgreSQL 16 database stays in place -- no data migration, no schema changes. Six phases take us from a compilable Rust backend with auth through complete CRUD, advanced business logic, Vue frontend scaffold, all 15 views, and finally Docker containerization.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Rust Foundation** - Models, DTOs, config, JWT auth, and Granjas CRUD as the template for all handlers
- [ ] **Phase 2: Complete Backend CRUD** - All remaining endpoints (Lotes, Dashboard, Financas, Consumo, Pesagem, Sanitario, Sensores, Estoque, Auditoria, Profile)
- [ ] **Phase 3: Reports & Business Logic** - Avicultura analytics, all reports, cache layer, and audit automation
- [ ] **Phase 4: Vue Scaffold + Auth** - Vue 3 project setup, Pinia stores, router guards, layout components, and LoginView
- [ ] **Phase 5: Vue CRUD Views** - All 15 views with CRUD, charts, and export functionality
- [ ] **Phase 6: Docker & Finalization** - Dockerfiles, docker-compose, and health check endpoint

## Phase Details

### Phase 1: Rust Foundation
**Goal**: A running Rust backend that authenticates users and performs Granjas CRUD against the existing PostgreSQL database
**Depends on**: Nothing (first phase)
**Requirements**: FOUND-01, FOUND-02, FOUND-03, FOUND-04, FOUND-05, FOUND-06, FOUND-07, FOUND-08, FOUND-09, AUTH-01, AUTH-02, AUTH-03, AUTH-04, AUTH-05, AUTH-06, AUTH-07, GRAN-01, GRAN-02, GRAN-03, GRAN-04, GRAN-05
**Success Criteria** (what must be TRUE):
  1. Rust server starts and connects to the existing PostgreSQL database without errors
  2. User can log in with an existing .NET-created BCrypt password and receive a valid JWT
  3. Admin can perform full CRUD on users (list, get, create, update, delete)
  4. Authenticated user can perform full CRUD on Granjas with role-based filtering
  5. Swagger UI is accessible and documents all implemented endpoints
**Plans:** 3 plans

Plans:
- [x] 01-01-PLAN.md -- Project scaffold: Cargo.toml, config, errors, db, tracing, main.rs skeleton
- [x] 01-02-PLAN.md -- All 16 entity models, all DTOs, JWT Claims middleware extractor
- [x] 01-03-PLAN.md -- Auth service + Granjas service + handlers + CORS + Swagger UI wiring

### Phase 2: Complete Backend CRUD
**Goal**: Every CRUD endpoint from the original .NET backend responds identically in the Rust backend
**Depends on**: Phase 1
**Requirements**: LOTE-01, LOTE-02, LOTE-03, LOTE-04, LOTE-05, LOTE-06, LOTE-07, LOTE-08, DASH-01, DASH-02, FINA-01, FINA-02, FINA-03, FINA-04, CONS-01, CONS-02, CONS-03, CONS-04, CONS-05, PESA-01, PESA-02, PESA-03, SANI-01, SANI-02, SANI-03, SANI-04, SENS-01, SENS-02, SENS-03, SENS-04, SENS-05, ESTO-01, ESTO-02, ESTO-03, ESTO-04, AUDI-01, AUDI-02, PERF-01, PERF-02, PERF-03
**Success Criteria** (what must be TRUE):
  1. User can perform full CRUD on Lotes including mortalidade registration and calculated properties (IEP, CA, viabilidade)
  2. Dashboard KPIs and monthly summary endpoints return correct aggregated data
  3. User can manage financial transactions, consumption records, weighings, sanitary events, sensors, and stock products via their respective endpoints
  4. Profile endpoints allow viewing/editing profile and changing password
  5. Audit logs are recorded automatically for CRUD operations
**Plans:** 3 plans

Plans:
- [x] 02-01-PLAN.md -- Lotes CRUD (mortalidade, computed properties) + Dashboard (KPIs, monthly summary)
- [x] 02-02-PLAN.md -- Financas (5-min edit window) + Consumo + Pesagem + Sanitario
- [x] 02-03-PLAN.md -- Sensores (public leituras) + Estoque + Auditoria + Profile (password change)

### Phase 3: Reports & Business Logic
**Goal**: All advanced analytics (avicultura module) and report endpoints produce results identical to the .NET backend
**Depends on**: Phase 2
**Requirements**: AVIC-01, AVIC-02, AVIC-03, AVIC-04, AVIC-05, AVIC-06, AVIC-07, AVIC-08, AVIC-09, RELA-01, RELA-02, RELA-03, RELA-04, RELA-05, RELA-06, RELA-07, CACH-01
**Success Criteria** (what must be TRUE):
  1. Avicultura dashboard endpoint returns complete lote analytics (metrics, growth curves, alerts, industry comparison, slaughter projection)
  2. All 6 report endpoints (simplified financial, full financial, production, aviculture, lote performance, advanced with filters) return correct data
  3. Health check endpoint responds at /health
  4. Cache layer (moka) reduces response time for heavy endpoints (dashboard, reports)
**Plans:** 3 plans

Plans:
- [ ] 03-01-PLAN.md -- AviculturaService (9 endpoints: metricas, alertas, comparacao-industria, dashboard, stubs)
- [ ] 03-02-PLAN.md -- RelatorioService + RelatorioAvancadoService (6 report endpoints with role filtering)
- [ ] 03-03-PLAN.md -- CacheService (moka) + health check enhancement + cache wiring into handlers

### Phase 4: Vue Scaffold + Auth
**Goal**: Users can log in to the Vue frontend against the Rust backend, navigate protected routes, and toggle dark mode / font scale
**Depends on**: Phase 3
**Requirements**: FRON-01, FRON-02, FRON-03, FRON-04, FRON-05, FRON-06, FRON-07, FRON-08, VIEW-01
**Success Criteria** (what must be TRUE):
  1. Vue 3 + Vuetify 3 project builds successfully with Vite
  2. User can log in via LoginView and session persists across browser refreshes (localStorage)
  3. Unauthenticated users are redirected to login; authenticated users see the navigation drawer and app bar
  4. Dark mode toggle and font scale adjustment work correctly
**Plans**: TBD
**UI hint**: yes

Plans:
- [ ] 04-01: TBD
- [ ] 04-02: TBD
- [ ] 04-03: TBD

### Phase 5: Vue CRUD Views
**Goal**: All 14 remaining views are functional with CRUD operations, charts, and data display matching the React frontend
**Depends on**: Phase 4
**Requirements**: VIEW-02, VIEW-03, VIEW-04, VIEW-05, VIEW-06, VIEW-07, VIEW-08, VIEW-09, VIEW-10, VIEW-11, VIEW-12, VIEW-13, VIEW-14, VIEW-15
**Success Criteria** (what must be TRUE):
  1. Dashboard displays KPIs and charts (vue-chartjs) with real data from the Rust backend
  2. All CRUD views (Granjas, Lotes, Usuarios, Financeiro, Estoque, Sensores) allow creating, reading, updating, and deleting records through dialogs
  3. Data-intensive views (Consumo, Pesagem, Sanitario, Avicultura) display charts and summaries correctly
  4. Relatorios view generates reports and exports to PDF (jsPDF) and Excel (SheetJS)
  5. All views are responsive (mobile + desktop) and respect the current theme/font scale
**Plans**: TBD
**UI hint**: yes

Plans:
- [ ] 05-01: TBD
- [ ] 05-02: TBD
- [ ] 05-03: TBD
- [ ] 05-04: TBD

### Phase 6: Docker & Finalization
**Goal**: The entire stack (PostgreSQL + Rust backend + Vue frontend) runs via a single docker-compose command
**Depends on**: Phase 5
**Requirements**: DOCK-01, DOCK-02, DOCK-03, DOCK-04
**Success Criteria** (what must be TRUE):
  1. `docker-compose up` starts all three containers (PostgreSQL, Rust backend, Vue/nginx frontend) without errors
  2. Health check endpoint at /health responds with 200 OK from within the container
  3. User can access the full application through the nginx-served frontend and perform end-to-end workflows
**Plans**: TBD

Plans:
- [ ] 06-01: TBD
- [ ] 06-02: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Rust Foundation | 0/3 | Planned | - |
| 2. Complete Backend CRUD | 0/3 | Planned | - |
| 3. Reports & Business Logic | 0/3 | Not started | - |
| 4. Vue Scaffold + Auth | 0/3 | Not started | - |
| 5. Vue CRUD Views | 0/4 | Not started | - |
| 6. Docker & Finalization | 0/2 | Not started | - |
