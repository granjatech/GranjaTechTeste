# GranjaTech — Migração .NET 8 + React para Rust + Vue.js

## What This Is

Sistema de gestão avícola (GranjaTech) sendo migrado de .NET 8 (C#) + React 19 para Rust (Actix-web + SQLx) + Vue 3 (Vuetify 3). O sistema gerencia granjas, lotes de aves, sensores, finanças, estoque, consumo, pesagem, eventos sanitários, relatórios e auditoria. A migração deve manter paridade total de funcionalidade com o sistema atual.

## Core Value

Toda funcionalidade que existe hoje no .NET/React deve funcionar de forma idêntica no Rust/Vue — paridade total é inegociável.

## Requirements

### Validated

Funcionalidades existentes no sistema atual (inferidas do codebase):

- ✓ Autenticação JWT com 3 perfis (Administrador, Produtor, Financeiro) — existente
- ✓ CRUD de granjas com filtragem por role — existente
- ✓ CRUD de lotes com mortalidades e propriedades calculadas (IEP, CA, viabilidade) — existente
- ✓ Dashboard com KPIs e resumo mensal — existente
- ✓ Gestão financeira (transações CRUD) — existente
- ✓ Módulo de avicultura (métricas, curvas crescimento, alertas, comparação indústria, projeção abate) — existente
- ✓ Consumo de ração e água por lote — existente
- ✓ Pesagem semanal com resumo — existente
- ✓ Eventos sanitários + cronograma de vacinação — existente
- ✓ Sensores e leituras — existente
- ✓ Estoque de produtos — existente
- ✓ Auditoria de ações — existente
- ✓ Perfil do usuário + troca de senha — existente
- ✓ Relatórios (financeiro, produção, avicultura, desempenho lote, avançado) — existente
- ✓ Export PDF (jsPDF) e Excel (SheetJS) — existente
- ✓ Dark mode + font scale (acessibilidade) — existente
- ✓ Navegação responsiva (mobile + desktop) — existente
- ✓ Swagger/OpenAPI — existente
- ✓ Docker Compose (PostgreSQL + Backend + Frontend) — existente

### Active

Migração para novo stack mantendo paridade:

- [ ] Backend Rust (Actix-web 4 + SQLx) implementando todos os 60+ endpoints
- [ ] Frontend Vue 3 (Vuetify 3 + TypeScript) com todas as 15 views
- [ ] JWT compatível (mesma estrutura de claims: id, email, role)
- [ ] BCrypt hash compatível com senhas existentes no PostgreSQL
- [ ] Mesmo banco PostgreSQL 16 — sem migração de schema
- [ ] Swagger/OpenAPI via utoipa
- [ ] Docker Compose atualizado (Rust + Vue no lugar de .NET + React)
- [ ] Cache in-memory via moka (substituindo MemoryCache)
- [ ] Gráficos via vue-chartjs (substituindo Recharts)
- [ ] Pinia stores (substituindo React Context)
- [ ] Vue Router com navigation guards (substituindo ProtectedRoute)

### Out of Scope

- CI/CD Azure — deploy só Docker local por enquanto
- GitHub Actions — não necessário nesta migração
- Novas funcionalidades — foco é paridade, não features novas
- Testes automatizados extensivos — testes manuais de paridade são suficientes
- Migração de dados — mesmo banco, mesmas tabelas

## Context

- Codebase atual: ~4.650 linhas backend (.NET 8, Clean Architecture, 4 projetos), ~7.500 linhas frontend (React 19 + MUI 7)
- 16 entidades de domínio, 36 DTOs, 11 interfaces de serviço, 15 controllers, 15 páginas
- PostgreSQL 16 com seed data (3 perfis, 1 admin padrão)
- Role-based filtering: Administrador vê tudo, Produtor vê só seus dados, Financeiro vê produtores associados
- Plano de migração detalhado disponível em `plano-migracao-granjatech.md`
- Mapeamento completo de endpoints, entidades, dependências e equivalências de componentes

## Constraints

- **Stack alvo**: Rust (Actix-web 4, SQLx, jsonwebtoken, bcrypt, utoipa, moka, tracing) + Vue 3 (Vuetify 3, Pinia, Vue Router 4, TypeScript, Vite)
- **Banco**: PostgreSQL 16 existente — SQLx conecta nas mesmas tabelas, sem migrations destrutivas
- **Compatibilidade**: BCrypt hashes devem ser compatíveis entre .NET e Rust
- **API Contract**: Mesmos endpoints, mesmas rotas, mesmos payloads JSON
- **Deploy**: Docker Compose local apenas

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| SQLx (não Diesel/SeaORM) | Compile-time checked queries, mais próximo do SQL puro, sem ORM pesado | — Pending |
| Actix-web (não Axum) | Ecossistema maduro, boa performance, ampla documentação | — Pending |
| Vuetify 3 (não PrimeVue/Quasar) | Equivalência mais direta com MUI, components similares | — Pending |
| Pinia (não Vuex) | Recomendação oficial Vue 3, API mais simples que Vuex | — Pending |
| Manter mesmo banco | Evita migração de dados, reduz risco, senhas BCrypt compatíveis | — Pending |
| Docker local only | Simplifica escopo, CI/CD pode ser adicionado depois | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

## Current State

Phase 01 (Rust Foundation) complete — compilable Rust project with Actix-web 4, SQLx models (16 entities), JWT auth, auth/granja CRUD services and handlers, CORS, Swagger UI. Human verification pending for DB connection, BCrypt compatibility, Swagger UI render, and role-based filtering.

---
*Last updated: 2026-04-07 after Phase 01 completion*
