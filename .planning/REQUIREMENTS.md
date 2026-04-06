# Requirements: GranjaTech Migration (.NET → Rust, React → Vue)

**Defined:** 2026-04-06
**Core Value:** Paridade total — toda funcionalidade existente deve funcionar de forma idêntica no novo stack

## v1 Requirements

### Backend Foundation (FOUND)

- [ ] **FOUND-01**: Projeto Rust (Actix-web 4) compila e inicia com pool SQLx conectando ao PostgreSQL existente
- [ ] **FOUND-02**: CORS middleware configurado para aceitar requests do frontend
- [ ] **FOUND-03**: JWT middleware extrai claims (id, email, role) e protege rotas autenticadas
- [ ] **FOUND-04**: Swagger/OpenAPI acessível via utoipa com documentação de todos os endpoints
- [ ] **FOUND-05**: Logging estruturado via tracing + tracing-subscriber
- [ ] **FOUND-06**: Configuração via .env (DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE)
- [ ] **FOUND-07**: Todas as 16 entidades convertidas para structs Rust com sqlx::FromRow
- [ ] **FOUND-08**: Todos os 36 DTOs convertidos com Serialize/Deserialize/Validate
- [ ] **FOUND-09**: Tipo de erro unificado (AppError) com respostas HTTP apropriadas

### Authentication (AUTH)

- [ ] **AUTH-01**: Usuário pode fazer login com email/senha e receber JWT com claims (id, email, role)
- [ ] **AUTH-02**: Usuário pode se registrar com email/senha
- [ ] **AUTH-03**: Admin pode listar todos os usuários
- [ ] **AUTH-04**: Admin pode buscar usuário por ID
- [ ] **AUTH-05**: Admin pode atualizar dados de usuário
- [ ] **AUTH-06**: Admin pode deletar usuário
- [ ] **AUTH-07**: BCrypt hash compatível com hashes existentes no banco (senhas .NET funcionam no Rust)

### Granjas (GRAN)

- [ ] **GRAN-01**: Usuário autenticado pode listar granjas (filtrado por role)
- [ ] **GRAN-02**: Usuário autenticado pode buscar granja por ID
- [ ] **GRAN-03**: Usuário autenticado pode criar granja
- [ ] **GRAN-04**: Usuário autenticado pode atualizar granja
- [ ] **GRAN-05**: Usuário autenticado pode deletar granja

### Lotes (LOTE)

- [ ] **LOTE-01**: Usuário pode listar lotes (filtrado por role)
- [ ] **LOTE-02**: Usuário pode buscar lote por ID
- [ ] **LOTE-03**: Usuário pode criar lote
- [ ] **LOTE-04**: Usuário pode atualizar lote
- [ ] **LOTE-05**: Usuário pode deletar lote
- [ ] **LOTE-06**: Usuário pode registrar mortalidade em lote
- [ ] **LOTE-07**: Usuário pode listar mortalidades de um lote
- [ ] **LOTE-08**: Propriedades calculadas (viabilidade, IEP, CA) funcionam corretamente

### Dashboard (DASH)

- [ ] **DASH-01**: Endpoint retorna KPIs (total granjas, lotes ativos, aves, receita)
- [ ] **DASH-02**: Endpoint retorna resumo mensal (dados agregados por mês)

### Finanças (FINA)

- [ ] **FINA-01**: Usuário pode listar transações financeiras
- [ ] **FINA-02**: Usuário pode criar transação financeira
- [ ] **FINA-03**: Usuário pode atualizar transação financeira
- [ ] **FINA-04**: Usuário pode deletar transação financeira

### Avicultura (AVIC)

- [ ] **AVIC-01**: Endpoint retorna métricas do lote
- [ ] **AVIC-02**: Endpoint retorna análise de consumo do lote
- [ ] **AVIC-03**: Endpoint retorna curvas de crescimento do lote
- [ ] **AVIC-04**: Endpoint retorna resumo sanitário do lote
- [ ] **AVIC-05**: Endpoint retorna alertas do lote
- [ ] **AVIC-06**: Endpoint retorna comparação com indústria
- [ ] **AVIC-07**: Endpoint retorna projeção de abate
- [ ] **AVIC-08**: Endpoint retorna estimativa de peso
- [ ] **AVIC-09**: Endpoint retorna dashboard completo do lote

### Consumo (CONS)

- [ ] **CONS-01**: Usuário pode registrar consumo de ração
- [ ] **CONS-02**: Usuário pode registrar consumo de água
- [ ] **CONS-03**: Usuário pode listar consumo de ração por lote
- [ ] **CONS-04**: Usuário pode listar consumo de água por lote
- [ ] **CONS-05**: Endpoint retorna resumo de consumo por lote

### Pesagem (PESA)

- [ ] **PESA-01**: Usuário pode registrar pesagem semanal
- [ ] **PESA-02**: Usuário pode listar pesagens por lote
- [ ] **PESA-03**: Endpoint retorna resumo de pesagens por lote

### Sanitário (SANI)

- [ ] **SANI-01**: Usuário pode registrar evento sanitário
- [ ] **SANI-02**: Usuário pode listar eventos sanitários por lote
- [ ] **SANI-03**: Endpoint retorna resumo sanitário por lote
- [ ] **SANI-04**: Endpoint retorna cronograma de vacinação

### Sensores (SENS)

- [ ] **SENS-01**: Usuário pode listar sensores
- [ ] **SENS-02**: Usuário pode criar sensor
- [ ] **SENS-03**: Usuário pode deletar sensor
- [ ] **SENS-04**: Usuário pode listar leituras de um sensor
- [ ] **SENS-05**: Usuário pode registrar leitura de sensor

### Estoque (ESTO)

- [ ] **ESTO-01**: Usuário pode listar produtos do estoque
- [ ] **ESTO-02**: Usuário pode criar produto
- [ ] **ESTO-03**: Usuário pode atualizar produto
- [ ] **ESTO-04**: Usuário pode deletar produto

### Auditoria (AUDI)

- [ ] **AUDI-01**: Endpoint retorna logs de auditoria
- [ ] **AUDI-02**: Service registra ações automaticamente após operações CRUD

### Perfil (PERF)

- [ ] **PERF-01**: Usuário pode ver seu perfil
- [ ] **PERF-02**: Usuário pode editar seu perfil
- [ ] **PERF-03**: Usuário pode trocar sua senha

### Relatórios (RELA)

- [ ] **RELA-01**: Endpoint health check público
- [ ] **RELA-02**: Relatório financeiro simplificado
- [ ] **RELA-03**: Relatório financeiro completo
- [ ] **RELA-04**: Relatório de produção
- [ ] **RELA-05**: Relatório de avicultura
- [ ] **RELA-06**: Relatório de desempenho por lote
- [ ] **RELA-07**: Relatório avançado com filtros

### Cache (CACH)

- [ ] **CACH-01**: Cache in-memory (moka) para endpoints pesados (dashboard, relatórios)

### Frontend Foundation (FRON)

- [ ] **FRON-01**: Projeto Vue 3 + Vuetify 3 + TypeScript + Vite configurado e buildando
- [ ] **FRON-02**: Vuetify tema migrado do MUI (cores, dark mode, fontScale)
- [ ] **FRON-03**: Auth store (Pinia) com login/logout/register e persistência em localStorage
- [ ] **FRON-04**: Accessibility store (Pinia) com dark mode toggle e font scale
- [ ] **FRON-05**: API service (Axios) com interceptors de token e redirect 401
- [ ] **FRON-06**: Vue Router com navigation guards (rotas protegidas)
- [ ] **FRON-07**: ResponsiveNavigation.vue (drawer + app bar)
- [ ] **FRON-08**: PageContainer.vue e LoadingSpinner.vue

### Frontend Views (VIEW)

- [ ] **VIEW-01**: LoginView.vue funcional contra backend Rust
- [ ] **VIEW-02**: DashboardView.vue com KPIs e gráficos (vue-chartjs)
- [ ] **VIEW-03**: GranjasView.vue com CRUD e dialogs
- [ ] **VIEW-04**: LotesView.vue com CRUD e campos extras
- [ ] **VIEW-05**: UsuariosView.vue com admin CRUD
- [ ] **VIEW-06**: FinanceiroView.vue com transações e resumo
- [ ] **VIEW-07**: EstoqueView.vue com produtos CRUD
- [ ] **VIEW-08**: ProfileView.vue com perfil e troca de senha
- [ ] **VIEW-09**: AuditoriaView.vue com tabela read-only
- [ ] **VIEW-10**: SensoresView.vue com sensores, leituras e gráficos
- [ ] **VIEW-11**: ConsumoView.vue com ração/água e gráficos
- [ ] **VIEW-12**: PesagemView.vue com pesagens e gráficos
- [ ] **VIEW-13**: SanitarioView.vue com eventos e cronograma
- [ ] **VIEW-14**: AviculturaView.vue com dashboard de lote
- [ ] **VIEW-15**: RelatoriosView.vue com relatórios, export PDF e Excel

### Docker (DOCK)

- [ ] **DOCK-01**: Dockerfile multi-stage para backend Rust (build + runtime slim)
- [ ] **DOCK-02**: Dockerfile multi-stage para frontend Vue (build + nginx)
- [ ] **DOCK-03**: docker-compose.yml com PostgreSQL 16 + Rust backend + Vue frontend
- [ ] **DOCK-04**: Health check endpoint respondendo em /health

## v2 Requirements

### Testing

- **TEST-01**: Testes de integração comparando respostas Rust vs .NET
- **TEST-02**: Testes E2E automatizados para fluxos críticos

### CI/CD

- **CICD-01**: GitHub Actions para build + test Rust
- **CICD-02**: GitHub Actions para build Vue
- **CICD-03**: Deploy automatizado para Azure

### Performance

- **PERF-01**: Benchmark comparativo .NET vs Rust (latência, throughput)
- **PERF-02**: Otimização de queries SQLx para endpoints pesados

## Out of Scope

| Feature | Reason |
|---------|--------|
| Novas funcionalidades | Foco é paridade, não features novas |
| OAuth/SSO | Email/senha suficiente para v1 (já era assim no .NET) |
| Real-time/WebSocket | Não existia no sistema atual |
| Mobile app | Web-first, mesma abordagem do sistema atual |
| Migração de dados | Mesmo banco, mesmas tabelas |
| CI/CD Azure | Deploy só Docker local por enquanto |
| Testes automatizados extensivos | Verificação manual de paridade suficiente para migração |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| FOUND-01..09 | Phase 1 | Pending |
| AUTH-01..07 | Phase 1 | Pending |
| GRAN-01..05 | Phase 1 | Pending |
| LOTE-01..08 | Phase 2 | Pending |
| DASH-01..02 | Phase 2 | Pending |
| FINA-01..04 | Phase 2 | Pending |
| CONS-01..05 | Phase 2 | Pending |
| PESA-01..03 | Phase 2 | Pending |
| SANI-01..04 | Phase 2 | Pending |
| SENS-01..05 | Phase 2 | Pending |
| ESTO-01..04 | Phase 2 | Pending |
| AUDI-01..02 | Phase 2 | Pending |
| PERF-01..03 | Phase 2 | Pending |
| AVIC-01..09 | Phase 3 | Pending |
| RELA-01..07 | Phase 3 | Pending |
| CACH-01 | Phase 3 | Pending |
| FRON-01..08 | Phase 4 | Pending |
| VIEW-01 | Phase 4 | Pending |
| VIEW-02..09 | Phase 5 | Pending |
| VIEW-10..15 | Phase 5 | Pending |
| DOCK-01..04 | Phase 6 | Pending |

**Coverage:**
- v1 requirements: 88 total
- Mapped to phases: 88
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-06*
*Last updated: 2026-04-06 after initial definition*
