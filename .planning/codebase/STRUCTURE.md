# Codebase Structure

**Analysis Date:** 2026-04-06

## Directory Layout

```
GranjaTechTeste/
├── GranjaTech.Api/                # ASP.NET Web API project (composition root)
│   ├── Controllers/               # API controllers (15 controllers)
│   ├── Properties/                # launchSettings.json
│   ├── wwwroot/css/               # Static files (Swagger dark theme CSS)
│   ├── Program.cs                 # Application entry point, DI, middleware
│   └── GranjaTech.Api.csproj      # Project file (.NET 8)
├── GranjaTech.Application/        # Interfaces and DTOs (contracts layer)
│   ├── Services/Interfaces/       # Service interface definitions (11 interfaces)
│   ├── DTOs/                      # Data transfer objects (~35 DTOs)
│   │   └── Relatorios/            # Advanced report-specific DTOs
│   └── GranjaTech.Application.csproj
├── GranjaTech.Domain/             # Domain entity models
│   ├── GranjaTech.Domain/         # Nested directory (contains Produto.cs, Extensions/)
│   │   └── Extensions/            # LoteExtensions.cs
│   ├── *.cs                       # Entity classes (15 entities at root level)
│   └── GranjaTech.Domain.csproj
├── GranjaTech.Infrastructure/     # Service implementations, EF, migrations
│   ├── Services/
│   │   ├── Interfaces/            # Infrastructure-specific interfaces (ICacheService)
│   │   └── Implementations/       # Service implementations (12 services)
│   ├── Data/                      # Seed data (AviculturaSeedData.cs)
│   ├── Migrations/                # EF Core migrations (6 migrations)
│   ├── GranjaTechDbContext.cs     # Database context (17 DbSets)
│   └── GranjaTech.Infrastructure.csproj
├── frontend/                      # React SPA (Create React App)
│   ├── src/
│   │   ├── pages/                 # Page components (15 pages)
│   │   ├── components/            # Shared components (5 files)
│   │   ├── services/              # API client (apiService.js, relatoriosApi.js)
│   │   ├── context/               # React contexts (AuthContext.js, AccessibilityContext.js)
│   │   ├── App.js                 # Root component with routing
│   │   ├── theme.js               # MUI theme configuration
│   │   └── index.js               # React entry point
│   ├── public/                    # Static assets
│   ├── Dockerfile                 # Frontend Docker build (nginx)
│   ├── nginx.conf                 # Nginx config for SPA routing
│   └── package.json               # NPM dependencies
├── docs/                          # SQL scripts (banco.sql, schema.sql)
├── Documentação/                  # Project documentation
│   └── diagrama e sql/            # Database diagrams and SQL
├── .github/workflows/             # CI/CD workflow definitions
├── GranjaTech.sln                 # Visual Studio solution file
├── Dockerfile                     # Backend Docker build (multi-stage .NET 8)
├── docker-compose.yml             # Production Docker Compose
├── docker-compose.dev.yml         # Development Docker Compose (includes pgAdmin)
├── .env.example                   # Environment variable template
├── .gitignore                     # Git ignore rules
└── README.md                      # Project documentation
```

## Directory Purposes

**GranjaTech.Api/Controllers/:**
- Purpose: HTTP endpoint handlers
- Contains: 15 controller classes, one per domain area
- Key files: `AuthController.cs` (auth endpoints), `GranjasController.cs`, `LotesController.cs`, `DashboardController.cs`, `RelatoriosController.cs` (most complex, ~570 lines with debug endpoints)

**GranjaTech.Application/Services/Interfaces/:**
- Purpose: Service contracts that define business operations
- Contains: 11 interface files
- Key files: `IGranjaService.cs`, `IAuthService.cs`, `IRelatorioAvancadoService.cs`, `IAviculturaService.cs`

**GranjaTech.Application/DTOs/:**
- Purpose: Request/response data shapes for API communication
- Contains: ~35 DTO classes + 3 in `Relatorios/` subdirectory
- Key files: `LoginDto.cs`, `LoginResponseDto.cs`, `CreateGranjaDto.cs`, `DashboardKpiDto.cs`
- Naming pattern: `Create*Dto` for creation, `Update*Dto` for updates, `*DetailDto` for detailed responses

**GranjaTech.Domain/:**
- Purpose: Core domain entities mapped to database tables
- Contains: 15 entity classes + extensions
- Key files: `Lote.cs` (richest entity with computed properties and business methods), `Usuario.cs`, `Granja.cs`, `TransacaoFinanceira.cs`
- Note: `Produto.cs` lives in nested `GranjaTech.Domain/GranjaTech.Domain/` subdirectory (structural oddity)

**GranjaTech.Infrastructure/Services/Implementations/:**
- Purpose: Business logic implementations that query the database
- Contains: 12 service classes (11 domain + 1 cache)
- Key files: `AuthService.cs` (JWT generation, BCrypt), `GranjaService.cs` (role-based filtering pattern), `RelatorioAvancadoService.cs` (complex queries), `AviculturaService.cs`

**frontend/src/pages/:**
- Purpose: Top-level page components, one per route
- Contains: 15 page components
- Key files: `DashboardPage.js`, `LoginPage.js`, `RelatoriosPage.js`, `AviculturaPage.js`

**frontend/src/services/:**
- Purpose: HTTP API client layer
- Contains: `apiService.js` (main API client with axios), `relatoriosApi.js` (report-specific endpoints)
- Key files: `apiService.js` (centralized API methods, request/response interceptors, auto-logout on 401)

**frontend/src/context/:**
- Purpose: React context providers for global state
- Contains: `AuthContext.js` (JWT auth state, login/logout), `AccessibilityContext.js` (theme mode, font scale)

## Key File Locations

**Entry Points:**
- `GranjaTech.Api/Program.cs`: Backend composition root (DI, middleware, startup, auto-migration)
- `frontend/src/index.js`: React app bootstrap
- `frontend/src/App.js`: Root component with all route definitions

**Configuration:**
- `GranjaTech.Api/GranjaTech.Api.csproj`: Backend project dependencies (.NET 8, EF Core 9, JWT)
- `frontend/package.json`: Frontend dependencies (React 19, MUI 7, axios)
- `GranjaTech.sln`: Solution structure (4 projects)
- `.env.example`: Required environment variables template
- `docker-compose.yml`: Production container orchestration
- `docker-compose.dev.yml`: Development container orchestration (adds pgAdmin)

**Core Backend Logic:**
- `GranjaTech.Infrastructure/GranjaTechDbContext.cs`: Database schema, relationships, seeds (Perfis + admin user)
- `GranjaTech.Infrastructure/Services/Implementations/AuthService.cs`: Authentication, JWT, user CRUD
- `GranjaTech.Infrastructure/Services/Implementations/GranjaService.cs`: Granja CRUD with role filtering
- `GranjaTech.Infrastructure/Services/Implementations/RelatorioAvancadoService.cs`: Complex report queries
- `GranjaTech.Domain/Lote.cs`: Core domain entity with business calculations (CA, IEP)

**Core Frontend Logic:**
- `frontend/src/services/apiService.js`: Centralized API client with all endpoint methods
- `frontend/src/context/AuthContext.js`: Authentication state management
- `frontend/src/components/ProtectedRoute.js`: Route guard component
- `frontend/src/components/ResponsiveNavigation.js`: Navigation bar/sidebar
- `frontend/src/theme.js`: MUI theme with dark/light mode and accessibility

**Testing:**
- `frontend/src/App.test.js`: Single React test file
- `frontend/src/setupTests.js`: Test setup (jest-dom)
- No backend test project exists

## Naming Conventions

**Files (Backend C#):**
- Controllers: `{DomainArea}Controller.cs` (e.g., `GranjasController.cs`, `AuthController.cs`)
- Service interfaces: `I{DomainArea}Service.cs` (e.g., `IGranjaService.cs`)
- Service implementations: `{DomainArea}Service.cs` (e.g., `GranjaService.cs`)
- DTOs: `{Action}{Entity}Dto.cs` (e.g., `CreateGranjaDto.cs`, `UpdateLoteDto.cs`, `LoginResponseDto.cs`)
- Domain entities: `{EntityName}.cs` in PascalCase (e.g., `TransacaoFinanceira.cs`, `LeituraSensor.cs`)
- All entity and DTO names use Portuguese (e.g., `Usuario`, `Granja`, `Lote`, `TransacaoFinanceira`)

**Files (Frontend JavaScript):**
- Pages: `{DomainArea}Page.js` (e.g., `DashboardPage.js`, `GranjasPage.js`)
- Components: `{ComponentName}.js` in PascalCase (e.g., `ProtectedRoute.js`, `LoadingSpinner.js`)
- Services: `{name}Service.js` or `{name}Api.js` in camelCase (e.g., `apiService.js`, `relatoriosApi.js`)
- Contexts: `{Name}Context.js` in PascalCase (e.g., `AuthContext.js`)

**Directories:**
- Backend: PascalCase for project directories (`GranjaTech.Api`, `Services`, `Controllers`)
- Frontend: lowercase for directories (`pages`, `components`, `services`, `context`)

**API Routes:**
- Pattern: `/api/{controller}` using `[Route("api/[controller]")]`
- Portuguese names used consistently (e.g., `/api/granjas`, `/api/lotes`, `/api/financas`, `/api/relatorios`)

## Where to Add New Code

**New Backend Feature (e.g., new domain area "Vacinas"):**
1. Domain entity: `GranjaTech.Domain/Vacina.cs`
2. Service interface: `GranjaTech.Application/Services/Interfaces/IVacinaService.cs`
3. DTOs: `GranjaTech.Application/DTOs/CreateVacinaDto.cs`, `GranjaTech.Application/DTOs/UpdateVacinaDto.cs`
4. Service implementation: `GranjaTech.Infrastructure/Services/Implementations/VacinaService.cs`
5. DbSet + config in: `GranjaTech.Infrastructure/GranjaTechDbContext.cs`
6. Controller: `GranjaTech.Api/Controllers/VacinasController.cs`
7. DI registration: `GranjaTech.Api/Program.cs` (add `builder.Services.AddScoped<IVacinaService, VacinaService>()`)
8. Migration: `dotnet ef migrations add AddVacina -p GranjaTech.Infrastructure -s GranjaTech.Api`

**New Frontend Page:**
1. Page component: `frontend/src/pages/{Feature}Page.js`
2. API methods: Add to `frontend/src/services/apiService.js` in the `apiService` object
3. Route: Add to `frontend/src/App.js` inside `<Routes>` wrapped in `<ProtectedRoute>`
4. Navigation link: Add to `frontend/src/components/ResponsiveNavigation.js`

**New DTO:**
- Standard DTOs: `GranjaTech.Application/DTOs/`
- Report DTOs: `GranjaTech.Application/DTOs/Relatorios/`

**New EF Migration:**
- Run from solution root: `dotnet ef migrations add {MigrationName} -p GranjaTech.Infrastructure -s GranjaTech.Api`
- Migrations auto-apply on every app startup via `db.Database.Migrate()` in `Program.cs`

**Shared Utilities:**
- Backend domain extensions: `GranjaTech.Domain/GranjaTech.Domain/Extensions/`
- Frontend shared components: `frontend/src/components/`
- Frontend shared contexts: `frontend/src/context/`

## Special Directories

**GranjaTech.Infrastructure/Migrations/:**
- Purpose: EF Core database migration files
- Generated: Yes (via `dotnet ef migrations add`)
- Committed: Yes
- Note: Auto-applied on every app startup via `db.Database.Migrate()` in `Program.cs`

**GranjaTech.Domain/GranjaTech.Domain/:**
- Purpose: Nested directory containing `Produto.cs` and `Extensions/LoteExtensions.cs`
- Generated: No (appears to be accidental nesting from project restructuring)
- Committed: Yes
- Note: Structural oddity -- most domain files are at root, some in nested subdirectory

**frontend/public/:**
- Purpose: Static assets served directly by web server
- Generated: No
- Committed: Yes

**GranjaTech.Api/wwwroot/:**
- Purpose: Static files served by ASP.NET (contains Swagger dark theme CSS)
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-04-06*
