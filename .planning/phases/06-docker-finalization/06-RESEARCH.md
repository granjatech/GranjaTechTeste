# Phase 6: Docker & Finalization - Research

**Researched:** 2026-04-08
**Domain:** Docker containerization (Rust + Vue + PostgreSQL)
**Confidence:** HIGH

## Summary

Phase 6 containerizes the fully migrated GranjaTech stack: PostgreSQL 16, Rust/Actix-web backend, and Vue 3/Vite frontend behind nginx with reverse proxy. The existing project already has working Docker configs for the .NET/React stack that serve as direct templates -- the new configs follow the same topology (bridge network, named volumes, healthchecks) but swap the application containers.

The key architectural decision is the nginx reverse proxy (D-06): the Vue frontend and Rust API are both served through port 80, with `/api` and `/swagger-ui` proxied to the backend container. This eliminates cross-origin CORS complexity entirely. The Rust backend already binds to `0.0.0.0:8080` and exposes a `/health` endpoint returning JSON `{"status":"ok"}`, so container healthchecks are trivial.

cargo-chef is used for Rust Docker layer caching (D-02), which is the standard approach for Rust Docker builds. The three-stage pattern (planner, cacher, builder) ensures dependency compilation is cached and only application code changes trigger recompilation. Importantly, the Rust backend uses runtime `sqlx::query` / `sqlx::query_as` calls (165 occurrences across 17 files, zero `query!` macros), so no compile-time database connection is needed during Docker build. [VERIFIED: grep for `sqlx::query!` returned 0 matches]

**Primary recommendation:** Create four files: `granjatech-api/Dockerfile`, `granjatech-frontend/Dockerfile`, `granjatech-frontend/nginx.conf`, and a new `docker-compose.yml` (replacing the .NET one). Keep `docker-compose.dev.yml` unchanged.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Multi-stage build with `rust:1-slim-bookworm` (build) + `debian:bookworm-slim` (runtime). Install `pkg-config libssl-dev` in build, `ca-certificates libssl3` in runtime.
- **D-02:** Use `cargo-chef` for Rust dependency caching in Docker. Three stages: planner (generates recipe), cacher (compiles deps), builder (compiles app).
- **D-03:** Final image exposes port 8080. Single binary copied from builder.
- **D-04:** Multi-stage build with `node:20-alpine` (build) + `nginx:alpine` (runtime). Same pattern as current React Dockerfile but using Vite's `npm run build`.
- **D-05:** `VITE_API_URL=/api` hardcoded in Dockerfile. With nginx reverse proxy, everything goes through same origin -- no external variable needed.
- **D-06:** nginx.conf with reverse proxy: `location /api { proxy_pass http://backend:8080; }`. Frontend and API accessible on same port 80. Eliminates cross-origin CORS.
- **D-07:** Proxy also for `/swagger-ui` to access docs via same port.
- **D-08:** Keep security headers (X-Frame-Options, X-Content-Type-Options), gzip, static asset cache, and SPA routing (`try_files $uri /index.html`) from existing nginx.conf.
- **D-09:** External ports: 8080 (backend direct, optional) and 80 (frontend + proxy). Changed from .NET ports (5099 + 3000) to more conventional ones.
- **D-10:** PostgreSQL keeps port 5432 and same config (postgres:16-alpine image, named volume, pg_isready healthcheck).
- **D-11:** Backend depends on PostgreSQL (condition: service_healthy). Frontend depends on backend.
- **D-12:** Rust backend env vars injected via docker-compose: DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE, ALLOWED_ORIGINS, SWAGGER_ENABLED.
- **D-13:** docker-compose.dev.yml keeps only PostgreSQL + pgAdmin (no app containers). Dev runs Rust via `cargo run` and Vue via `npm run dev` locally.

### Claude's Discretion
- Health check implementation details (interval, timeout, retries)
- Exact cargo-chef recipe structure
- Whether to keep the old .NET Dockerfile or remove it (cleanup)
- Non-root user configuration in runtime image

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DOCK-01 | Dockerfile multi-stage para backend Rust (build + runtime slim) | D-01, D-02, D-03: cargo-chef three-stage pattern with rust:1-slim-bookworm + debian:bookworm-slim |
| DOCK-02 | Dockerfile multi-stage para frontend Vue (build + nginx) | D-04, D-05, D-06, D-07, D-08: node:20-alpine build + nginx:alpine with reverse proxy config |
| DOCK-03 | docker-compose.yml com PostgreSQL 16 + Rust backend + Vue frontend | D-09 through D-13: Full compose topology with healthchecks, deps, env vars |
| DOCK-04 | Health check endpoint respondendo em /health | Already implemented in Rust backend (returns JSON {"status":"ok"}); nginx /health endpoint in config |
</phase_requirements>

## Standard Stack

### Core

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Docker | 29.3.1 | Container runtime | Already installed on dev machine [VERIFIED: `docker --version`] |
| Docker Compose | v5.1.1 | Multi-container orchestration | Already installed [VERIFIED: `docker compose version`] |
| rust:1-slim-bookworm | latest | Rust build stage base image | Locked decision D-01, slim reduces image size vs full |
| debian:bookworm-slim | latest | Rust runtime stage base image | Locked decision D-01, Debian for OpenSSL/glibc compat |
| cargo-chef | latest | Rust Docker layer caching | Locked decision D-02, standard approach for Rust Docker builds |
| node:20-alpine | 20.x | Vue build stage | Locked decision D-04, matches project Node requirement |
| nginx:alpine | latest | Frontend serving + reverse proxy | Locked decision D-04, lightweight production web server |
| postgres:16-alpine | 16.x | Database | Existing, unchanged (D-10) |

### Supporting

No additional libraries needed -- this phase is pure Docker/infrastructure configuration.

**Installation:** No packages to install. All tools are Docker images pulled at build time.

## Architecture Patterns

### File Structure

```
granjatech-api/
  Dockerfile              # NEW: Rust multi-stage build (cargo-chef)

granjatech-frontend/
  Dockerfile              # NEW: Vue/Vite build + nginx
  nginx.conf              # NEW: nginx config with reverse proxy

docker-compose.yml        # REPLACE: New stack (Rust + Vue + PostgreSQL)
docker-compose.dev.yml    # KEEP: Unchanged (PostgreSQL + pgAdmin only)

Dockerfile                # OLD: .NET backend (keep or remove -- discretion)
frontend/Dockerfile       # OLD: React frontend (keep as reference)
frontend/nginx.conf       # OLD: React nginx config (reference for new one)
```

### Pattern 1: cargo-chef Three-Stage Rust Build

**What:** Splits Rust Docker build into planner/cacher/builder stages to cache dependency compilation separately from application code. [CITED: https://github.com/LukeMathWalker/cargo-chef]

**When to use:** Any Rust project in Docker where rebuild speed matters.

**Example:**
```dockerfile
# Stage 1: Planner -- generate dependency recipe
FROM rust:1-slim-bookworm AS planner
RUN cargo install cargo-chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cacher -- compile dependencies only (cached unless Cargo.toml changes)
FROM rust:1-slim-bookworm AS cacher
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Builder -- compile application code
FROM rust:1-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
RUN cargo build --release

# Stage 4: Runtime
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/granjatech-api .
EXPOSE 8080
CMD ["./granjatech-api"]
```

### Pattern 2: Nginx Reverse Proxy for SPA + API

**What:** Single nginx serves Vue SPA files and proxies `/api` and `/swagger-ui` to backend container. [ASSUMED]

**When to use:** When frontend and backend should share the same origin to avoid CORS.

**Example:**
```nginx
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Gzip
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/json;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # API reverse proxy
    location /api {
        proxy_pass http://backend:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Swagger UI reverse proxy
    location /swagger-ui {
        proxy_pass http://backend:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # OpenAPI spec reverse proxy
    location /api-docs {
        proxy_pass http://backend:8080;
        proxy_set_header Host $host;
    }

    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location ~* \.(jpg|jpeg|png|gif|ico|css|js|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # No cache for index.html
    location = /index.html {
        add_header Cache-Control "no-store, no-cache, must-revalidate";
    }

    # Health check
    location /health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
```

### Anti-Patterns to Avoid

- **Building Rust without cargo-chef:** Full recompilation of all dependencies on every code change (10+ minutes). Always use cargo-chef or equivalent caching strategy.
- **Using Alpine for Rust runtime:** musl libc causes issues with OpenSSL/SQLx. Use Debian-based images for Rust (decision D-01 already avoids this). [ASSUMED]
- **Hardcoding passwords in Dockerfile:** Environment variables should be injected via docker-compose, not baked into images.
- **Running as root in production containers:** Add a non-root user in the runtime stage for security.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rust dependency caching | Custom COPY Cargo.toml tricks | cargo-chef | Handles workspace layouts, conditional deps, features correctly |
| SSL certificates in container | Manual cert installation | `ca-certificates` package | System package handles trust store properly |
| Reverse proxy | Custom Node.js proxy | nginx `proxy_pass` | Battle-tested, zero overhead, handles WebSocket upgrade if needed |

## Common Pitfalls

### Pitfall 1: Vite Build Output Directory
**What goes wrong:** Copying from wrong directory -- React uses `build/`, Vite uses `dist/`.
**Why it happens:** Copy-pasting from the existing React Dockerfile.
**How to avoid:** The Vue Dockerfile must `COPY --from=build /app/dist /usr/share/nginx/html` (not `/app/build`).
**Warning signs:** Empty nginx directory, 403 errors on frontend.
**Verified:** `granjatech-frontend/dist/` exists with `index.html` and `assets/`. [VERIFIED: `ls` command]

### Pitfall 2: VITE_API_URL Must Be Set at Build Time
**What goes wrong:** Setting `VITE_API_URL` as a runtime env var has no effect -- Vite embeds env vars at build time.
**Why it happens:** Unlike Create React App which also embeds at build time, developers sometimes expect runtime injection.
**How to avoid:** Use `ENV VITE_API_URL=/api` BEFORE `RUN npm run build` in the Dockerfile.
**Warning signs:** API calls going to `http://localhost:5099/api` instead of `/api`.
**Verified:** `api.ts` uses `import.meta.env.VITE_API_URL || 'http://localhost:5099/api'`. [VERIFIED: codebase grep]

### Pitfall 3: Docker .dockerignore for Rust
**What goes wrong:** Docker COPY includes `target/` directory (gigabytes of build artifacts) or `.env` file (secrets).
**Why it happens:** No `.dockerignore` file in the Rust project directory.
**How to avoid:** Create `granjatech-api/.dockerignore` excluding `target/`, `.env`, and `.git`.
**Warning signs:** Extremely slow Docker build context transfer, or secrets leaking into image layers.

### Pitfall 4: nginx Location Block Order
**What goes wrong:** The SPA catch-all `location /` matches before `/api` proxy.
**Why it happens:** nginx uses longest prefix match for `location` directives, but regex locations take priority over prefix.
**How to avoid:** Use `location /api` (prefix match) -- nginx matches the longer prefix `/api` before the shorter `/`. This is the correct nginx default behavior, so it works as expected. Just ensure no regex location interferes.
**Warning signs:** API requests returning index.html content instead of JSON.

### Pitfall 5: Docker Network DNS Resolution
**What goes wrong:** Backend can't connect to `postgres` hostname, or frontend can't proxy to `backend`.
**Why it happens:** Containers not on the same Docker network, or service name mismatch.
**How to avoid:** All services in the same compose network. Use service names (`postgres`, `backend`) as hostnames in connection strings and nginx proxy_pass.
**Warning signs:** Connection refused errors in logs.

### Pitfall 6: Vue Frontend .dockerignore
**What goes wrong:** Docker COPY includes `node_modules/` (hundreds of MB) or `dist/` (stale build).
**Why it happens:** No `.dockerignore` in the Vue frontend directory.
**How to avoid:** Create `granjatech-frontend/.dockerignore` excluding `node_modules/`, `dist/`, `.env*`.
**Warning signs:** Extremely slow build context, or stale build artifacts used instead of fresh build.

## Code Examples

### docker-compose.yml Structure

```yaml
services:
  postgres:
    image: postgres:16-alpine
    container_name: granjatech-postgres
    restart: unless-stopped
    environment:
      POSTGRES_DB: GranjaTechDb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres123
      PGDATA: /var/lib/postgresql/data/pgdata
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./docs:/docker-entrypoint-initdb.d:ro
    networks:
      - granjatech-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  backend:
    build:
      context: ./granjatech-api
      dockerfile: Dockerfile
    container_name: granjatech-backend
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://postgres:postgres123@postgres:5432/GranjaTechDb
      - JWT_KEY=74b9f1d2-a3e9-4f7c-a9d8-9e2c1a3b5d7e-granjatech-super-secret
      - JWT_ISSUER=GranjaTechAPI
      - JWT_AUDIENCE=GranjaTechApp
      - ALLOWED_ORIGINS=http://localhost:80;http://localhost:3000
      - SWAGGER_ENABLED=true
      - RUST_LOG=info
    depends_on:
      postgres:
        condition: service_healthy
    networks:
      - granjatech-network
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 60s

  frontend:
    build:
      context: ./granjatech-frontend
      dockerfile: Dockerfile
    container_name: granjatech-frontend
    restart: unless-stopped
    ports:
      - "80:80"
    depends_on:
      - backend
    networks:
      - granjatech-network
    healthcheck:
      test: ["CMD-SHELL", "wget --quiet --tries=1 --spider http://localhost:80/health || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

networks:
  granjatech-network:
    driver: bridge
    name: granjatech-network

volumes:
  postgres_data:
    name: granjatech-postgres-data
```

### Vue Dockerfile

```dockerfile
FROM node:20-alpine AS build
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci --silent
COPY . .
ENV VITE_API_URL=/api
RUN npm run build

FROM nginx:alpine AS production
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:80/health || exit 1
CMD ["nginx", "-g", "daemon off;"]
```

### Non-Root User in Rust Runtime (discretion recommendation: YES, add it)

```dockerfile
# In the runtime stage:
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser
```

## Existing Assets Analysis

### What to Reuse (adapted)
| Asset | Location | Adaptation Needed |
|-------|----------|-------------------|
| PostgreSQL service config | `docker-compose.yml` | Minimal -- same image, same volumes, update password to match |
| nginx security headers | `frontend/nginx.conf` | Copy headers block, add proxy locations |
| Healthcheck patterns | `docker-compose.yml` | Same `pg_isready`, `curl`, `wget` patterns |
| Network topology | `docker-compose.yml` | Same bridge network, same service names |

### What to Replace
| Old | New | Reason |
|-----|-----|--------|
| `Dockerfile` (.NET) | `granjatech-api/Dockerfile` (Rust) | Different build toolchain |
| `frontend/Dockerfile` (React) | `granjatech-frontend/Dockerfile` (Vue) | Different build output (`dist` vs `build`) |
| `frontend/nginx.conf` | `granjatech-frontend/nginx.conf` | Adds reverse proxy blocks |
| `docker-compose.yml` backend/frontend services | Updated services | Different build contexts, env vars |

### Key Differences from .NET Stack

| Aspect | .NET Stack | Rust Stack |
|--------|-----------|------------|
| Build context | Root directory (`.`) | `./granjatech-api` |
| Build time | ~30s (.NET restore + build) | ~5min first build, ~30s cached (cargo-chef) |
| Runtime image | `mcr.microsoft.com/dotnet/aspnet:8.0` | `debian:bookworm-slim` |
| Binary | `dotnet GranjaTech.Api.dll` | `./granjatech-api` (native binary) |
| Frontend build output | `build/` (CRA) | `dist/` (Vite) |
| API URL env var | `REACT_APP_API_URL` | `VITE_API_URL` |
| CORS approach | Cross-origin (port 3000 -> 5099) | Same-origin via nginx proxy |
| Backend port (external) | 5099 | 8080 |
| Frontend port (external) | 3000 | 80 |

## Verified Codebase Facts

| Fact | Source | Impact |
|------|--------|--------|
| Rust backend binds to `0.0.0.0:8080` | `main.rs:252` | No changes needed for Docker |
| Health endpoint at `/health` returns JSON `{"status":"ok"}` | `main.rs:232-238` | DOCK-04 already implemented |
| Swagger at `/swagger-ui/{_:.*}` and `/api-docs/openapi.json` | `main.rs:245-246` | Proxy both paths in nginx |
| API base URL uses `VITE_API_URL` with fallback | `api.ts:3` | Must set `VITE_API_URL=/api` at build time |
| Vite outputs to `dist/` | `granjatech-frontend/dist/` exists | Use `dist` not `build` in Dockerfile |
| Vue build command: `vue-tsc --noEmit && vite build` | `package.json` scripts | TypeScript check + Vite build |
| Rust env vars: DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE, ALLOWED_ORIGINS, SWAGGER_ENABLED, RUST_LOG | `.env` file | All must be in docker-compose environment block |
| Existing compose uses `postgres123` password | `docker-compose.yml` | Keep consistent in new compose |
| `docs/` directory mounted as init scripts | `docker-compose.yml` volume | Keep `./docs:/docker-entrypoint-initdb.d:ro` |
| No `sqlx::query!` macros used (0 matches) | `grep sqlx::query!` | No compile-time DB needed -- Docker build is safe |
| 165 runtime `sqlx::query` calls across 17 files | `grep sqlx::query` | All queries are runtime-checked, no offline cache needed |
| `package-lock.json` exists in `granjatech-frontend/` | `ls` command | `npm ci` will work in Docker build |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Docker Compose + manual E2E |
| Config file | `docker-compose.yml` |
| Quick run command | `docker compose up -d && sleep 5 && curl http://localhost:8080/health && curl http://localhost:80` |
| Full suite command | `docker compose up --build -d && docker compose ps && curl -f http://localhost:8080/health && curl -f http://localhost:80/health` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOCK-01 | Rust Dockerfile builds successfully | smoke | `docker compose build backend` | Wave 0 |
| DOCK-02 | Vue Dockerfile builds successfully | smoke | `docker compose build frontend` | Wave 0 |
| DOCK-03 | All three containers start and are healthy | smoke | `docker compose up -d && docker compose ps` | Wave 0 |
| DOCK-04 | /health returns 200 OK | smoke | `curl -f http://localhost:8080/health` | Already in backend code |

### Sampling Rate
- **Per task commit:** `docker compose build <service>` (verify image builds)
- **Per wave merge:** `docker compose up --build -d && docker compose ps` (full stack up)
- **Phase gate:** Full stack running, health checks passing, frontend accessible, API proxied correctly

### Wave 0 Gaps
- [ ] `granjatech-api/Dockerfile` -- Rust multi-stage build (DOCK-01)
- [ ] `granjatech-frontend/Dockerfile` -- Vue multi-stage build (DOCK-02)
- [ ] `granjatech-frontend/nginx.conf` -- nginx config with reverse proxy (DOCK-02)
- [ ] `docker-compose.yml` -- Updated for Rust+Vue stack (DOCK-03)

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Docker | All containers | Yes | 29.3.1 | -- |
| Docker Compose | Orchestration | Yes | v5.1.1 | -- |
| PostgreSQL | Database | Via container | 16-alpine | -- |
| curl | Backend healthcheck | In debian:bookworm-slim | -- | Install in Dockerfile |

**Missing dependencies with no fallback:** None -- all requirements met.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No (handled by backend code) | -- |
| V3 Session Management | No | -- |
| V4 Access Control | No | -- |
| V5 Input Validation | No (infrastructure only) | -- |
| V6 Cryptography | No | -- |
| V14 Configuration | Yes | Non-root user, no secrets in images, env var injection |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Secrets baked into Docker image | Information Disclosure | Inject via docker-compose environment, never COPY .env; use .dockerignore |
| Container running as root | Elevation of Privilege | Non-root user in runtime stage (discretion item -- recommend YES) |
| Exposed unnecessary ports | Information Disclosure | Only expose 80 (frontend) and optionally 8080 (backend) |
| Outdated base images | All | Use specific tags, rebuild periodically |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Alpine + musl causes issues with SQLx/OpenSSL in Rust | Anti-Patterns | Low -- D-01 already locks Debian, this just explains why |
| A2 | nginx prefix location matching works correctly for /api before / | Pitfall 4 | Low -- standard nginx behavior, well-documented |
| A3 | cargo-chef `cargo install` works in rust:1-slim-bookworm without extra deps | Code Examples | Medium -- may need `gcc`/`make` for building cargo-chef from source |

## Open Questions

1. **Old .NET Docker files cleanup**
   - What we know: `Dockerfile` (root) and `frontend/Dockerfile` are for .NET/React stack
   - What's unclear: Whether to remove or keep them (discretion item)
   - Recommendation: Keep them for now -- they document the original stack. Optionally add a comment header noting they are legacy. Cleanup can happen in a future housekeeping pass.

~~2. SQLx compile-time macros~~ **RESOLVED:** No `query!` macros found (0 matches). All 165 SQLx calls use runtime `sqlx::query`/`sqlx::query_as`. No compile-time database or offline cache needed. [VERIFIED: codebase grep]

~~3. package-lock.json~~ **RESOLVED:** `granjatech-frontend/package-lock.json` exists. `npm ci` will work. [VERIFIED: ls command]

## Sources

### Primary (HIGH confidence)
- Existing codebase files: `docker-compose.yml`, `Dockerfile`, `frontend/Dockerfile`, `frontend/nginx.conf` -- direct templates
- `granjatech-api/src/main.rs` -- verified health endpoint, bind address, swagger paths
- `granjatech-frontend/src/services/api.ts` -- verified VITE_API_URL usage
- `granjatech-api/.env` -- verified environment variable names
- `granjatech-api/Cargo.toml` -- verified project name `granjatech-api` (binary name)
- SQLx usage pattern verified via grep (0 compile-time macros, 165 runtime calls)
- `package-lock.json` existence verified via ls

### Secondary (MEDIUM confidence)
- cargo-chef documentation at https://github.com/LukeMathWalker/cargo-chef -- standard Rust Docker caching approach

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all tools already installed and verified, Docker images are standard
- Architecture: HIGH -- direct templates exist in codebase, decisions are locked and specific
- Pitfalls: HIGH -- based on verified codebase facts (Vite output dir, env var names, bind address, SQLx pattern)

**Research date:** 2026-04-08
**Valid until:** 2026-05-08 (stable infrastructure, no fast-moving dependencies)
