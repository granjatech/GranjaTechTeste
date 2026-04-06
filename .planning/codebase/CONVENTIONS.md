# Coding Conventions

**Analysis Date:** 2026-04-06

## Languages

This is a bilingual codebase (Portuguese domain language, English framework code). All domain entities, DTOs, variables, route paths, error messages, and UI labels are in **Brazilian Portuguese**. Framework-level constructs (C# keywords, React hooks, MUI components) remain in English.

## Naming Patterns

**C# Files:**
- Domain entities: PascalCase singular nouns in Portuguese - `Granja.cs`, `Lote.cs`, `Usuario.cs`, `TransacaoFinanceira.cs`
- DTOs: PascalCase with prefix `Create`/`Update` + entity + `Dto` - `CreateGranjaDto.cs`, `UpdateLoteDto.cs`, `LoginResponseDto.cs`
- Service interfaces: `I` + entity + `Service` - `IGranjaService.cs`, `IAuthService.cs`
- Service implementations: entity + `Service` - `GranjaService.cs`, `AuthService.cs`
- Controllers: entity (pluralized) + `Controller` - `GranjasController.cs`, `LotesController.cs`
- DbContext: `GranjaTechDbContext.cs`

**JavaScript Files:**
- Pages: PascalCase + `Page` suffix - `DashboardPage.js`, `LoginPage.js`, `GranjasPage.js`
- Components: PascalCase descriptive - `ProtectedRoute.js`, `ResponsiveNavigation.js`, `PageContainer.js`, `LoadingSpinner.js`
- Contexts: PascalCase + `Context` suffix - `AuthContext.js`, `AccessibilityContext.js`
- Services: camelCase + descriptive - `apiService.js`, `relatoriosApi.js`

**C# Functions/Methods:**
- Service methods: async with `Async` suffix - `GetAllAsync()`, `AddAsync()`, `UpdateAsync()`, `DeleteAsync()`
- Private helpers: camelCase-style prefixed verb - `GetCurrentUser()`, `GetCurrentUserId()`, `GenerateJwtToken()`
- Audit logging: `RegistrarLog(string acao, string detalhes)` (Portuguese)

**C# Variables:**
- Private fields: underscore-prefixed camelCase - `_context`, `_authService`, `_httpContextAccessor`
- Local variables: camelCase in Portuguese - `granjaExistente`, `novaGranja`, `novoCodigo`, `senhaHash`
- Tuples for user context: `var (userId, userRole) = GetCurrentUser();`

**JavaScript Functions:**
- React components: PascalCase function declarations - `function DashboardPage() {}`
- Event handlers: `handle` prefix + verb - `handleSubmit`
- Data fetchers: `fetch` prefix - `fetchData`
- API service methods: camelCase verb + entity - `getGranjas()`, `createLote()`, `deleteTransacao()`

**JavaScript Variables:**
- State: camelCase descriptive - `const [kpis, setKpis] = useState(null)`
- Constants: camelCase for objects, UPPER_SNAKE not used

**Types/Interfaces (C#):**
- Interfaces: `I` prefix with PascalCase - `IGranjaService`, `IAuthService`, `ICacheService`
- Enums: Not used; string literals instead (e.g., `"Administrador"`, `"Produtor"`, `"Financeiro"` for roles)

## Code Style

**Formatting (Backend - C#):**
- No explicit formatter config (no `.editorconfig` detected)
- 4-space indentation
- Opening braces on same line for single-line blocks, new line for multi-line
- Nullable reference types enabled (`<Nullable>enable</Nullable>`)
- Implicit usings enabled

**Formatting (Frontend - JavaScript):**
- No Prettier or ESLint config files; relies on default `react-app` ESLint config in `package.json`
- 4-space indentation in JSX
- Single quotes for JS strings
- Template literals for dynamic strings

**Linting:**
- Backend: No analyzers configured
- Frontend: `react-app` and `react-app/jest` ESLint presets only (defined in `frontend/package.json` `eslintConfig`)

## Import Organization

**C# Import Order (observed in controllers and services):**
1. Project-internal namespaces (`GranjaTech.Application.DTOs`, `GranjaTech.Application.Services.Interfaces`, `GranjaTech.Domain`)
2. Microsoft/framework namespaces (`Microsoft.AspNetCore.*`, `Microsoft.EntityFrameworkCore`)
3. System namespaces (`System`, `System.Collections.Generic`, `System.Linq`, `System.Threading.Tasks`)

**JavaScript Import Order (observed in pages/components):**
1. React and React hooks - `import React, { useState, useEffect, useCallback } from 'react'`
2. Internal services/contexts - `import apiService from '../services/apiService'`
3. Internal components - `import PageContainer from '../components/PageContainer'`
4. MUI components - `import { Grid, Paper, Typography } from '@mui/material'`
5. MUI icons - `import { TrendingUp as TrendingUpIcon } from '@mui/icons-material'`
6. Third-party libraries (recharts, etc.)

**Path Aliases:**
- None configured. All imports use relative paths (`../services/apiService`, `../context/AuthContext`)

## Error Handling

**Backend Controller Pattern:**
Use try/catch blocks that catch `InvalidOperationException` for business rule violations:
```csharp
// Pattern in GranjaTech.Api/Controllers/AuthController.cs
try
{
    var sucesso = await _authService.RegistrarAsync(registerDto);
    if (!sucesso) return BadRequest(new { message = "Email ja cadastrado." });
    return Ok(new { message = "Usuario criado com sucesso!" });
}
catch (InvalidOperationException ex)
{
    return BadRequest(new { message = ex.Message });
}
```

**Backend Service Pattern:**
Throw `InvalidOperationException` for authorization/business rule failures:
```csharp
// Pattern in GranjaTech.Infrastructure/Services/Implementations/GranjaService.cs
if (userRole == "Financeiro")
    throw new InvalidOperationException("Usuarios do perfil Financeiro nao podem criar granjas.");
```

**Return booleans** for success/failure of update/delete operations. Return `null` for not-found entities.

**Global Error Handling:**
Custom middleware in `GranjaTech.Api/Program.cs` catches unhandled exceptions, logs them, and returns a JSON error response with `requestId` and status 500.

**Frontend Error Pattern:**
Try/catch in async functions with `console.error` and user-facing state:
```javascript
// Pattern in frontend/src/pages/DashboardPage.js
try {
    setLoading(true);
    const [kpisRes, monthlyRes] = await Promise.all([...]);
    setKpis(kpisRes.data);
} catch (error) {
    console.error("Erro ao buscar dados:", error);
} finally {
    setLoading(false);
}
```

**API Client Error Handling:**
Axios interceptor in `frontend/src/services/apiService.js` auto-redirects to `/login` on 401 responses and removes the stored token.

## Logging

**Backend Framework:** Built-in `ILogger<T>` from ASP.NET Core

**Backend Patterns:**
- Structured logging with named parameters: `_logger.LogInformation("Retornando relatorio com {TransacoesCount} transacoes", count)`
- Error logging with exception: `_logger.LogError(ex, "Erro ao gerar relatorio")`
- Request tracking middleware generates `requestId` for each request

**Frontend Framework:** `console.log` / `console.error` / `console.warn`

**Frontend Patterns:**
- Emoji-prefixed console logs in `apiService.js` for debug purposes (should be removed in production)
- Request/response interceptors log full details including headers and data

## Comments

**When to Comment (C#):**
- XML doc comments (`///`) used on domain entity properties in `GranjaTech.Domain/Lote.cs` for complex business logic
- Inline comments in Portuguese for section headers using `// ========` separators in `Program.cs`
- Code left with change notes (e.g., `// ADICIONE ESTA LINHA`, `// Assinatura alterada`) - these should be cleaned up

**When to Comment (JavaScript):**
- Minimal comments in React components
- Portuguese inline comments for business context

**JSDoc/TSDoc:** Not used (project is plain JavaScript, not TypeScript)

## Controller Design

**Route Pattern:** `[Route("api/[controller]")]` with `[ApiController]` attribute on all controllers

**Authorization Pattern:**
- Class-level `[Authorize]` for controllers requiring any authenticated user (`GranjasController`)
- Method-level `[Authorize(Roles = "...")]` for role-specific endpoints (`AuthController`, `FinancasController`)
- `[AllowAnonymous]` for public endpoints (login, register, health checks)

**Response Pattern:**
- Success: `return Ok(data)` or `return Ok(new { message = "..." })`
- Not found: `return NotFound()` or `return NotFound("message")`
- Validation error: `return BadRequest(new { message = "..." })`
- Auth failure: `return Forbid(ex.Message)` or `return Unauthorized(new { message = "..." })`
- Create success: `return Ok(new { message = "..." })` (not 201 Created)
- Update/Delete success: `return NoContent()`

## Service Design

**Constructor Injection Pattern:**
All services receive dependencies via constructor. Common injected dependencies:
- `GranjaTechDbContext _context` - database access
- `IHttpContextAccessor _httpContextAccessor` - current user context
- `IAuditoriaService _auditoriaService` - audit logging

**Current User Pattern:**
Private method `GetCurrentUser()` returns `(int userId, string userRole)` tuple extracted from JWT claims. Duplicated across multiple services (`GranjaService`, `FinancasService`, etc.).

**CRUD Pattern:**
Services follow consistent CRUD method signatures:
- `Task<IEnumerable<T>> GetAllAsync()` - filtered by user role
- `Task<T?> GetByIdAsync(int id)` - with role-based access check
- `Task AddAsync(CreateDto dto)` - maps DTO to entity, saves, logs audit
- `Task<bool> UpdateAsync(int id, UpdateDto dto)` - returns false if not found
- `Task DeleteAsync(int id)` - void or bool return

**Audit Pattern:**
All CUD operations call `_auditoriaService.RegistrarLog("ACTION_TYPE", "description")` after successful database save.

## Frontend Component Design

**Page Components:**
- Wrap content in `<PageContainer title="..." subtitle="...">` for consistent layout
- Use `<LoadingSpinner message="..." />` during data fetching
- State management via `useState` + `useEffect` with `useCallback` for fetch functions
- All API calls go through `apiService` singleton from `frontend/src/services/apiService.js`

**Auth Pattern:**
- `AuthContext` provides `{ token, user, login, logout }`
- `ProtectedRoute` wraps pages requiring authentication
- Token stored in `localStorage`
- Role-based navigation filtering in `ResponsiveNavigation`

**Theming:**
- MUI v7 with custom theme factory in `frontend/src/theme.js`
- Dark/light mode via `AccessibilityContext`
- Font scaling support
- Green primary color (`#2E7D32`), orange secondary (`#FF6F00`)

## Module Design

**Backend Exports:** One class per file. Namespaces follow folder structure exactly.

**Frontend Exports:** Default exports for all components, pages, and services. No barrel files (index.js re-exports).

**DI Registration:** All service registrations in `GranjaTech.Api/Program.cs` as `AddScoped<Interface, Implementation>()`.

---

*Convention analysis: 2026-04-06*
