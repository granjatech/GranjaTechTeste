# Testing Patterns

**Analysis Date:** 2026-04-06

## Test Framework

**Frontend Runner:**
- Jest (via react-scripts 5.0.1, bundled with Create React App)
- Config: Inline in `frontend/package.json` scripts, no separate `jest.config.js`

**Frontend Assertion Library:**
- `@testing-library/jest-dom` ^6.7.0 (custom DOM matchers)
- `@testing-library/react` ^16.3.0 (React component rendering)
- `@testing-library/user-event` ^13.5.0 (user interaction simulation)

**Backend Runner:**
- **None.** No test projects exist. No `*.Tests.csproj` files detected anywhere in the solution.

**Run Commands:**
```bash
cd frontend && npm test       # Run frontend tests (Jest watch mode)
cd frontend && npm test -- --coverage  # Coverage report
cd frontend && CI=true npm test        # Single run (CI mode)
```

## Test File Organization

**Frontend Location:**
- Co-located with source: `frontend/src/App.test.js` alongside `frontend/src/App.js`

**Frontend Naming:**
- `[ComponentName].test.js` pattern

**Current Test Files:**
```
frontend/src/
  App.test.js          # Only test file in the entire project
  setupTests.js        # Jest-DOM setup
```

**Backend Location:**
- No test directory exists. No test projects in `GranjaTech.sln`.

## Test Structure

**Setup File:**
`frontend/src/setupTests.js` imports `@testing-library/jest-dom` globally for all tests:
```javascript
// frontend/src/setupTests.js
import '@testing-library/jest-dom';
```

**Existing Test (only one):**
```javascript
// frontend/src/App.test.js
import { render, screen } from '@testing-library/react';
import React from 'react';

jest.mock(
  'react-router-dom',
  () => ({
    BrowserRouter: ({ children }) => <div>{children}</div>,
    Routes: ({ children }) => <div>{children}</div>,
    Route: ({ element }) => <div>{element}</div>,
    Navigate: () => null,
  }),
  { virtual: true }
);

const App = require('./App').default;

test('renders learn react link', () => {
  render(<App />);
  const linkElement = screen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});
```

**NOTE:** This test is broken. It searches for "learn react" text which does not exist in the actual `App.js` component. This test will fail if executed.

## Mocking

**Framework:** Jest built-in `jest.mock()`

**Observed Pattern:**
```javascript
// Mocking react-router-dom with virtual module flag
jest.mock(
  'react-router-dom',
  () => ({
    BrowserRouter: ({ children }) => <div>{children}</div>,
    Routes: ({ children }) => <div>{children}</div>,
    Route: ({ element }) => <div>{element}</div>,
    Navigate: () => null,
  }),
  { virtual: true }
);
```

**What Would Need Mocking (for future tests):**
- `apiService` - all API calls go through `frontend/src/services/apiService.js`
- `AuthContext` - authentication state (`token`, `user`, `login`, `logout`)
- `AccessibilityContext` - theme mode and font scale
- `localStorage` - token storage
- `react-router-dom` - navigation hooks (`useNavigate`, `useLocation`)
- `jwt-decode` - token decoding

**Backend Mocking (if tests were added):**
- `GranjaTechDbContext` - EF Core in-memory database or mock
- `IHttpContextAccessor` - user claims for role-based testing
- Service interfaces via their `I*Service` interfaces (all registered via DI)

## Fixtures and Factories

**Test Data:**
- No test fixtures or factories exist
- Seed data exists for development only in `GranjaTech.Infrastructure/Data/AviculturaSeedData.cs`
- Profile seed data hardcoded in `GranjaTech.Infrastructure/GranjaTechDbContext.cs` `OnModelCreating()`:
  - Perfil: Administrador (1), Produtor (2), Financeiro (3)
  - Default admin user: admin@admin.com

**Location:**
- No dedicated fixtures directory

## Coverage

**Requirements:** None enforced. No coverage thresholds configured.

**View Coverage:**
```bash
cd frontend && CI=true npm test -- --coverage
```

**Current State:** Effectively 0% meaningful coverage. The single existing test is a broken CRA boilerplate test.

## Test Types

**Unit Tests:**
- Frontend: One broken test exists. No unit tests for services, contexts, or components.
- Backend: None. No test project exists.

**Integration Tests:**
- None for either frontend or backend.

**E2E Tests:**
- Not configured. No Cypress, Playwright, or similar framework installed.

## Recommended Test Setup (for new tests)

**Frontend Unit Test Pattern (recommended):**
```javascript
// Example: frontend/src/pages/LoginPage.test.js
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import LoginPage from './LoginPage';

const renderWithProviders = (ui, { authValue = {} } = {}) => {
  const defaultAuth = { token: null, user: null, login: jest.fn(), logout: jest.fn() };
  return render(
    <AuthContext.Provider value={{ ...defaultAuth, ...authValue }}>
      <BrowserRouter>{ui}</BrowserRouter>
    </AuthContext.Provider>
  );
};

describe('LoginPage', () => {
  it('renders login form', () => {
    renderWithProviders(<LoginPage />);
    expect(screen.getByRole('button')).toBeInTheDocument();
  });
});
```

**Backend Test Project (recommended setup):**
```bash
# Create test project
dotnet new xunit -n GranjaTech.Tests
dotnet sln add GranjaTech.Tests/GranjaTech.Tests.csproj
dotnet add GranjaTech.Tests reference GranjaTech.Application GranjaTech.Infrastructure GranjaTech.Domain
dotnet add GranjaTech.Tests package Moq
dotnet add GranjaTech.Tests package Microsoft.EntityFrameworkCore.InMemory
```

**Backend Unit Test Pattern (recommended):**
```csharp
// Example: GranjaTech.Tests/Services/GranjaServiceTests.cs
using GranjaTech.Infrastructure;
using GranjaTech.Infrastructure.Services.Implementations;
using Microsoft.EntityFrameworkCore;
using Moq;
using Xunit;

public class GranjaServiceTests
{
    private GranjaTechDbContext CreateInMemoryContext()
    {
        var options = new DbContextOptionsBuilder<GranjaTechDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new GranjaTechDbContext(options);
    }

    [Fact]
    public async Task GetAllAsync_Admin_ReturnsAllGranjas()
    {
        // Arrange
        using var context = CreateInMemoryContext();
        // ... setup mock IHttpContextAccessor with admin claims
        // Act & Assert
    }
}
```

## Common Patterns

**Async Testing (frontend):**
```javascript
// Use waitFor for async operations
await waitFor(() => {
  expect(screen.getByText('Dashboard')).toBeInTheDocument();
});
```

**Error Testing (frontend):**
```javascript
// Mock API failure
jest.spyOn(apiService, 'getGranjas').mockRejectedValue(new Error('Network error'));
// Render and verify error state
```

## Critical Gaps

1. **No backend tests at all** - Zero C# test projects. All business logic (role-based access, financial calculations, aviculture metrics in `Lote.cs`) is untested.
2. **Single broken frontend test** - `frontend/src/App.test.js` asserts text that does not exist in the component.
3. **No CI test enforcement** - The GitHub Actions workflow (`.github/workflows/`) does not appear to run tests as a gate.
4. **Complex business logic untested** - `Lote.CalcularConversaoAlimentar()` and `Lote.CalcularIEP()` in `GranjaTech.Domain/Lote.cs` contain arithmetic that should be unit tested.
5. **Role-based access untested** - Every service has `GetCurrentUser()` role filtering (Administrador/Produtor/Financeiro) with no test coverage.

---

*Testing analysis: 2026-04-06*
