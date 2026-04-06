# Technology Stack

**Analysis Date:** 2026-04-06

## Languages

**Primary:**
- C# (.NET 8) - Backend API, domain logic, infrastructure, data access
- JavaScript (ES6+) - Frontend React application

**Secondary:**
- SQL - Database schema and seed scripts in `docs/`

## Runtime

**Backend:**
- .NET 8.0 (ASP.NET Core Web API)
- Target framework: `net8.0` (all four projects)

**Frontend:**
- Node.js 20 (build-time, via `node:20-alpine` Docker image)
- Nginx Alpine (production serving)

**Package Manager:**
- NuGet (backend, implicit via `dotnet restore`)
- npm (frontend)
- Lockfile: `frontend/package-lock.json` present

## Frameworks

**Core:**
- ASP.NET Core 8.0 - Web API framework (`GranjaTech.Api/GranjaTech.Api.csproj`)
- React 19.1.1 - Frontend SPA (`frontend/package.json`)
- React Router DOM 7.8.2 - Client-side routing (`frontend/package.json`)

**UI:**
- MUI (Material UI) 7.3.1 - Component library (`@mui/material`, `@mui/icons-material`)
- Emotion 11.14.x - CSS-in-JS styling (`@emotion/react`, `@emotion/styled`)

**Data Visualization:**
- Recharts 3.1.2 - Charts and graphs (`frontend/package.json`)

**Testing:**
- Jest (via react-scripts) - Frontend test runner
- Testing Library (React 16.3.0, DOM 10.4.1, jest-dom 6.7.0) - Frontend testing utilities

**Build/Dev:**
- react-scripts 5.0.1 - Frontend build toolchain (Create React App)
- Docker multi-stage builds - Both backend and frontend
- docker-compose - Orchestration

## Key Dependencies

**Backend - Critical:**
- `Microsoft.EntityFrameworkCore` 9.0.8 - ORM for PostgreSQL (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `Npgsql.EntityFrameworkCore.PostgreSQL` 9.0.4 - PostgreSQL EF Core provider (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)
- `Microsoft.AspNetCore.Authentication.JwtBearer` 8.0.8 - JWT authentication (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `System.IdentityModel.Tokens.Jwt` 7.7.0 - JWT token generation (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)
- `BCrypt.Net-Next` 4.0.3 - Password hashing (`GranjaTech.Infrastructure/GranjaTech.Infrastructure.csproj`)

**Backend - Infrastructure:**
- `Swashbuckle.AspNetCore` 6.6.2 - Swagger/OpenAPI documentation (`GranjaTech.Api/GranjaTech.Api.csproj`)
- `Microsoft.EntityFrameworkCore.Design` 9.0.8 - EF Core migrations tooling
- `Microsoft.EntityFrameworkCore.Tools` 9.0.8 - EF Core CLI tools

**Frontend - Critical:**
- `axios` 1.11.0 - HTTP client for API communication (`frontend/package.json`)
- `jwt-decode` 4.0.0 - JWT token decoding on client side
- `jspdf` 3.0.1 + `jspdf-autotable` 5.0.2 - PDF report generation
- `xlsx` 0.18.5 - Excel spreadsheet export
- `web-vitals` 2.1.4 - Performance monitoring

## Configuration

**Backend Configuration:**
- `GranjaTech.Api/appsettings.json` - Base configuration (connection string, JWT settings)
- `GranjaTech.Api/appsettings.Development.json` - Development overrides (verbose logging)
- `GranjaTech.Api/Properties/launchSettings.json` - Development server profiles (ports 5099 HTTP, 7135 HTTPS)
- Environment variables override appsettings via `__` notation (e.g., `ConnectionStrings__DefaultConnection`)

**Frontend Configuration:**
- `REACT_APP_API_URL` environment variable - API base URL (defaults to `https://localhost:7135/api`)
- ESLint config inline in `frontend/package.json` (extends `react-app`, `react-app/jest`)

**Key Config Sections:**
- `ConnectionStrings:DefaultConnection` - PostgreSQL connection
- `Jwt:Key` / `Jwt:Issuer` / `Jwt:Audience` - JWT token configuration
- `AllowedOrigins` - CORS allowed origins (semicolon-separated)
- `Swagger:Enabled` - Toggle Swagger in production

**Environment Template:**
- `.env.example` - Documents all required environment variables

## Build Configuration

**Backend Build:**
- Solution file: `GranjaTech.sln`
- Multi-stage Docker build: `Dockerfile` (SDK 8.0 build, ASP.NET 8.0 runtime)
- Exposes port 8080 in container
- Auto-applies EF Core migrations on startup (`db.Database.Migrate()`)

**Frontend Build:**
- Create React App toolchain via `react-scripts`
- Multi-stage Docker build: `frontend/Dockerfile` (Node 20 build, Nginx serve)
- Nginx config: `frontend/nginx.conf` (gzip, security headers, SPA routing, static asset caching)
- Exposes port 80 in container

## Platform Requirements

**Development:**
- .NET 8 SDK
- Node.js 20+
- PostgreSQL 16 (or via `docker-compose.dev.yml`)
- pgAdmin 4 available via dev compose on port 5050

**Production (Docker):**
- Docker + Docker Compose
- PostgreSQL 16 Alpine container
- Backend on port 5099 (mapped to container 8080)
- Frontend on port 3000 (mapped to container 80)

**Solution Structure (4 projects):**
- `GranjaTech.Api` - Web API entry point, controllers, DI configuration
- `GranjaTech.Application` - Service interfaces, DTOs
- `GranjaTech.Domain` - Domain entities (no dependencies)
- `GranjaTech.Infrastructure` - EF Core DbContext, service implementations, migrations

---

*Stack analysis: 2026-04-06*
