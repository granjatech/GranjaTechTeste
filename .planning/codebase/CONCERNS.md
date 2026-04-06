# Codebase Concerns

**Analysis Date:** 2026-04-06

## Tech Debt

**Stub/Unimplemented Methods in AviculturaService:**
- Issue: 14 interface methods return hardcoded empty values (`Task.FromResult(0m)`, `new List<>()`, `new Dto()`). These are placeholders that were never implemented.
- Files: `GranjaTech.Infrastructure/Services/Implementations/AviculturaService.cs` (lines 354-367)
- Impact: Any frontend or API consumer calling these endpoints receives empty/zero data silently. No error is raised, so bugs are hidden. Affected methods include `CalcularViabilidadeAsync`, `CalcularUniformidadeAsync`, `CalcularDensidadeAtualAsync`, `CalcularConsumoMedioRacaoPorAveAsync`, `CalcularConsumoMedioAguaPorAveAsync`, `CalcularRelacaoAguaRacaoAsync`, `CalcularMortalidadeAcumuladaAsync`, `CalcularMortalidadeSemanalAsync`, `CalcularMortalidadePorFaseAsync`, `ObterCurvasCrescimentoAsync`, `AnaliseConsumoDetalhada`, `ObterResumoSanitarioAsync`, `CalcularProjecaoAbateAsync`, `EstimarPesoMedioAbateAsync`.
- Fix approach: Implement each method using the same patterns as `CalcularIEPAsync` and `CalcularConversaoAlimentarAsync` which are already working. Alternatively, remove stub methods from the interface if not needed.

**Duplicated GetCurrentUser Pattern Across Services:**
- Issue: The `GetCurrentUser()` / `GetCurrentUserId()` private method is copy-pasted in at least 9 service files with slight variations. Some return `(int, string)`, one returns just `int`.
- Files: `GranjaTech.Infrastructure/Services/Implementations/AuthService.cs`, `EstoqueService.cs`, `SensorService.cs`, `FinancasService.cs`, `GranjaService.cs`, `RelatorioService.cs`, `DashboardService.cs`, `LoteService.cs`
- Impact: Inconsistent error handling across implementations. If auth logic changes, all 9 files must be updated.
- Fix approach: Extract a shared `ICurrentUserService` or base class with a single `GetCurrentUser()` implementation. Register it in DI and inject where needed.

**DbContext Directly Used in Controllers:**
- Issue: Multiple controllers inject `GranjaTechDbContext` directly and perform queries with business logic inline, bypassing the service layer.
- Files: `GranjaTech.Api/Controllers/ConsumoController.cs`, `GranjaTech.Api/Controllers/PesagemController.cs`, `GranjaTech.Api/Controllers/SanitarioController.cs`, `GranjaTech.Api/Controllers/RelatoriosController.cs` (avicultura and desempenho-lote endpoints), `GranjaTech.Api/Controllers/LeiturasController.cs`
- Impact: Business logic is scattered between controllers and services. No consistent authorization check on some controller-level queries (e.g., ConsumoController does not verify the user owns the lote). Makes unit testing controllers difficult.
- Fix approach: Move all DbContext queries into corresponding service classes. Controllers should only call service methods and map responses.

**Leftover Scaffold Files:**
- Issue: Auto-generated `Class1.cs` placeholder files still present.
- Files: `GranjaTech.Application/Class1.cs`, `GranjaTech.Infrastructure/Class1.cs`
- Impact: Minor clutter, no functional impact.
- Fix approach: Delete both files.

**Debug Endpoints Left in Production Code:**
- Issue: The RelatoriosController has 4 debug/test endpoints marked `[AllowAnonymous]` that expose internal data without authentication.
- Files: `GranjaTech.Api/Controllers/RelatoriosController.cs` (lines 36-124: `/health`, `/debug/memory`, `/debug/test-basic`, `/debug/simple`)
- Impact: Memory diagnostics and financial report test data are accessible without authentication. The `/debug/memory` endpoint exposes process memory info. The `/debug/test-basic` endpoint returns financial transaction counts.
- Fix approach: Remove debug endpoints or gate them behind development environment check and authentication.

**Excessive Console Logging in Frontend:**
- Issue: 86 `console.log`/`console.warn`/`console.error` calls across 16 frontend files, including detailed request/response logging in `apiService.js` that logs full headers, data payloads, and JWT token details.
- Files: `frontend/src/services/apiService.js` (lines 25-33, 46-51, 173-192), plus all pages in `frontend/src/pages/`
- Impact: Exposes sensitive data (JWT tokens, request payloads, headers) in browser console for any user. Performance overhead on every API call.
- Fix approach: Remove all debug `console.log` calls or wrap them in a `process.env.NODE_ENV === 'development'` check. Remove `debugToken()` function entirely.

## Security Considerations

**Unauthenticated IoT Endpoint:**
- Risk: The `LeiturasController` (`POST /api/leituras`) has no authentication. Any client can submit arbitrary sensor readings by guessing a sensor's `IdentificadorUnico`.
- Files: `GranjaTech.Api/Controllers/LeiturasController.cs`
- Current mitigation: None. The code comments acknowledge this: "Numa aplicacao real, ele seria protegido por uma chave de API."
- Recommendations: Implement API key authentication for IoT devices. Add rate limiting. Validate sensor data ranges.

**Exception Messages Leaked to Clients:**
- Risk: Internal exception messages (including `ex.Message`) are returned to API clients in error responses across all controllers. One endpoint even returns `ex.StackTrace`.
- Files: All files in `GranjaTech.Api/Controllers/` -- see grep results for `ex.Message`. Stack trace leak at `GranjaTech.Api/Controllers/RelatoriosController.cs` line 87.
- Current mitigation: None.
- Recommendations: Return generic error messages to clients. Log detailed exceptions server-side only. Remove `ex.StackTrace` from the `/debug/test-basic` response immediately.

**No Rate Limiting:**
- Risk: No rate limiting on any endpoint, including authentication (`/api/auth/login`). Brute-force password attacks are possible.
- Files: `GranjaTech.Api/Program.cs` (no rate limiting middleware configured)
- Current mitigation: None.
- Recommendations: Add `AspNetCoreRateLimit` or .NET 7+ built-in rate limiting middleware. At minimum, rate-limit the login endpoint.

**No CSRF Protection:**
- Risk: No anti-forgery token validation. While JWT-based APIs are less vulnerable to CSRF than cookie-based auth, the frontend stores the token in `localStorage` which is vulnerable to XSS.
- Files: `frontend/src/services/apiService.js` (line 19: `localStorage.getItem('token')`)
- Current mitigation: JWT stored in localStorage (not cookies), reducing CSRF risk but increasing XSS risk.
- Recommendations: Consider storing JWT in httpOnly cookies instead of localStorage. Add Content-Security-Policy headers.

**No Input Validation on Several DTOs:**
- Risk: Controllers that directly use DbContext accept DTOs without thorough server-side validation. For example, `ConsumoController` validates `AvesVivas` but not `QuantidadeKg` for negative values or `Data` for future dates.
- Files: `GranjaTech.Api/Controllers/ConsumoController.cs`, `PesagemController.cs`, `SanitarioController.cs`
- Current mitigation: Partial validation only.
- Recommendations: Add `[Range]`, `[Required]` data annotations on all DTO properties. Consider FluentValidation for complex rules.

**Magic Number for Profile ID:**
- Risk: Profile/role authorization uses hardcoded integer `3` to check for "Financeiro" role instead of a constant or enum.
- Files: `GranjaTech.Infrastructure/Services/Implementations/AuthService.cs` (lines 116, 156)
- Current mitigation: None.
- Recommendations: Create a `Perfis` enum or constants class (e.g., `PerfilIds.Financeiro = 3`).

## Performance Bottlenecks

**N+1 Query Patterns in Avicultura Report:**
- Problem: The `/api/relatorios/avicultura` endpoint loads all lotes with 6 `.Include()` chains, then performs in-memory LINQ operations (`.Sum()`, `.Average()`, `.OrderByDescending()`, `.Select()`) on each lote's collections.
- Files: `GranjaTech.Api/Controllers/RelatoriosController.cs` (lines 229-336)
- Cause: All related entity collections are eagerly loaded into memory. For lotes with many records, this loads massive amounts of data.
- Improvement path: Move this query to a service. Use projections with `.Select()` at the database level. Use `AsNoTracking()`. Consider pagination or server-side aggregation.

**Hardcoded Take(1000) Limits:**
- Problem: Financial and production reports use `Take(1000)` to cap results, but this is arbitrary and silently truncates data.
- Files: `GranjaTech.Infrastructure/Services/Implementations/RelatorioService.cs` (lines 171, 178, 308, 369)
- Cause: Added as a memory protection but without informing the client that data was truncated.
- Improvement path: Implement proper pagination with `skip`/`take` parameters. Return total count so clients know if data is incomplete.

**DateTime.Now vs DateTime.UtcNow Inconsistency:**
- Problem: Some code uses `DateTime.Now` (local time) while the database and most code uses UTC. This causes incorrect comparisons when server timezone differs from UTC.
- Files: `GranjaTech.Infrastructure/Services/Implementations/AviculturaService.cs` (lines 181, 197), `GranjaTech.Infrastructure/Services/Implementations/RelatorioService.cs` (line 82), `GranjaTech.Api/Controllers/SanitarioController.cs` (line 189), `GranjaTech.Infrastructure/Data/AviculturaSeedData.cs` (line 62)
- Cause: Inconsistent convention across the codebase.
- Improvement path: Replace all `DateTime.Now` with `DateTime.UtcNow`. Consider an `IClock` abstraction for testability.

## Fragile Areas

**RelatoriosController (571 lines):**
- Files: `GranjaTech.Api/Controllers/RelatoriosController.cs`
- Why fragile: Mixes concerns -- some endpoints delegate to services, others build complex anonymous objects inline with deep nesting (avicultura report builds a 100+ line anonymous type). Contains debug endpoints. Uses both service layer and direct DbContext.
- Safe modification: Extract the avicultura and desempenho-lote endpoints into `IRelatorioService` methods. Keep controller thin.
- Test coverage: Zero tests.

**Frontend Page Files (300-973 lines each):**
- Files: `frontend/src/pages/RelatoriosPage.js` (973 lines), `frontend/src/pages/SanitarioPage.js` (689 lines), `frontend/src/pages/ConsumoPage.js` (635 lines), `frontend/src/pages/PesagemPage.js` (596 lines)
- Why fragile: Each page is a single monolithic component with state management, API calls, data transformation, and rendering all in one file. No component decomposition.
- Safe modification: Extract data fetching into custom hooks. Break UI into smaller sub-components. Extract table/form sections.
- Test coverage: Zero meaningful tests (only `App.test.js` exists and it tests for "learn react" text that does not exist in the app).

**Authorization Logic Scattered:**
- Files: Every service in `GranjaTech.Infrastructure/Services/Implementations/` plus controllers that bypass services
- Why fragile: Authorization checks are implemented differently in controllers vs services. Controllers using DbContext directly may skip authorization entirely (e.g., `ConsumoController` does not verify user owns the lote before writing data).
- Safe modification: Centralize authorization into the service layer. Ensure every write operation validates ownership.
- Test coverage: None.

## Test Coverage Gaps

**No Backend Tests:**
- What's not tested: The entire backend -- zero test projects exist in the solution.
- Files: No test project in `GranjaTech.sln`
- Risk: Any refactoring, bug fix, or feature addition has zero safety net. Authorization logic, business calculations (IEP, CA, GMD), financial reports, and data integrity are all untested.
- Priority: High

**Single Broken Frontend Test:**
- What's not tested: All frontend functionality. The only test file searches for "learn react" text that does not exist in the application.
- Files: `frontend/src/App.test.js`
- Risk: This test likely fails if run. No page, component, or service is tested.
- Priority: High

## Dependencies at Risk

**Frontend .env Files Committed:**
- Risk: `frontend/.env.development` and `frontend/.env.production` are tracked in git. While they currently contain only API URLs (not secrets), this pattern encourages adding secrets later.
- Files: `frontend/.env.development`, `frontend/.env.production`
- Impact: If secrets are added, they will be in git history permanently.
- Migration plan: Add `frontend/.env.development` and `frontend/.env.production` to `.gitignore`. Use `.env.example` files instead.

## Missing Critical Features

**No Password Reset Flow:**
- Problem: Users can change passwords (if they know the current one) but there is no "forgot password" or password reset mechanism.
- Blocks: Users locked out of accounts have no self-service recovery.

**No Pagination on List Endpoints:**
- Problem: All list endpoints (`GET /api/granjas`, `GET /api/lotes`, `GET /api/financas`, `GET /api/estoque`, `GET /api/auditoria`) return all records without pagination.
- Files: All service `GetAll` methods in `GranjaTech.Infrastructure/Services/Implementations/`
- Blocks: Will not scale beyond small datasets. Audit logs will grow unbounded.

**No Request Validation Middleware:**
- Problem: No global model validation or error handling middleware. Each controller implements its own try/catch pattern with inconsistent error response formats.
- Files: `GranjaTech.Api/Program.cs` (no `UseExceptionHandler` or validation filter)
- Blocks: Inconsistent API error responses make frontend error handling brittle.

---

*Concerns audit: 2026-04-06*
