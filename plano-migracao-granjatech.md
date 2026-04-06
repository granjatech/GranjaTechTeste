# Plano de Migração GranjaTech

## De .NET 8 + React para Rust + Vue.js

---

## 1. Diagnóstico do Projeto Atual

### Backend (.NET 8 / C# — Clean Architecture)

| Camada | Conteudo | Linhas |
|--------|----------|--------|
| **Domain** | 16 entidades (Granja, Lote, Usuario, Sensor, LeituraSensor, TransacaoFinanceira, ConsumoRacao, ConsumoAgua, EventoSanitario, PesagemSemanal, RegistroMortalidade, LogAuditoria, Perfil, QualidadeAr, RegistroAbate, FinanceiroProdutor) | ~600 |
| **Application** | 36 DTOs + 11 interfaces de servico | ~750 |
| **Infrastructure** | 12 services, DbContext, migrations, seed data, cache | ~2.400 |
| **Api** | 15 controllers, Program.cs (JWT, CORS, Swagger, EF) | ~900 |

**Stack atual:** EF Core + Npgsql (PostgreSQL), JWT Bearer, BCrypt, Swagger, MemoryCache.

### Frontend (React 19 + Material UI 7)

| Area | Arquivos | Linhas |
|------|----------|--------|
| Pages | 15 paginas | ~6.200 |
| Components | 5 componentes (Nav, ProtectedRoute, Loading, PageContainer, Notification) | ~700 |
| Services | apiService.js + relatoriosApi.js (Axios) | ~460 |
| Context | AuthContext + AccessibilityContext | ~160 |
| **Total** | ~25 arquivos | **~7.500** |

### Infraestrutura

Docker Compose com 3 containers (PostgreSQL 16, Backend .NET, Frontend Nginx). Deploy via GitHub Actions para Azure (Static Web Apps + App Service).

---

## 2. Arquitetura Alvo

### Backend: Rust (Actix-web + SQLx)

```
granjatech-api/
├── Cargo.toml
├── .env
├── migrations/                  # SQLx migrations (SQL puro)
│   ├── 001_create_perfis.sql
│   ├── 002_create_usuarios.sql
│   ├── ...
│   └── 014_seed_data.sql
├── src/
│   ├── main.rs                  # Bootstrap: server, pool, CORS, rotas
│   ├── config.rs                # Leitura de envs (JWT secret, DB URL, etc.)
│   ├── errors.rs                # Tipo de erro unificado (AppError)
│   ├── models/                  # Equivalente ao Domain
│   │   ├── mod.rs
│   │   ├── usuario.rs
│   │   ├── granja.rs
│   │   ├── lote.rs
│   │   ├── sensor.rs
│   │   ├── leitura_sensor.rs
│   │   ├── transacao_financeira.rs
│   │   ├── consumo_racao.rs
│   │   ├── consumo_agua.rs
│   │   ├── evento_sanitario.rs
│   │   ├── pesagem_semanal.rs
│   │   ├── registro_mortalidade.rs
│   │   ├── registro_abate.rs
│   │   ├── qualidade_ar.rs
│   │   ├── log_auditoria.rs
│   │   ├── perfil.rs
│   │   ├── financeiro_produtor.rs
│   │   └── produto.rs
│   ├── dto/                     # Equivalente ao Application/DTOs
│   │   ├── mod.rs
│   │   ├── auth.rs              # LoginDto, RegisterDto, LoginResponseDto, etc.
│   │   ├── granja.rs            # CreateGranjaDto, UpdateGranjaDto
│   │   ├── lote.rs              # CreateLoteDto, UpdateLoteDto
│   │   ├── dashboard.rs         # DashboardKpiDto, MonthlySummaryDto
│   │   ├── financeiro.rs        # CreateTransacaoDto, RelatorioFinanceiroDto, etc.
│   │   ├── avicultura.rs        # Metricas, curvas, alertas, etc.
│   │   ├── consumo.rs           # CreateConsumoRacaoDto, CreateConsumoAguaDto
│   │   ├── pesagem.rs           # CreatePesagemSemanalDto
│   │   ├── sanitario.rs         # CreateEventoSanitarioDto
│   │   ├── sensor.rs            # CreateSensorDto, CreateLeituraDto
│   │   ├── relatorios.rs        # GeralReportDto, FinanceReportDto, SetorReportDto
│   │   └── profile.rs           # ProfileDetailDto, UpdateProfileDto, ChangePasswordDto
│   ├── handlers/                # Equivalente aos Controllers
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── granjas.rs
│   │   ├── lotes.rs
│   │   ├── dashboard.rs
│   │   ├── financas.rs
│   │   ├── avicultura.rs
│   │   ├── consumo.rs
│   │   ├── pesagem.rs
│   │   ├── sanitario.rs
│   │   ├── sensores.rs
│   │   ├── leituras.rs
│   │   ├── estoque.rs
│   │   ├── auditoria.rs
│   │   ├── relatorios.rs
│   │   └── profile.rs
│   ├── services/                # Logica de negocio (equivalente ao Infrastructure/Services)
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── granja_service.rs
│   │   ├── lote_service.rs
│   │   ├── dashboard_service.rs
│   │   ├── financas_service.rs
│   │   ├── avicultura_service.rs
│   │   ├── relatorio_service.rs
│   │   ├── relatorio_avancado_service.rs
│   │   ├── sensor_service.rs
│   │   ├── estoque_service.rs
│   │   ├── auditoria_service.rs
│   │   └── cache_service.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── jwt.rs               # Extractor de claims JWT
│   │   └── logging.rs           # Request tracking middleware
│   └── db.rs                    # Pool de conexoes (PgPool)
└── tests/
    ├── integration/
    └── common/
```

### Frontend: Vue 3 + Vuetify 3

```
granjatech-frontend/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── public/
│   ├── favicon.ico
│   └── logo192.png
├── src/
│   ├── main.ts
│   ├── App.vue
│   ├── router/
│   │   └── index.ts             # Vue Router (equivale ao React Router)
│   ├── stores/                  # Pinia (equivale aos Contexts)
│   │   ├── auth.ts              # AuthStore (substitui AuthContext)
│   │   └── accessibility.ts     # AccessibilityStore (substitui AccessibilityContext)
│   ├── composables/             # Logica reusavel (hooks equivalentes)
│   │   ├── useNotification.ts   # Substitui NotificationContext
│   │   └── useApi.ts            # Interceptors, token, base URL
│   ├── services/
│   │   ├── api.ts               # Instancia Axios com interceptors
│   │   └── relatoriosApi.ts     # Chamadas especificas de relatorios
│   ├── components/
│   │   ├── ResponsiveNavigation.vue
│   │   ├── ProtectedRoute.vue   # Navigation guard (ou middleware de rota)
│   │   ├── LoadingSpinner.vue
│   │   └── PageContainer.vue
│   ├── views/                   # Equivalente as Pages
│   │   ├── LoginView.vue
│   │   ├── DashboardView.vue
│   │   ├── GranjasView.vue
│   │   ├── LotesView.vue
│   │   ├── UsuariosView.vue
│   │   ├── FinanceiroView.vue
│   │   ├── AuditoriaView.vue
│   │   ├── ProfileView.vue
│   │   ├── EstoqueView.vue
│   │   ├── AviculturaView.vue
│   │   ├── ConsumoView.vue
│   │   ├── PesagemView.vue
│   │   ├── SanitarioView.vue
│   │   ├── SensoresView.vue
│   │   └── RelatoriosView.vue
│   ├── plugins/
│   │   └── vuetify.ts           # Config do Vuetify (substitui theme.js do MUI)
│   └── styles/
│       └── main.css
└── env.d.ts
```

---

## 3. Mapeamento Tecnologico

| Funcionalidade | .NET 8 / React (Atual) | Rust / Vue (Alvo) |
|---|---|---|
| Framework HTTP | ASP.NET Core | **Actix-web 4** |
| ORM / Banco | EF Core + Npgsql | **SQLx** (compile-time checked queries) |
| Migrations | EF Core Migrations (C#) | **SQLx CLI** (SQL puro) |
| Autenticacao | JWT Bearer (Microsoft.IdentityModel) | **jsonwebtoken** crate |
| Hash de senha | BCrypt.Net | **bcrypt** crate (ou argon2) |
| Serializacao | System.Text.Json | **serde + serde_json** |
| Validacao | DataAnnotations | **validator** crate |
| Docs API | Swagger / Swashbuckle | **utoipa** (OpenAPI auto-gerado) |
| Cache | MemoryCache | **moka** (cache in-memory async) |
| Logging | Microsoft.Extensions.Logging | **tracing + tracing-subscriber** |
| CORS | ASP.NET CORS Middleware | **actix-cors** |
| Env config | appsettings.json | **dotenvy + envy** |
| UI Framework | React 19 + MUI 7 | **Vue 3 + Vuetify 3** |
| Roteamento | React Router 7 | **Vue Router 4** |
| Estado global | Context API | **Pinia** |
| HTTP Client | Axios | **Axios** (mesmo) |
| Graficos | Recharts | **vue-chartjs** (Chart.js) ou **vue-apexcharts** |
| PDF export | jsPDF + autoTable | **jsPDF + autoTable** (mesmo) |
| Excel export | SheetJS (xlsx) | **SheetJS (xlsx)** (mesmo) |
| Build tool | react-scripts (Webpack) | **Vite** |
| Linguagem | JavaScript | **TypeScript** |

---

## 4. Mapeamento Completo de Endpoints (API Contract)

A API Rust deve implementar exatamente estes endpoints para manter compatibilidade:

### Auth (`/api/auth`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/auth/usuarios` | Administrador |
| GET | `/api/auth/usuarios/{id}` | Administrador |
| POST | `/api/auth/registrar` | Publico |
| POST | `/api/auth/login` | Publico |
| PUT | `/api/auth/usuarios/{id}` | Administrador |
| DELETE | `/api/auth/usuarios/{id}` | Administrador |

### Granjas (`/api/granjas`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/granjas` | Autenticado |
| GET | `/api/granjas/{id}` | Autenticado |
| POST | `/api/granjas` | Autenticado |
| PUT | `/api/granjas/{id}` | Autenticado |
| DELETE | `/api/granjas/{id}` | Autenticado |

### Lotes (`/api/lotes`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/lotes` | Autenticado |
| GET | `/api/lotes/{id}` | Autenticado |
| POST | `/api/lotes` | Autenticado |
| PUT | `/api/lotes/{id}` | Autenticado |
| DELETE | `/api/lotes/{id}` | Autenticado |
| POST | `/api/lotes/{id}/mortalidades` | Autenticado |
| GET | `/api/lotes/{id}/mortalidades` | Autenticado |

### Dashboard (`/api/dashboard`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/dashboard/kpis` | Autenticado |
| GET | `/api/dashboard/resumo-mensal` | Autenticado |

### Financas (`/api/financas`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/financas` | Autenticado |
| POST | `/api/financas` | Autenticado |
| PUT | `/api/financas/{id}` | Autenticado |
| DELETE | `/api/financas/{id}` | Autenticado |

### Avicultura (`/api/avicultura`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/avicultura/{loteId}/metricas` | Autenticado |
| GET | `/api/avicultura/{loteId}/analise-consumo` | Autenticado |
| GET | `/api/avicultura/{loteId}/curvas-crescimento` | Autenticado |
| GET | `/api/avicultura/{loteId}/resumo-sanitario` | Autenticado |
| GET | `/api/avicultura/{loteId}/alertas` | Autenticado |
| GET | `/api/avicultura/{loteId}/comparacao-industria` | Autenticado |
| GET | `/api/avicultura/{loteId}/projecao-abate` | Autenticado |
| GET | `/api/avicultura/{loteId}/estimar-peso` | Autenticado |
| GET | `/api/avicultura/{loteId}/dashboard` | Autenticado |

### Consumo (`/api/consumo`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| POST | `/api/consumo/racao` | Autenticado |
| POST | `/api/consumo/agua` | Autenticado |
| GET | `/api/consumo/racao/{loteId}` | Autenticado |
| GET | `/api/consumo/agua/{loteId}` | Autenticado |
| GET | `/api/consumo/resumo/{loteId}` | Autenticado |

### Pesagem (`/api/pesagem`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| POST | `/api/pesagem` | Autenticado |
| GET | `/api/pesagem/{loteId}` | Autenticado |
| GET | `/api/pesagem/{loteId}/resumo` | Autenticado |

### Sanitario (`/api/sanitario`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| POST | `/api/sanitario` | Autenticado |
| GET | `/api/sanitario/{loteId}` | Autenticado |
| GET | `/api/sanitario/{loteId}/resumo` | Autenticado |
| GET | `/api/sanitario/cronograma-vacinacao` | Autenticado |

### Sensores (`/api/sensores`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/sensores` | Autenticado |
| POST | `/api/sensores` | Autenticado |
| DELETE | `/api/sensores/{id}` | Autenticado |
| GET | `/api/sensores/{sensorId}/leituras` | Autenticado |

### Leituras (`/api/leituras`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| POST | `/api/leituras` | Autenticado |

### Estoque (`/api/estoque`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/estoque` | Autenticado |
| POST | `/api/estoque` | Autenticado |
| PUT | `/api/estoque/{id}` | Autenticado |
| DELETE | `/api/estoque/{id}` | Autenticado |

### Auditoria (`/api/auditoria`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/auditoria` | Autenticado |

### Profile (`/api/profile`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/profile` | Autenticado |
| PUT | `/api/profile` | Autenticado |
| POST | `/api/profile/change-password` | Autenticado |

### Relatorios (`/api/relatorios`)
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/api/relatorios/health` | Publico |
| GET | `/api/relatorios/financeiro-simplificado` | Autenticado |
| GET | `/api/relatorios/financeiro` | Autenticado |
| GET | `/api/relatorios/producao` | Autenticado |
| GET | `/api/relatorios/avicultura` | Autenticado |
| GET | `/api/relatorios/desempenho-lote/{loteId}` | Autenticado |
| GET | `/api/relatorios/avancado` | Autenticado |

### Health/Root
| Metodo | Rota | Autorizacao |
|--------|------|-------------|
| GET | `/health` | Publico |
| GET | `/` | Publico (redireciona para Swagger) |

---

## 5. Mapeamento de Entidades (C# para Rust)

Exemplo concreto da conversao de cada entidade:

### Granja
```csharp
// C# (atual)
public class Granja {
    public int Id { get; set; }
    public string Codigo { get; set; }
    public string Nome { get; set; }
    public string? Localizacao { get; set; }
    public int UsuarioId { get; set; }
    public Usuario Usuario { get; set; }
}
```
```rust
// Rust (alvo)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Granja {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub localizacao: Option<String>,
    pub usuario_id: i32,
}
```

### Regras gerais de conversao

| C# | Rust |
|----|------|
| `int` | `i32` |
| `decimal` | `rust_decimal::Decimal` (ou `f64` se precisao nao for critica) |
| `string` | `String` |
| `string?` | `Option<String>` |
| `DateTime` | `chrono::NaiveDateTime` |
| `DateTime?` | `Option<chrono::NaiveDateTime>` |
| `DateTimeOffset` | `chrono::DateTime<Utc>` |
| `bool` | `bool` |
| `ICollection<T>` | Nao mapear no model (carregar via queries separadas) |
| Navigation Properties | Nao existem no SQLx (joins manuais) |
| Computed Properties | Implementar como metodos `impl Lote { fn viabilidade(&self) -> Decimal }` |
| Data Annotations | `#[validate(...)]` do crate `validator` |

---

## 6. Cargo.toml (Dependencias Rust)

```toml
[package]
name = "granjatech-api"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-cors = "0.7"
actix-rt = "2"

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono", "decimal", "migrate"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Auth
jsonwebtoken = "9"
bcrypt = "0.16"

# Validation
validator = { version = "0.19", features = ["derive"] }

# Dates & Decimals
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = { version = "1", features = ["serde-with-str"] }

# Config
dotenvy = "0.15"
envy = "0.4"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-actix-web = "0.7"

# Cache
moka = { version = "0.12", features = ["future"] }

# OpenAPI docs
utoipa = { version = "5", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "8", features = ["actix-web"] }

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
tokio = { version = "1", features = ["full"] }
```

---

## 7. Fases de Implementacao

### FASE 1 — Fundacao Rust (Semana 1-2)

**Objetivo:** Servidor Rust funcional com auth e CRUD basico.

**Tarefas:**

1. **Scaffold do projeto**
   - `cargo init granjatech-api`
   - Configurar Cargo.toml com todas as dependencias
   - Criar `.env` com DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE

2. **Migrations SQL** (usar `sqlx migrate`)
   - Criar todas as tabelas na mesma ordem do schema atual
   - Seed dos perfis (Administrador, Produtor, Financeiro)
   - Seed do usuario admin padrao

3. **Models (`src/models/`)**
   - Converter todas as 16 entidades C# para structs Rust
   - Implementar `sqlx::FromRow` para cada struct
   - Implementar metodos calculados como `impl` blocks

4. **DTOs (`src/dto/`)**
   - Converter todos os 36 DTOs
   - Adicionar `#[derive(Serialize, Deserialize, Validate)]`

5. **Config e Bootstrap (`src/main.rs`, `src/config.rs`)**
   - Pool de conexoes com SQLx
   - CORS middleware
   - JWT middleware (extractor personalizado)
   - Logging com tracing
   - Swagger UI com utoipa

6. **Auth service + handler**
   - Login (gerar JWT)
   - Registro
   - CRUD de usuarios (admin only)

7. **Granjas handler** (CRUD completo como modelo para os demais)

**Criterio de conclusao:** Login funciona, CRUD de granjas funciona, JWT valida corretamente, Swagger acessivel.

---

### FASE 2 — CRUD Completo do Backend (Semana 3-4)

**Objetivo:** Todos os endpoints implementados em Rust.

**Tarefas:**

1. **Lotes handler** — CRUD + mortalidades
2. **Dashboard handler** — KPIs + resumo mensal
3. **Financas handler** — CRUD transacoes
4. **Consumo handler** — Racao + Agua
5. **Pesagem handler** — CRUD + resumo
6. **Sanitario handler** — CRUD + cronograma vacinacao
7. **Sensores handler** — CRUD + leituras
8. **Estoque handler** — CRUD produtos
9. **Auditoria handler** — Listagem de logs
10. **Profile handler** — Ver/editar perfil, trocar senha

**Criterio de conclusao:** Todos os 60+ endpoints respondem corretamente. Testar com o mesmo Swagger/Postman.

---

### FASE 3 — Relatorios e Logica de Negocio (Semana 5)

**Objetivo:** Portar toda a logica avancada.

**Tarefas:**

1. **Avicultura service** — Metricas, curvas de crescimento, alertas, comparacao com industria, projecao de abate, estimativa de peso, dashboard do lote
2. **Relatorio service** — Financeiro simplificado, financeiro completo, producao, avicultura, desempenho por lote
3. **Relatorio avancado service** — Relatorio geral com filtros
4. **Cache service** — Implementar com moka para endpoints pesados
5. **Auditoria service** — Registrar acoes automaticamente

**Criterio de conclusao:** Respostas identicas ao backend .NET para os mesmos dados de entrada.

---

### FASE 4 — Scaffold Vue + Autenticacao (Semana 6)

**Objetivo:** Projeto Vue funcional com login e navegacao.

**Tarefas:**

1. **Scaffold do projeto**
   - `npm create vue@latest` com TypeScript + Vue Router + Pinia
   - Instalar Vuetify 3, Axios, vue-chartjs, jspdf, xlsx

2. **Vuetify config** (`src/plugins/vuetify.ts`)
   - Migrar tema do MUI (cores, dark mode, fontScale)
   - Configurar icones (mdi)

3. **Auth store** (Pinia) — Substituir AuthContext
   - Estado: token, user, isAuthenticated
   - Actions: login, logout, register
   - Persistencia no localStorage

4. **Accessibility store** — Substituir AccessibilityContext
   - Dark mode toggle
   - Font scale

5. **API service** (`src/services/api.ts`)
   - Axios instance com interceptors (token, 401 redirect)
   - Mesma logica do apiService.js atual

6. **Router** com navigation guards
   - Rotas protegidas (equivalente ao ProtectedRoute)
   - Redirect para /login se nao autenticado

7. **Layout**
   - ResponsiveNavigation.vue (converter drawer + appbar do MUI para Vuetify)
   - PageContainer.vue
   - LoadingSpinner.vue

8. **LoginView.vue** — Converter LoginPage.js

**Criterio de conclusao:** Login funciona contra o backend Rust, navegacao protegida funciona, tema e acessibilidade funcionam.

---

### FASE 5 — Views CRUD (Semana 7-8)

**Objetivo:** Todas as paginas funcionais.

**Ordem de prioridade (por complexidade crescente):**

1. **DashboardView.vue** (176 linhas React) — KPIs + graficos
2. **GranjasView.vue** (312 linhas) — CRUD com dialogs
3. **LotesView.vue** (375 linhas) — CRUD com campos extras
4. **UsuariosView.vue** (335 linhas) — Admin CRUD
5. **FinanceiroView.vue** (362 linhas) — Transacoes + resumo
6. **EstoqueView.vue** (334 linhas) — Produtos CRUD
7. **ProfileView.vue** (132 linhas) — Perfil + troca de senha
8. **AuditoriaView.vue** (136 linhas) — Tabela read-only
9. **SensoresView.vue** (489 linhas) — Sensores + leituras + graficos
10. **ConsumoView.vue** (635 linhas) — Racao/Agua + graficos
11. **PesagemView.vue** (596 linhas) — Pesagens + graficos
12. **SanitarioView.vue** (689 linhas) — Eventos + cronograma
13. **AviculturaView.vue** (479 linhas) — Dashboard de lote
14. **RelatoriosView.vue** (973 linhas) — Relatorios + export PDF/Excel

**Criterio de conclusao:** Todas as 15 views renderizam corretamente, CRUD funciona, graficos exibem dados.

---

### FASE 6 — Docker, CI/CD e Finalizacao (Semana 9)

**Objetivo:** Deploy pronto.

**Tarefas:**

1. **Dockerfile do backend Rust**
   ```dockerfile
   # Build stage
   FROM rust:1.82-bookworm AS builder
   WORKDIR /app
   COPY . .
   RUN cargo build --release
   
   # Runtime stage
   FROM debian:bookworm-slim
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   COPY --from=builder /app/target/release/granjatech-api /usr/local/bin/
   EXPOSE 8080
   CMD ["granjatech-api"]
   ```

2. **Dockerfile do frontend Vue**
   ```dockerfile
   FROM node:20-alpine AS build
   WORKDIR /app
   COPY package*.json ./
   RUN npm ci
   COPY . .
   RUN npm run build
   
   FROM nginx:alpine
   COPY --from=build /app/dist /usr/share/nginx/html
   COPY nginx.conf /etc/nginx/conf.d/default.conf
   ```

3. **docker-compose.yml atualizado**
   - Mesmo PostgreSQL 16
   - Backend Rust no lugar do .NET
   - Frontend Vue no lugar do React

4. **GitHub Actions**
   - Build + test Rust
   - Build Vue
   - Deploy (Azure ou outra plataforma)

5. **Testes de integracao**
   - Comparar respostas do backend Rust vs .NET para mesmos inputs
   - Testar todos os fluxos no frontend

---

## 8. Equivalencias Componentizacao MUI para Vuetify

| MUI (React) | Vuetify (Vue) |
|---|---|
| `<TextField>` | `<v-text-field>` |
| `<Button>` | `<v-btn>` |
| `<Dialog>` | `<v-dialog>` |
| `<Table>` / `<TableContainer>` | `<v-data-table>` |
| `<Card>` | `<v-card>` |
| `<AppBar>` | `<v-app-bar>` |
| `<Drawer>` | `<v-navigation-drawer>` |
| `<Tabs>` / `<Tab>` | `<v-tabs>` / `<v-tab>` |
| `<Select>` | `<v-select>` |
| `<Snackbar>` | `<v-snackbar>` |
| `<CircularProgress>` | `<v-progress-circular>` |
| `<Grid>` | `<v-row>` / `<v-col>` |
| `<Paper>` | `<v-sheet>` |
| `<IconButton>` + MUI Icons | `<v-btn icon>` + mdi icons |
| `<Tooltip>` | `<v-tooltip>` |
| `<Menu>` / `<MenuItem>` | `<v-menu>` / `<v-list-item>` |
| `<Chip>` | `<v-chip>` |
| `<Alert>` | `<v-alert>` |
| `<Switch>` | `<v-switch>` |
| `<Accordion>` | `<v-expansion-panels>` |
| `ThemeProvider` + `createTheme` | `createVuetify({ theme: {...} })` |
| `useContext(AuthContext)` | `useAuthStore()` (Pinia) |
| `useState` / `useEffect` | `ref()` / `onMounted()` / `watch()` |

---

## 9. Riscos e Mitigacoes

| Risco | Impacto | Mitigacao |
|-------|---------|----------|
| SQLx nao tem lazy loading como EF Core | Queries mais verbosas para relacoes | Criar queries com JOINs explicitos, separar em repository functions |
| Rust tem curva de aprendizado (ownership, lifetimes) | Atrasos na implementacao | Comecar pelos handlers CRUD simples, progredir para logica complexa |
| Computed properties do Lote (IEP, CA) dependem de colecoes | Precisam de queries separadas | Implementar como metodos no service que carregam os dados necessarios |
| Vuetify v-data-table tem API diferente do MUI Table | Ajustes na paginacao e filtros | Usar a documentacao do Vuetify como referencia principal |
| Migrations SQLx sao SQL puro (sem rollback automatico) | Risco ao alterar schema | Manter scripts DOWN separados para cada migracao |
| BCrypt hash do .NET pode ter formato levemente diferente | Usuarios existentes nao conseguem logar | Usar mesmo algoritmo BCrypt (crate `bcrypt` em Rust e compativel) |

---

## 10. Estimativa de Esforço

| Fase | Descricao | Duracao | Dev-Hours |
|------|-----------|---------|-----------|
| 1 | Fundacao Rust | 2 semanas | ~60h |
| 2 | CRUD completo backend | 2 semanas | ~60h |
| 3 | Relatorios e logica avancada | 1 semana | ~40h |
| 4 | Scaffold Vue + Auth | 1 semana | ~30h |
| 5 | Views CRUD (15 paginas) | 2 semanas | ~80h |
| 6 | Docker, CI/CD, testes | 1 semana | ~30h |
| **Total** | | **~9 semanas** | **~300h** |

> Estimativa para 1 desenvolvedor com experiencia intermediaria em Rust e Vue. Com time de 2, reduz para ~5-6 semanas.

---

## 11. Checklist Pre-Deploy

- [ ] Todos os 60+ endpoints respondendo no backend Rust
- [ ] JWT com mesma estrutura (claims: id, email, role)
- [ ] Hash BCrypt compativel com senhas existentes
- [ ] Todas as 15 views funcionais no Vue
- [ ] Dark mode e font scale funcionando
- [ ] Graficos (recharts para vue-chartjs) renderizando corretamente
- [ ] Export PDF e Excel funcionando
- [ ] Navigation responsiva (mobile + desktop)
- [ ] Docker Compose levanta os 3 containers sem erros
- [ ] Health check endpoint respondendo
- [ ] Swagger/OpenAPI acessivel
- [ ] CI/CD pipeline passando
- [ ] Dados existentes no PostgreSQL acessiveis sem migracao de banco
