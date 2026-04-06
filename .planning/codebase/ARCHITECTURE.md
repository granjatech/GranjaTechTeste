# Architecture

**Analysis Date:** 2026-04-06

## Pattern Overview

**Overall:** Layered Architecture (Clean Architecture variant) with separate React SPA frontend

**Key Characteristics:**
- Four-project .NET solution following Domain/Application/Infrastructure/API layering
- Separate React frontend communicating via REST API with JWT authentication
- Entity Framework Core with PostgreSQL (code-first migrations)
- Role-based access control (Administrador, Produtor, Financeiro) enforced at service layer
- No repository pattern -- services query `DbContext` directly

## Layers

**API Layer (GranjaTech.Api):**
- Purpose: HTTP entry point, routing, middleware, DI registration, authentication pipeline
- Location: `GranjaTech.Api/`
- Contains: Controllers, `Program.cs` (composition root), Swagger config, static files
- Depends on: GranjaTech.Application (interfaces/DTOs), GranjaTech.Infrastructure (service implementations, DbContext)
- Used by: Frontend (React SPA via HTTP), external HTTP clients

**Application Layer (GranjaTech.Application):**
- Purpose: Service interfaces (contracts) and DTOs -- defines the API boundary
- Location: `GranjaTech.Application/`
- Contains: Service interfaces in `Services/Interfaces/`, DTO classes in `DTOs/`
- Depends on: GranjaTech.Domain (entity types referenced in interfaces)
- Used by: GranjaTech.Api (controller dependencies), GranjaTech.Infrastructure (implementations)

**Infrastructure Layer (GranjaTech.Infrastructure):**
- Purpose: Service implementations, database context, migrations, seed data, caching
- Location: `GranjaTech.Infrastructure/`
- Contains: `GranjaTechDbContext.cs`, EF migrations in `Migrations/`, service implementations in `Services/Implementations/`, cache interface in `Services/Interfaces/ICacheService.cs`, seed data in `Data/`
- Depends on: GranjaTech.Application (implements interfaces), GranjaTech.Domain (entity models)
- Used by: GranjaTech.Api (DI registration)

**Domain Layer (GranjaTech.Domain):**
- Purpose: Entity models (anemic + some business logic), domain extensions
- Location: `GranjaTech.Domain/`
- Contains: Entity classes (e.g., `Usuario.cs`, `Granja.cs`, `Lote.cs`), `Extensions/LoteExtensions.cs`
- Depends on: Nothing (no project references)
- Used by: All other layers

**Frontend Layer (frontend/):**
- Purpose: Single-page application for user interaction
- Location: `frontend/`
- Contains: React components, pages, services, context providers
- Depends on: GranjaTech.Api via HTTP (REST + JWT)
- Used by: End users (browser)

## Data Flow

**Typical API Request (authenticated):**

1. React frontend calls `apiService.getGranjas()` which sends `GET /api/granjas` with JWT Bearer token via axios
2. ASP.NET middleware pipeline: request logging -> CORS -> authentication (JWT validation) -> authorization
3. `GranjasController.GetGranjas()` receives the request, delegates to `IGranjaService.GetAllAsync()`
4. `GranjaService.GetAllAsync()` extracts current user ID/role from `IHttpContextAccessor`, builds role-filtered EF query against `GranjaTechDbContext`
5. Results returned as domain entities through controller -> JSON serialization -> HTTP response
6. Frontend receives JSON, updates component state

**Authentication Flow:**

1. User submits credentials on `LoginPage.js` -> `AuthContext.login()` -> `apiService.login()` -> `POST /api/auth/login`
2. `AuthController.Login()` -> `AuthService.LoginAsync()` verifies BCrypt hash, generates JWT with claims (NameIdentifier, Email, Role)
3. JWT token returned to frontend, stored in `localStorage`, decoded via `jwt-decode` into `AuthContext.user`
4. Subsequent requests attach token via axios request interceptor
5. 401 responses trigger automatic logout via axios response interceptor

**Role-Based Data Filtering:**

1. Every service method calls `GetCurrentUser()` to extract userId and role from JWT claims
2. **Administrador**: sees all data (no filter)
3. **Produtor**: sees only own data (`WHERE UsuarioId == userId`)
4. **Financeiro**: sees data for associated Produtores via `FinanceiroProdutor` junction table

**State Management:**
- Frontend uses React Context API (`AuthContext`, `AccessibilityContext`)
- No Redux or external state management
- JWT token persisted in `localStorage`
- Server-side: no distributed state; in-memory cache via `MemoryCacheService` implementing `ICacheService`

## Key Abstractions

**Service Interfaces (Application Layer):**
- Purpose: Define business operations as async contracts
- Examples: `GranjaTech.Application/Services/Interfaces/IGranjaService.cs`, `GranjaTech.Application/Services/Interfaces/IAuthService.cs`, `GranjaTech.Application/Services/Interfaces/ILoteService.cs`
- Pattern: One interface per domain aggregate, implemented in Infrastructure layer
- Full list: `IGranjaService`, `ILoteService`, `IAuthService`, `IFinancasService`, `IDashboardService`, `IAuditoriaService`, `IEstoqueService`, `ISensorService`, `IRelatorioService`, `IRelatorioAvancadoService`, `IAviculturaService`

**DTOs (Application Layer):**
- Purpose: Data transfer between API and services, input validation
- Examples: `GranjaTech.Application/DTOs/CreateGranjaDto.cs`, `GranjaTech.Application/DTOs/LoginDto.cs`, `GranjaTech.Application/DTOs/DashboardKpiDto.cs`
- Pattern: `Create*Dto` for creation, `Update*Dto` for updates, plain `*Dto` for responses
- Sub-namespace: `GranjaTech.Application/DTOs/Relatorios/` for advanced report DTOs

**Domain Entities:**
- Purpose: Database-mapped models with some computed properties
- Examples: `GranjaTech.Domain/Lote.cs` (rich with computed properties like `Viabilidade`, `CalcularIEP()`), `GranjaTech.Domain/Granja.cs` (anemic)
- Pattern: EF entities with navigation properties, validation attributes on some

**DbContext:**
- Purpose: Single database context for all entities
- Location: `GranjaTech.Infrastructure/GranjaTechDbContext.cs`
- Pattern: Fluent API configuration in `OnModelCreating`, seed data for Perfis and admin user

## Entry Points

**Backend API:**
- Location: `GranjaTech.Api/Program.cs`
- Triggers: `dotnet run` or Docker container startup
- Responsibilities: Configure DI, middleware pipeline, apply migrations on startup, seed dev data, start Kestrel

**Frontend SPA:**
- Location: `frontend/src/index.js` -> `frontend/src/App.js`
- Triggers: `npm start` (dev) or nginx serving built files (prod)
- Responsibilities: React app bootstrap, routing, theme/accessibility providers

**Docker:**
- Backend: `Dockerfile` (multi-stage .NET 8 build)
- Frontend: `frontend/Dockerfile` (separate, served by nginx)
- Orchestration: `docker-compose.yml` (production), `docker-compose.dev.yml` (development with pgAdmin)

## Error Handling

**Strategy:** Exception-based with try/catch in controllers

**Patterns:**
- Controllers wrap service calls in try/catch, return appropriate HTTP status codes (400, 401, 403, 404, 500)
- `InvalidOperationException` used as a domain validation error (e.g., "Financeiro cannot create granjas")
- Global request-tracking middleware in `Program.cs` catches unhandled exceptions, returns JSON error with requestId
- Frontend axios interceptor auto-redirects to `/login` on 401 responses
- No structured error response type -- each controller constructs anonymous `{ message }` objects

## Cross-Cutting Concerns

**Logging:**
- ASP.NET Core `ILogger<T>` via DI
- Custom request-tracking middleware logs every request/response with generated requestId
- Frontend uses `console.log` with emoji prefixes for debug output

**Validation:**
- Data annotations on some DTOs and domain entities (`[Required]`, `[StringLength]`, `[Range]`)
- Manual validation in service methods (e.g., duplicate email checks)
- `[FromQuery, Required]` on controller parameters for report endpoints

**Authentication:**
- JWT Bearer tokens with 8-hour expiration
- BCrypt password hashing (`BCrypt.Net.BCrypt`)
- Claims: `NameIdentifier` (user ID), `Email`, `Role` (profile name)
- Config via `Jwt:Key`, `Jwt:Issuer`, `Jwt:Audience` in appsettings

**Authorization:**
- Role-based via `[Authorize(Roles = "...")]` on controllers
- Data-level filtering in service implementations based on user role
- Three roles seeded: Administrador (ID 1), Produtor (ID 2), Financeiro (ID 3)

**Auditing:**
- `IAuditoriaService` / `AuditoriaService` logs domain events to `LogsAuditoria` table
- Called explicitly after CRUD operations in service methods
- Logs include event type string and descriptive message

---

*Architecture analysis: 2026-04-06*
