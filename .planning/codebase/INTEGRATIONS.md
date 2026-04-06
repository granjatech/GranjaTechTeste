# External Integrations

**Analysis Date:** 2026-04-06

## APIs & External Services

**No third-party external APIs are consumed.** The application is self-contained with its own backend API.

**Internal API:**
- GranjaTech REST API - Backend serves frontend via HTTP
  - Base URL (dev): `http://localhost:5099/api`
  - Base URL (docker prod): `http://localhost:5099/api` (mapped from container port 8080)
  - Client: Axios 1.11.0 via `frontend/src/services/apiService.js`
  - Auth: JWT Bearer token in `Authorization` header
  - Timeout: 30 seconds

## Data Storage

**Database:**
- PostgreSQL 16 Alpine
  - Connection var: `ConnectionStrings__DefaultConnection`
  - Default DB name: `GranjaTechDb`
  - ORM: Entity Framework Core 9.0.8 with Npgsql provider
  - DbContext: `GranjaTech.Infrastructure/GranjaTechDbContext.cs`
  - Migrations: `GranjaTech.Infrastructure/Migrations/` (auto-applied on startup)
  - SQL seed scripts: `docs/banco.sql`, `docs/schema.sql`, `docs/mortalidade update.sql`
  - Docker volume: `granjatech-postgres-data`

**Database Entities (17 tables):**
- `Usuarios` - User accounts
- `Perfis` - User profiles/roles (Admin, Produtor, Financeiro)
- `FinanceiroProdutor` - Many-to-many user association
- `Granjas` - Farm records
- `Lotes` - Batch/lot records
- `TransacoesFinanceiras` - Financial transactions
- `LogsAuditoria` - Audit logs
- `Produtos` - Inventory products
- `Sensores` - IoT sensors
- `LeiturasSensores` - Sensor readings
- `ConsumosRacao` - Feed consumption
- `ConsumosAgua` - Water consumption
- `PesagensSemanais` - Weekly weighings
- `EventosSanitarios` - Sanitary events
- `MedicoesQualidadeAr` - Air quality measurements
- `RegistrosMortalidade` - Mortality records
- `RegistrosAbate` - Slaughter records

**File Storage:**
- Not used. No file upload/download functionality detected.

**Caching:**
- In-memory cache via `Microsoft.Extensions.Caching.Memory`
  - Interface: `GranjaTech.Infrastructure/Services/Interfaces/ICacheService.cs`
  - Implementation: `GranjaTech.Infrastructure/Services/Implementations/MemoryCacheService.cs`
  - Registered as scoped DI in `GranjaTech.Api/Program.cs`
  - No distributed cache (Redis, etc.)

## Authentication & Identity

**Auth Provider:** Custom JWT implementation (no external identity provider)

**Backend Auth:**
- Password hashing: BCrypt via `BCrypt.Net-Next` 4.0.3
- Token generation: `System.IdentityModel.Tokens.Jwt` in `GranjaTech.Infrastructure/Services/Implementations/AuthService.cs`
- Algorithm: HMAC-SHA256
- Token expiry: 8 hours
- Claims: `NameIdentifier` (user ID), `Email`, `Role` (profile name)
- Validation: `Microsoft.AspNetCore.Authentication.JwtBearer` middleware

**Frontend Auth:**
- Token storage: `localStorage` (key: `token`)
- Auth context: `frontend/src/context/AuthContext.js`
- Protected routes: `frontend/src/components/ProtectedRoute.js`
- Auto-redirect on 401: Axios response interceptor clears token and redirects to `/login`
- JWT decoding: `jwt-decode` library

**JWT Configuration (env vars):**
- `Jwt__Key` - Signing key (minimum 32 characters recommended)
- `Jwt__Issuer` - Token issuer (default: `GranjaTechAPI`)
- `Jwt__Audience` - Token audience (default: `GranjaTechApp`)

**Role-Based Access:**
- Three profiles seeded: Administrador (ID 1), Produtor (ID 2), Financeiro (ID 3)
- Role claims embedded in JWT token
- Default admin: `admin@admin.com` (seeded in DbContext)

## Monitoring & Observability

**Error Tracking:**
- No external error tracking service (no Sentry, Application Insights, etc.)
- Custom request tracking middleware in `GranjaTech.Api/Program.cs` (generates request IDs, logs start/end/errors)

**Logs:**
- Built-in ASP.NET Core `ILogger` / `ILoggerFactory`
- Default level: Information (Debug in Development for EF Core)
- Configuration: `GranjaTech.Api/appsettings.json` and `GranjaTech.Api/appsettings.Development.json`
- Frontend: `console.log`/`console.error` with emoji prefixes in `frontend/src/services/apiService.js`

**Health Checks:**
- Backend: `GET /health` endpoint (returns 200 OK, anonymous access)
- Frontend Nginx: `GET /health` returns "healthy" (in `frontend/nginx.conf`)
- Docker healthchecks configured for all three services (postgres, backend, frontend)

## CI/CD & Deployment

**Hosting:**
- Docker containers (no specific cloud platform configured in repo)
- Production compose: `docker-compose.yml` (postgres + backend + frontend)
- Development compose: `docker-compose.dev.yml` (postgres + pgAdmin only)

**CI Pipeline:**
- No CI/CD configuration files detected in the current repo state.

**Development Tools:**
- pgAdmin 4 available in dev compose on port 5050

## Environment Configuration

**Required env vars (production):**
- `ConnectionStrings__DefaultConnection` - PostgreSQL connection string
- `Jwt__Key` - JWT signing secret key
- `Jwt__Issuer` - JWT issuer identifier
- `Jwt__Audience` - JWT audience identifier
- `AllowedOrigins` - CORS allowed origins (semicolon-separated)
- `REACT_APP_API_URL` - Frontend API base URL (build-time)
- `ASPNETCORE_ENVIRONMENT` - Runtime environment (Production/Development)

**Optional env vars:**
- `Swagger__Enabled` - Enable Swagger UI in production (default: false)
- `PGADMIN_DEFAULT_EMAIL` / `PGADMIN_DEFAULT_PASSWORD` - pgAdmin credentials (dev only)

**Secrets location:**
- `.env.example` documents all variables (template only, no real secrets)
- `.env` file expected but gitignored
- Docker Compose files contain hardcoded dev credentials (not for production use)
- No secrets manager integration (docs recommend Azure Key Vault or AWS Secrets Manager)

## Webhooks & Callbacks

**Incoming:**
- None detected

**Outgoing:**
- None detected

## API Documentation

**Swagger/OpenAPI:**
- Swashbuckle.AspNetCore 6.6.2
- Available at `/swagger` (auto-redirect from root `/`)
- Enabled by default in Development; toggleable in Production via `Swagger:Enabled` config
- JWT Bearer auth configured in Swagger UI
- Custom dark theme CSS: `/css/swagger-dark.css`

## Report Generation (Client-Side)

**PDF Export:**
- `jspdf` 3.0.1 + `jspdf-autotable` 5.0.2 - PDF generation in browser
- Service: `frontend/src/services/relatoriosApi.js`

**Excel Export:**
- `xlsx` 0.18.5 - Spreadsheet generation in browser

---

*Integration audit: 2026-04-06*
