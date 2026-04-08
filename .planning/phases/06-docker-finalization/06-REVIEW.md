---
phase: 06-docker-finalization
reviewed: 2026-04-08T12:00:00Z
depth: standard
files_reviewed: 8
files_reviewed_list:
  - granjatech-api/Dockerfile
  - granjatech-api/.dockerignore
  - granjatech-frontend/Dockerfile
  - granjatech-frontend/.dockerignore
  - granjatech-frontend/nginx.conf
  - docker-compose.yml
  - granjatech-frontend/src/views/AviculturaView.vue
  - granjatech-frontend/src/router/index.ts
findings:
  critical: 2
  warning: 3
  info: 2
  total: 7
status: issues_found
---

# Phase 6: Code Review Report

**Reviewed:** 2026-04-08T12:00:00Z
**Depth:** standard
**Files Reviewed:** 8
**Status:** issues_found

## Summary

Reviewed Docker infrastructure (Dockerfiles, docker-compose, nginx config) and two Vue frontend files (AviculturaView, router). The Docker setup is well-structured with multi-stage builds, health checks, and proper dependency caching via cargo-chef. However, there are two critical security issues in docker-compose.yml (hardcoded credentials) and several warnings related to nginx security header inheritance and TypeScript type safety.

## Critical Issues

### CR-01: Hardcoded Database Password in docker-compose.yml

**File:** `docker-compose.yml:11`
**Issue:** The PostgreSQL password `postgres123` is hardcoded in the compose file. This file is committed to version control. Anyone with repo access gains database credentials. The same password appears in the DATABASE_URL on line 39.
**Fix:** Use environment variable substitution with a `.env` file (already in .gitignore):
```yaml
# docker-compose.yml
environment:
  POSTGRES_DB: GranjaTechDb
  POSTGRES_USER: ${POSTGRES_USER:-postgres}
  POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:?POSTGRES_PASSWORD must be set}

# backend environment
- DATABASE_URL=postgres://${POSTGRES_USER:-postgres}:${POSTGRES_PASSWORD}@postgres:5432/GranjaTechDb
```

### CR-02: Hardcoded JWT Secret in docker-compose.yml

**File:** `docker-compose.yml:40`
**Issue:** The JWT signing key `74b9f1d2-a3e9-4f7c-a9d8-9e2c1a3b5d7e-granjatech-super-secret` is hardcoded in the compose file committed to source control. This key is used to sign authentication tokens -- if leaked, an attacker can forge valid JWT tokens for any user/role.
**Fix:** Reference an environment variable:
```yaml
- JWT_KEY=${JWT_KEY:?JWT_KEY must be set}
```
And document the required variable in `.env.example`.

## Warnings

### WR-01: Nginx Security Headers Lost on Static Asset and Index Locations

**File:** `granjatech-frontend/nginx.conf:46-54`
**Issue:** In nginx, when an `add_header` directive appears inside a `location` block, it completely replaces all `add_header` directives inherited from the parent `server` block. The static assets location (line 46) adds `Cache-Control` but loses `X-Frame-Options`, `X-Content-Type-Options`, and `X-XSS-Protection`. The same applies to the `index.html` location (line 52) and the `/health` location (line 57).
**Fix:** Repeat the security headers in each location that uses `add_header`, or move to a shared `include` file:
```nginx
# Option A: Repeat in each location
location ~* \.(jpg|jpeg|png|gif|ico|css|js|svg|woff|woff2|ttf|eot)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}

# Option B: Use an include
# /etc/nginx/snippets/security-headers.conf
# Then: include /etc/nginx/snippets/security-headers.conf; in each location
```

### WR-02: Loose `any` Types in Dashboard Interface

**File:** `granjatech-frontend/src/views/AviculturaView.vue:93-95`
**Issue:** The `Dashboard` interface uses `any` for three fields (`resumoConsumo`, `resumoPesagem`, `resumoSanitario`). This disables type checking for these objects and allows runtime errors to pass through silently -- especially since these values are iterated with `v-for="(val, key) in ..."` in the template (lines 511, 525, 539).
**Fix:** Define proper interfaces for the resume types, or at minimum use `Record<string, string | number>`:
```typescript
interface Dashboard {
  metricas: Metrica
  alertas: Alerta[]
  resumoConsumo: Record<string, string | number> | null
  resumoPesagem: Record<string, string | number> | null
  resumoSanitario: Record<string, string | number> | null
}
```

### WR-03: Type Assertion `as any` Bypasses Type Safety

**File:** `granjatech-frontend/src/views/AviculturaView.vue:403`
**Issue:** `(dashboard.metricas as any)[m.key]` casts to `any` to perform dynamic key access. If a key in `metricaCards` does not match a property in `Metrica`, this will silently return `undefined` instead of producing a compile-time error.
**Fix:** Use a typed index approach:
```typescript
function getMetricaValue(key: string): number | undefined {
  if (!dashboard.value?.metricas) return undefined
  return (dashboard.value.metricas as Record<string, number>)[key]
}
```
Or define `metricaCards` with keys typed as `keyof Metrica`.

## Info

### IN-01: cargo-chef Installed Twice in Separate Build Stages

**File:** `granjatech-api/Dockerfile:8,15`
**Issue:** `cargo install cargo-chef` runs in both the `planner` and `cacher` stages. While this is standard for multi-stage Docker builds (stages don't share layers), you can avoid the double install by using a shared base image.
**Fix:** Optional optimization -- create a shared base:
```dockerfile
FROM rust:1-slim-bookworm AS chef-base
RUN cargo install cargo-chef

FROM chef-base AS planner
# ...

FROM chef-base AS cacher
# ...
```

### IN-02: Catch-all Route Redirects to Dashboard Without Auth Check

**File:** `granjatech-frontend/src/router/index.ts:96-98`
**Issue:** The catch-all route `/:pathMatch(.*)*` redirects to `/` (Dashboard). The `beforeEach` guard will then redirect unauthenticated users to Login, so this is not a security issue. However, users navigating to nonexistent paths get silently redirected to Dashboard rather than seeing a 404 page, which can be confusing.
**Fix:** Consider adding a dedicated NotFound view:
```typescript
{
  path: '/:pathMatch(.*)*',
  name: 'NotFound',
  component: () => import('@/views/NotFoundView.vue'),
  meta: { requiresAuth: false },
}
```

---

_Reviewed: 2026-04-08T12:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
