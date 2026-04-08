<div align="center">

# GranjaTech

**Sistema de Gestao Avicola de Alta Performance**

Plataforma completa para gestao de granjas de corte — monitoramento de lotes, controle financeiro, sensores IoT, relatorios avancados e auditoria em tempo real.

[![Rust](https://img.shields.io/badge/Backend-Rust%20%7C%20Actix--web-000000?style=for-the-badge&logo=rust&logoColor=white)](https://actix.rs/)
[![Vue.js](https://img.shields.io/badge/Frontend-Vue%203%20%7C%20Vuetify-4FC08D?style=for-the-badge&logo=vue.js&logoColor=white)](https://vuejs.org/)
[![PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL%2016-4169E1?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/Deploy-Docker%20Compose-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://docs.docker.com/compose/)

</div>

---

## Arquitetura

```
                        ┌─────────────────────────────────────────────┐
                        │              Docker Compose                  │
                        │                                             │
   Browser :80   ──────►│  ┌──────────┐    ┌──────────┐  ┌────────┐  │
                        │  │  nginx   │───►│  Actix   │─►│ Postgres│  │
                        │  │  + Vue   │    │  Web API │  │   16   │  │
                        │  │  :80     │    │  :8080   │  │  :5432 │  │
                        │  └──────────┘    └──────────┘  └────────┘  │
                        │   /api ──────────► proxy_pass               │
                        │   /swagger-ui ──► proxy_pass               │
                        │   /* ────────────► SPA (Vue Router)        │
                        └─────────────────────────────────────────────┘
```

| Camada | Tecnologia | Responsabilidade |
|--------|-----------|------------------|
| **Frontend** | Vue 3 + Vuetify 3 + TypeScript + Pinia | 16 views, graficos, export PDF/Excel |
| **Reverse Proxy** | nginx (Alpine) | Routing, gzip, security headers, cache |
| **Backend** | Rust + Actix-web 4 + SQLx | 65 endpoints REST, JWT auth, business logic |
| **Database** | PostgreSQL 16 | Dados persistentes, seed via SQL scripts |
| **Docs** | utoipa + Swagger UI | Documentacao OpenAPI auto-gerada |

---

## Quick Start

```bash
# Clone e suba tudo com um comando
git clone <repo-url> && cd GranjaTechTeste
docker compose up --build -d

# Aguarde ~2min (primeira build compila Rust)
# Builds subsequentes usam cache do cargo-chef (~30s)
```

| Servico | URL | Descricao |
|---------|-----|-----------|
| **App** | http://localhost | Interface principal |
| **API** | http://localhost:8080 | Acesso direto ao backend |
| **Swagger** | http://localhost/swagger-ui/ | Documentacao interativa da API |
| **pgAdmin** | http://localhost:5050 | `docker compose -f docker-compose.dev.yml up -d` |

### Desenvolvimento Local

```bash
# Terminal 1 — Database
docker compose -f docker-compose.dev.yml up -d   # PostgreSQL + pgAdmin

# Terminal 2 — Backend (hot reload com cargo-watch)
cd granjatech-api
cp .env.example .env                               # configurar DATABASE_URL
cargo watch -x run                                 # http://localhost:8080

# Terminal 3 — Frontend (HMR com Vite)
cd granjatech-frontend
npm install
npm run dev                                        # http://localhost:5173
```

---

## Funcionalidades

### Gestao Operacional
- **Granjas** — cadastro com filtragem por perfil de acesso
- **Lotes** — ciclo completo: criacao, mortalidade, pesagem, abate
- **Metricas calculadas** — IEP, conversao alimentar, viabilidade, ganho medio diario
- **Consumo** — racao e agua por lote com analise de tendencia
- **Pesagem semanal** — registro e acompanhamento de peso medio
- **Estoque** — controle de produtos com movimentacoes

### Monitoramento
- **Sensores IoT** — temperatura, umidade, qualidade do ar
- **Dashboard** — KPIs em tempo real com graficos interativos
- **Alertas** — parametros fora do padrao da industria
- **Curvas de crescimento** — projecao de abate e comparacao com benchmarks

### Financeiro
- **Transacoes** — receitas e despesas por granja/lote
- **Relatorios** — financeiro, producao, avicultura, desempenho de lote
- **Export** — PDF (jsPDF) e Excel (SheetJS) em todos os relatorios

### Seguranca e Compliance
- **JWT Authentication** — tokens com 8h de expiracao
- **3 perfis RBAC** — Administrador, Produtor, Financeiro
- **Auditoria completa** — log de todas as acoes com usuario e timestamp
- **Eventos sanitarios** — registro com cronograma de vacinacao

---

## Stack Tecnica

### Backend — `granjatech-api/`

```
Rust 1.x (stable)
├── actix-web 4        Servidor HTTP async de alta performance
├── sqlx 0.8           Query builder com verificacao em compile-time
├── jsonwebtoken       Autenticacao JWT (claims: id, email, role)
├── bcrypt             Hash de senhas (compativel com .NET BCrypt)
├── utoipa + swagger   Documentacao OpenAPI auto-gerada
├── moka               Cache in-memory async (TTL configuravel)
├── tracing            Structured logging
└── validator          Validacao de DTOs
```

### Frontend — `granjatech-frontend/`

```
Vue 3.5 + TypeScript
├── Vuetify 3          Component library (Material Design 3)
├── Pinia              State management
├── Vue Router 4       Routing com guards por role
├── vue-chartjs        Graficos (Chart.js wrapper)
├── jsPDF              Geracao de relatorios PDF
├── SheetJS (xlsx)     Export para Excel
└── Vite 6             Build tool com HMR
```

### Infraestrutura

```
Docker Compose
├── postgres:16-alpine     Database com healthcheck (pg_isready)
├── granjatech-api         Rust binary em debian:bookworm-slim (non-root)
│   └── cargo-chef         4-stage build com cache de dependencias
├── granjatech-frontend    nginx:alpine com reverse proxy
│   └── nginx.conf         /api proxy, security headers, gzip, SPA routing
└── granjatech-network     Bridge network isolada
```

---

## Estrutura do Projeto

```
GranjaTechTeste/
├── granjatech-api/                 # Backend Rust
│   ├── src/
│   │   ├── main.rs                 # Entry point + route registration
│   │   ├── models/                 # Domain entities (SQLx FromRow)
│   │   ├── handlers/               # HTTP handlers por dominio
│   │   ├── services/               # Business logic + cache
│   │   ├── middleware/             # JWT auth middleware
│   │   └── dto/                    # Request/Response DTOs
│   ├── Cargo.toml
│   ├── Dockerfile                  # Multi-stage cargo-chef build
│   └── .dockerignore
│
├── granjatech-frontend/            # Frontend Vue 3
│   ├── src/
│   │   ├── views/                  # 16 views (pages)
│   │   ├── components/             # Componentes reutilizaveis
│   │   ├── stores/                 # Pinia stores
│   │   ├── router/                 # Vue Router + guards
│   │   ├── services/               # API client (axios)
│   │   └── types/                  # TypeScript interfaces
│   ├── package.json
│   ├── nginx.conf                  # Reverse proxy + security
│   ├── Dockerfile                  # Node build + nginx serve
│   └── .dockerignore
│
├── docs/                           # SQL seed scripts (mounted in postgres)
├── docker-compose.yml              # Production stack
└── docker-compose.dev.yml          # Dev: PostgreSQL + pgAdmin
```

---

## API

65 endpoints REST organizados por dominio:

| Modulo | Endpoints | Auth | Descricao |
|--------|-----------|------|-----------|
| `/api/auth` | 2 | Public | Login, registro |
| `/api/granjas` | 5 | JWT | CRUD + filtragem por role |
| `/api/lotes` | 5 | JWT | CRUD + mortalidades |
| `/api/dashboard` | 2 | JWT | KPIs e resumo mensal |
| `/api/financas` | 4 | JWT | Transacoes financeiras |
| `/api/avicultura` | 5 | JWT | Metricas, curvas, alertas, projecao |
| `/api/consumo` | 4 | JWT | Racao e agua |
| `/api/pesagem` | 4 | JWT | Pesagem semanal |
| `/api/sanitario` | 4 | JWT | Eventos + vacinacao |
| `/api/sensores` | 5 | JWT | Sensores + leituras |
| `/api/estoque` | 4 | JWT | Produtos |
| `/api/auditoria` | 2 | Admin | Logs de auditoria |
| `/api/relatorios` | 8 | JWT | Relatorios avancados |
| `/api/profile` | 3 | JWT | Perfil + troca de senha |
| `/api/users` | 4 | Admin | Gestao de usuarios |
| `/health` | 1 | Public | Health check |

Documentacao completa em **http://localhost/swagger-ui/** apos `docker compose up`.

---

## Variaveis de Ambiente

| Variavel | Default (compose) | Descricao |
|----------|-------------------|-----------|
| `DATABASE_URL` | `postgres://postgres:postgres123@postgres:5432/GranjaTechDb` | Conexao PostgreSQL |
| `JWT_KEY` | `74b9f1d2-...` | Chave de assinatura JWT |
| `JWT_ISSUER` | `GranjaTechAPI` | Issuer do token |
| `JWT_AUDIENCE` | `GranjaTechApp` | Audience do token |
| `ALLOWED_ORIGINS` | `http://localhost:80` | CORS origins (`;` separados) |
| `SWAGGER_ENABLED` | `true` | Habilitar Swagger UI |
| `RUST_LOG` | `info` | Nivel de log (tracing) |
| `VITE_API_URL` | `/api` | Base URL da API (baked at build time) |

---

## Seguranca

- Containers rodam com **usuario non-root** (`appuser`)
- `.dockerignore` exclui `.env`, `target/`, `node_modules/` das imagens
- nginx adiciona `X-Frame-Options`, `X-Content-Type-Options`, `X-XSS-Protection`
- `index.html` servido com `no-cache` — assets estaticos com hash + cache 1 ano
- SQL seed scripts montados como **read-only** (`:ro`)
- BCrypt para hashing de senhas (compativel cross-platform)

---

## Equipe

Projeto academico **FATEC** migrado de .NET 8 + React para Rust + Vue.js.

| Nome | Responsabilidade |
|------|-----------------|
| **Felipe Bianchini** | Backend, API, infraestrutura Docker |
| **Wendell Nascimento** | Frontend, componentes, testes |
| **Guilherme Oliveira** | Banco de dados, auditoria, endpoints |
| **Adryan Thiago** | Relatorios, dashboards, sensores |

---

<div align="center">

**10.200+ linhas Rust** | **6.300+ linhas Vue/TS** | **65 endpoints** | **16 views** | **3 containers**

Built with Rust + Vue.js

</div>
