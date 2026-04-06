<!-- GSD:project-start source:PROJECT.md -->
## Project

**GranjaTech — Migração .NET 8 + React para Rust + Vue.js**

Sistema de gestão avícola (GranjaTech) sendo migrado de .NET 8 (C#) + React 19 para Rust (Actix-web + SQLx) + Vue 3 (Vuetify 3). O sistema gerencia granjas, lotes de aves, sensores, finanças, estoque, consumo, pesagem, eventos sanitários, relatórios e auditoria. A migração deve manter paridade total de funcionalidade com o sistema atual.

**Core Value:** Toda funcionalidade que existe hoje no .NET/React deve funcionar de forma idêntica no Rust/Vue — paridade total é inegociável.

### Constraints

- **Stack alvo**: Rust (Actix-web 4, SQLx, jsonwebtoken, bcrypt, utoipa, moka, tracing) + Vue 3 (Vuetify 3, Pinia, Vue Router 4, TypeScript, Vite)
- **Banco**: PostgreSQL 16 existente — SQLx conecta nas mesmas tabelas, sem migrations destrutivas
- **Compatibilidade**: BCrypt hashes devem ser compatíveis entre .NET e Rust
- **API Contract**: Mesmos endpoints, mesmas rotas, mesmos payloads JSON
- **Deploy**: Docker Compose local apenas
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- C# (.NET 8) - Backend API, domain logic, infrastructure, data access
- JavaScript (ES6+) - Frontend React application
- SQL - Database schema and seed scripts in `docs/`
## Runtime
- .NET 8.0 (ASP.NET Core Web API)
- Target framework: `net8.0` (all four projects)
- Node.js 20 (build-time, via `node:20-alpine` Docker image)
- Nginx Alpine (production serving)
- NuGet (backend, implicit via `dotnet restore`)
- npm (frontend)
- Lockfile: `frontend/package-lock.json` present
## Frameworks
- ASP.NET Core 8.0 - Web API framework (`GranjaTech.Api/GranjaTech.Api.csproj`)
- React 19.1.1 - Frontend SPA (`frontend/package.json`)
- React Router DOM 7.8.2 - Client-side routing (`frontend/package.json`)
- MUI (Material UI) 7.3.1 - Component library (`@mui/material`, `@mui/icons-material`)
- Emotion 11.14.x - CSS-in-JS styling (`@emotion/react`, `@emotion/styled`)
- Recharts 3.1.2 - Charts and graphs (`frontend/package.json`)
- Jest (via react-scripts) - Frontend test runner
- Testing Library (React 16.3.0, DOM 10.4.1, jest-dom 6.7.0) - Frontend testing utilities
- react-scripts 5.0.1 - Frontend build toolchain (Create React App)
- Docker multi-stage builds - Both backend and frontend
- docker-compose - Orchestration
## Key Dependencies
- `Microsoft.EntityFrameworkCore` 9.0.8 - ORM for PostgreSQL (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `Npgsql.EntityFrameworkCore.PostgreSQL` 9.0.4 - PostgreSQL EF Core provider (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)
- `Microsoft.AspNetCore.Authentication.JwtBearer` 8.0.8 - JWT authentication (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `System.IdentityModel.Tokens.Jwt` 7.7.0 - JWT token generation (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)
- `BCrypt.Net-Next` 4.0.3 - Password hashing (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)
- `Swashbuckle.AspNetCore` 6.6.2 - Swagger/OpenAPI documentation (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `Microsoft.EntityFrameworkCore.Design` 9.0.8 - EF Core migrations tooling
- `Microsoft.EntityFrameworkCore.Tools` 9.0.8 - EF Core CLI tools
- `axios` 1.11.0 - HTTP client for API communication (`frontend/package.json`)
- `jwt-decode` 4.0.0 - JWT token decoding on client side
- `jspdf` 3.0.1 + `jspdf-autotable` 5.0.2 - PDF report generation
- `xlsx` 0.18.5 - Excel spreadsheet export
- `web-vitals` 2.1.4 - Performance monitoring
## Configuration
- `GranjaTech.Api/appsettings.json` - Base configuration (connection string, JWT settings)
- `GranjaTech.Api/appsettings.Development.json` - Development overrides (verbose logging)
- `GranjaTech.Api/Properties/launchSettings.json` - Development server profiles (ports 5099 HTTP, 7135 HTTPS)
- Environment variables override appsettings via `__` notation (e.g., `ConnectionStrings__DefaultConnection`)
- `REACT_APP_API_URL` environment variable - API base URL (defaults to `https://localhost:7135/api`)
- ESLint config inline in `frontend/package.json` (extends `react-app`, `react-app/jest`)
- `ConnectionStrings:DefaultConnection` - PostgreSQL connection
- `Jwt:Key` / `Jwt:Issuer` / `Jwt:Audience` - JWT token configuration
- `AllowedOrigins` - CORS allowed origins (semicolon-separated)
- `Swagger:Enabled` - Toggle Swagger in production
- `.env.example` - Documents all required environment variables
## Build Configuration
- Solution file: `GranjaTech.sln`
- Multi-stage Docker build: `Dockerfile` (SDK 8.0 build, ASP.NET 8.0 runtime)
- Exposes port 8080 in container
- Auto-applies EF Core migrations on startup (`db.Database.Migrate()`)
- Create React App toolchain via `react-scripts`
- Multi-stage Docker build: `frontend/Dockerfile` (Node 20 build, Nginx serve)
- Nginx config: `frontend/nginx.conf` (gzip, security headers, SPA routing, static asset caching)
- Exposes port 80 in container
## Platform Requirements
- .NET 8 SDK
- Node.js 20+
- PostgreSQL 16 (or via `docker-compose.dev.yml`)
- pgAdmin 4 available via dev compose on port 5050
- Docker + Docker Compose
- PostgreSQL 16 Alpine container
- Backend on port 5099 (mapped to container 8080)
- Frontend on port 3000 (mapped to container 80)
- `GranjaTech.Api` - Web API entry point, controllers, DI configuration
- `GranjaTech.Application` - Service interfaces, DTOs
- `GranjaTech.Domain` - Domain entities (no dependencies)
- `GranjaTech.Infrastructure` - EF Core DbContext, service implementations, migrations
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Languages
## Naming Patterns
- Domain entities: PascalCase singular nouns in Portuguese - `Granja.cs`, `Lote.cs`, `Usuario.cs`, `TransacaoFinanceira.cs`
- DTOs: PascalCase with prefix `Create`/`Update` + entity + `Dto` - `CreateGranjaDto.cs`, `UpdateLoteDto.cs`, `LoginResponseDto.cs`
- Service interfaces: `I` + entity + `Service` - `IGranjaService.cs`, `IAuthService.cs`
- Service implementations: entity + `Service` - `GranjaService.cs`, `AuthService.cs`
- Controllers: entity (pluralized) + `Controller` - `GranjasController.cs`, `LotesController.cs`
- DbContext: `GranjaTechDbContext.cs`
- Pages: PascalCase + `Page` suffix - `DashboardPage.js`, `LoginPage.js`, `GranjasPage.js`
- Components: PascalCase descriptive - `ProtectedRoute.js`, `ResponsiveNavigation.js`, `PageContainer.js`, `LoadingSpinner.js`
- Contexts: PascalCase + `Context` suffix - `AuthContext.js`, `AccessibilityContext.js`
- Services: camelCase + descriptive - `apiService.js`, `relatoriosApi.js`
- Service methods: async with `Async` suffix - `GetAllAsync()`, `AddAsync()`, `UpdateAsync()`, `DeleteAsync()`
- Private helpers: camelCase-style prefixed verb - `GetCurrentUser()`, `GetCurrentUserId()`, `GenerateJwtToken()`
- Audit logging: `RegistrarLog(string acao, string detalhes)` (Portuguese)
- Private fields: underscore-prefixed camelCase - `_context`, `_authService`, `_httpContextAccessor`
- Local variables: camelCase in Portuguese - `granjaExistente`, `novaGranja`, `novoCodigo`, `senhaHash`
- Tuples for user context: `var (userId, userRole) = GetCurrentUser();`
- React components: PascalCase function declarations - `function DashboardPage() {}`
- Event handlers: `handle` prefix + verb - `handleSubmit`
- Data fetchers: `fetch` prefix - `fetchData`
- API service methods: camelCase verb + entity - `getGranjas()`, `createLote()`, `deleteTransacao()`
- State: camelCase descriptive - `const [kpis, setKpis] = useState(null)`
- Constants: camelCase for objects, UPPER_SNAKE not used
- Interfaces: `I` prefix with PascalCase - `IGranjaService`, `IAuthService`, `ICacheService`
- Enums: Not used; string literals instead (e.g., `"Administrador"`, `"Produtor"`, `"Financeiro"` for roles)
## Code Style
- No explicit formatter config (no `.editorconfig` detected)
- 4-space indentation
- Opening braces on same line for single-line blocks, new line for multi-line
- Nullable reference types enabled (`<Nullable>enable</Nullable>`)
- Implicit usings enabled
- No Prettier or ESLint config files; relies on default `react-app` ESLint config in `package.json`
- 4-space indentation in JSX
- Single quotes for JS strings
- Template literals for dynamic strings
- Backend: No analyzers configured
- Frontend: `react-app` and `react-app/jest` ESLint presets only (defined in `frontend/package.json` `eslintConfig`)
## Import Organization
- None configured. All imports use relative paths (`../services/apiService`, `../context/AuthContext`)
## Error Handling
## Logging
- Structured logging with named parameters: `_logger.LogInformation("Retornando relatorio com {TransacoesCount} transacoes", count)`
- Error logging with exception: `_logger.LogError(ex, "Erro ao gerar relatorio")`
- Request tracking middleware generates `requestId` for each request
- Emoji-prefixed console logs in `apiService.js` for debug purposes (should be removed in production)
- Request/response interceptors log full details including headers and data
## Comments
- XML doc comments (`///`) used on domain entity properties in `GranjaTech.Domain/Lote.cs` for complex business logic
- Inline comments in Portuguese for section headers using `// ========` separators in `Program.cs`
- Code left with change notes (e.g., `// ADICIONE ESTA LINHA`, `// Assinatura alterada`) - these should be cleaned up
- Minimal comments in React components
- Portuguese inline comments for business context
## Controller Design
- Class-level `[Authorize]` for controllers requiring any authenticated user (`GranjasController`)
- Method-level `[Authorize(Roles = "...")]` for role-specific endpoints (`AuthController`, `FinancasController`)
- `[AllowAnonymous]` for public endpoints (login, register, health checks)
- Success: `return Ok(data)` or `return Ok(new { message = "..." })`
- Not found: `return NotFound()` or `return NotFound("message")`
- Validation error: `return BadRequest(new { message = "..." })`
- Auth failure: `return Forbid(ex.Message)` or `return Unauthorized(new { message = "..." })`
- Create success: `return Ok(new { message = "..." })` (not 201 Created)
- Update/Delete success: `return NoContent()`
## Service Design
- `GranjaTechDbContext _context` - database access
- `IHttpContextAccessor _httpContextAccessor` - current user context
- `IAuditoriaService _auditoriaService` - audit logging
- `Task<IEnumerable<T>> GetAllAsync()` - filtered by user role
- `Task<T?> GetByIdAsync(int id)` - with role-based access check
- `Task AddAsync(CreateDto dto)` - maps DTO to entity, saves, logs audit
- `Task<bool> UpdateAsync(int id, UpdateDto dto)` - returns false if not found
- `Task DeleteAsync(int id)` - void or bool return
## Frontend Component Design
- Wrap content in `<PageContainer title="..." subtitle="...">` for consistent layout
- Use `<LoadingSpinner message="..." />` during data fetching
- State management via `useState` + `useEffect` with `useCallback` for fetch functions
- All API calls go through `apiService` singleton from `frontend/src/services/apiService.js`
- `AuthContext` provides `{ token, user, login, logout }`
- `ProtectedRoute` wraps pages requiring authentication
- Token stored in `localStorage`
- Role-based navigation filtering in `ResponsiveNavigation`
- MUI v7 with custom theme factory in `frontend/src/theme.js`
- Dark/light mode via `AccessibilityContext`
- Font scaling support
- Green primary color (`#2E7D32`), orange secondary (`#FF6F00`)
## Module Design
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Four-project .NET solution following Domain/Application/Infrastructure/API layering
- Separate React frontend communicating via REST API with JWT authentication
- Entity Framework Core with PostgreSQL (code-first migrations)
- Role-based access control (Administrador, Produtor, Financeiro) enforced at service layer
- No repository pattern -- services query `DbContext` directly
## Layers
- Purpose: HTTP entry point, routing, middleware, DI registration, authentication pipeline
- Location: `GranjaTech.Api/`
- Contains: Controllers, `Program.cs` (composition root), Swagger config, static files
- Depends on: GranjaTech.Application (interfaces/DTOs), GranjaTech.Infrastructure (service implementations, DbContext)
- Used by: Frontend (React SPA via HTTP), external HTTP clients
- Purpose: Service interfaces (contracts) and DTOs -- defines the API boundary
- Location: `GranjaTech.Application/`
- Contains: Service interfaces in `Services/Interfaces/`, DTO classes in `DTOs/`
- Depends on: GranjaTech.Domain (entity types referenced in interfaces)
- Used by: GranjaTech.Api (controller dependencies), GranjaTech.Infrastructure (implementations)
- Purpose: Service implementations, database context, migrations, seed data, caching
- Location: `GranjaTech.Infrastructure/`
- Contains: `GranjaTechDbContext.cs`, EF migrations in `Migrations/`, service implementations in `Services/Implementations/`, cache interface in `Services/Interfaces/ICacheService.cs`, seed data in `Data/`
- Depends on: GranjaTech.Application (implements interfaces), GranjaTech.Domain (entity models)
- Used by: GranjaTech.Api (DI registration)
- Purpose: Entity models (anemic + some business logic), domain extensions
- Location: `GranjaTech.Domain/`
- Contains: Entity classes (e.g., `Usuario.cs`, `Granja.cs`, `Lote.cs`), `Extensions/LoteExtensions.cs`
- Depends on: Nothing (no project references)
- Used by: All other layers
- Purpose: Single-page application for user interaction
- Location: `frontend/`
- Contains: React components, pages, services, context providers
- Depends on: GranjaTech.Api via HTTP (REST + JWT)
- Used by: End users (browser)
## Data Flow
- Frontend uses React Context API (`AuthContext`, `AccessibilityContext`)
- No Redux or external state management
- JWT token persisted in `localStorage`
- Server-side: no distributed state; in-memory cache via `MemoryCacheService` implementing `ICacheService`
## Key Abstractions
- Purpose: Define business operations as async contracts
- Examples: `GranjaTech.Application/Services/Interfaces/IGranjaService.cs`, `GranjaTech.Application/Services/Interfaces/IAuthService.cs`, `GranjaTech.Application/Services/Interfaces/ILoteService.cs`
- Pattern: One interface per domain aggregate, implemented in Infrastructure layer
- Full list: `IGranjaService`, `ILoteService`, `IAuthService`, `IFinancasService`, `IDashboardService`, `IAuditoriaService`, `IEstoqueService`, `ISensorService`, `IRelatorioService`, `IRelatorioAvancadoService`, `IAviculturaService`
- Purpose: Data transfer between API and services, input validation
- Examples: `GranjaTech.Application/DTOs/CreateGranjaDto.cs`, `GranjaTech.Application/DTOs/LoginDto.cs`, `GranjaTech.Application/DTOs/DashboardKpiDto.cs`
- Pattern: `Create*Dto` for creation, `Update*Dto` for updates, plain `*Dto` for responses
- Sub-namespace: `GranjaTech.Application/DTOs/Relatorios/` for advanced report DTOs
- Purpose: Database-mapped models with some computed properties
- Examples: `GranjaTech.Domain/Lote.cs` (rich with computed properties like `Viabilidade`, `CalcularIEP()`), `GranjaTech.Domain/Granja.cs` (anemic)
- Pattern: EF entities with navigation properties, validation attributes on some
- Purpose: Single database context for all entities
- Location: `GranjaTech.Infrastructure/GranjaTechDbContext.cs`
- Pattern: Fluent API configuration in `OnModelCreating`, seed data for Perfis and admin user
## Entry Points
- Location: `GranjaTech.Api/Program.cs`
- Triggers: `dotnet run` or Docker container startup
- Responsibilities: Configure DI, middleware pipeline, apply migrations on startup, seed dev data, start Kestrel
- Location: `frontend/src/index.js` -> `frontend/src/App.js`
- Triggers: `npm start` (dev) or nginx serving built files (prod)
- Responsibilities: React app bootstrap, routing, theme/accessibility providers
- Backend: `Dockerfile` (multi-stage .NET 8 build)
- Frontend: `frontend/Dockerfile` (separate, served by nginx)
- Orchestration: `docker-compose.yml` (production), `docker-compose.dev.yml` (development with pgAdmin)
## Error Handling
- Controllers wrap service calls in try/catch, return appropriate HTTP status codes (400, 401, 403, 404, 500)
- `InvalidOperationException` used as a domain validation error (e.g., "Financeiro cannot create granjas")
- Global request-tracking middleware in `Program.cs` catches unhandled exceptions, returns JSON error with requestId
- Frontend axios interceptor auto-redirects to `/login` on 401 responses
- No structured error response type -- each controller constructs anonymous `{ message }` objects
## Cross-Cutting Concerns
- ASP.NET Core `ILogger<T>` via DI
- Custom request-tracking middleware logs every request/response with generated requestId
- Frontend uses `console.log` with emoji prefixes for debug output
- Data annotations on some DTOs and domain entities (`[Required]`, `[StringLength]`, `[Range]`)
- Manual validation in service methods (e.g., duplicate email checks)
- `[FromQuery, Required]` on controller parameters for report endpoints
- JWT Bearer tokens with 8-hour expiration
- BCrypt password hashing (`BCrypt.Net.BCrypt`)
- Claims: `NameIdentifier` (user ID), `Email`, `Role` (profile name)
- Config via `Jwt:Key`, `Jwt:Issuer`, `Jwt:Audience` in appsettings
- Role-based via `[Authorize(Roles = "...")]` on controllers
- Data-level filtering in service implementations based on user role
- Three roles seeded: Administrador (ID 1), Produtor (ID 2), Financeiro (ID 3)
- `IAuditoriaService` / `AuditoriaService` logs domain events to `LogsAuditoria` table
- Called explicitly after CRUD operations in service methods
- Logs include event type string and descriptive message
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, or `.github/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
