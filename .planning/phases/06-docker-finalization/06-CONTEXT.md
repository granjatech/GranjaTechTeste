# Phase 6: Docker & Finalization - Context

**Gathered:** 2026-04-08
**Status:** Ready for planning

<domain>
## Phase Boundary

Containerizar o stack migrado completo (PostgreSQL 16 + Rust/Actix-web backend + Vue 3/Vite frontend) via Docker Compose. Substituir os Dockerfiles existentes (.NET + React) por novos (Rust + Vue/nginx com proxy reverso). O resultado final: `docker-compose up` levanta toda a aplicação funcional.

</domain>

<decisions>
## Implementation Decisions

### Dockerfile Rust Backend
- **D-01:** Multi-stage build com `rust:1-slim-bookworm` (build) + `debian:bookworm-slim` (runtime). Instalar `pkg-config libssl-dev` no build, `ca-certificates libssl3` no runtime.
- **D-02:** Usar `cargo-chef` para cache de dependências Rust no Docker. Três estágios: planner (gera recipe), cacher (compila deps), builder (compila app).
- **D-03:** Imagem final expõe porta 8080. Binário único copiado do builder.

### Dockerfile Vue Frontend
- **D-04:** Multi-stage build com `node:20-alpine` (build) + `nginx:alpine` (runtime). Mesmo padrão do React Dockerfile atual, mas usando `npm run build` do Vite.
- **D-05:** `VITE_API_URL=/api` fixado no Dockerfile (hardcoded). Com proxy reverso no nginx, não precisa de variável externa — tudo passa pela mesma origem.

### Nginx e Proxy Reverso
- **D-06:** nginx.conf com proxy reverso: `location /api { proxy_pass http://backend:8080; }`. Frontend e API acessíveis pela mesma porta 80. Elimina necessidade de CORS cross-origin.
- **D-07:** Proxy também para `/swagger` para acessar documentação via mesma porta.
- **D-08:** Manter security headers (X-Frame-Options, X-Content-Type-Options), gzip, cache de assets estáticos, e SPA routing (`try_files $uri /index.html`) do nginx.conf existente.

### Docker Compose (Produção)
- **D-09:** Portas externas: 8080 (backend direto, opcional) e 80 (frontend + proxy). Alterar das portas .NET atuais (5099 + 3000) para portas mais convencionais.
- **D-10:** PostgreSQL mantém porta 5432 e mesma configuração (imagem postgres:16-alpine, volume nomeado, healthcheck pg_isready).
- **D-11:** Backend depende de PostgreSQL (condition: service_healthy). Frontend depende de backend.
- **D-12:** Variáveis de ambiente do Rust backend injetadas via docker-compose: DATABASE_URL, JWT_KEY, JWT_ISSUER, JWT_AUDIENCE, ALLOWED_ORIGINS, SWAGGER_ENABLED.

### Docker Compose (Desenvolvimento)
- **D-13:** docker-compose.dev.yml mantém apenas PostgreSQL + pgAdmin (sem containers de app). Dev roda Rust via `cargo run` e Vue via `npm run dev` localmente com hot-reload nativo.

### Claude's Discretion
- Health check implementation details (interval, timeout, retries)
- Exact cargo-chef recipe structure
- Whether to keep the old .NET Dockerfile or remove it (cleanup)
- Non-root user configuration in runtime image

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Existing Docker Configuration (reference for structure/patterns)
- `docker-compose.yml` — Current .NET/React compose (structure to replicate with new stack)
- `docker-compose.dev.yml` — Dev compose with PostgreSQL + pgAdmin (keep as-is)
- `Dockerfile` — Current .NET multi-stage build (replace with Rust equivalent)
- `frontend/Dockerfile` — Current React multi-stage build (reference for Vue equivalent)
- `frontend/nginx.conf` — Current nginx config (adapt for Vue + add proxy reverso)

### Rust Backend
- `granjatech-api/Cargo.toml` — Dependencies and project name for Docker build
- `granjatech-api/.env` — Environment variables format (DATABASE_URL, JWT_*)

### Vue Frontend
- `granjatech-frontend/vite.config.ts` — Vite config (port 5173 dev, build output)
- `granjatech-frontend/package.json` — Build scripts

### Migration Plan
- `plano-migracao-granjatech.md` — Original migration plan with Docker section

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `frontend/nginx.conf` — Base nginx config with gzip, security headers, SPA routing, health endpoint. Adapt for Vue and add proxy reverso blocks.
- `docker-compose.yml` — Network topology (bridge network, named volumes, healthchecks). Same pattern applies to new stack.
- `docker-compose.dev.yml` — Kept unchanged; already serves dev PostgreSQL + pgAdmin.

### Established Patterns
- Multi-stage Docker builds (both .NET and React use this pattern)
- Healthchecks in docker-compose (pg_isready for DB, curl for backend, wget for frontend)
- Named volumes for PostgreSQL data persistence
- Bridge network for inter-container communication

### Integration Points
- `granjatech-api/` directory → new Rust Dockerfile context
- `granjatech-frontend/` directory → new Vue Dockerfile context
- PostgreSQL service stays identical (same image, same seed data from `docs/`)
- Backend reads `DATABASE_URL=postgres://postgres:PASSWORD@postgres:5432/GranjaTechDb` (host=postgres via Docker network)

</code_context>

<specifics>
## Specific Ideas

- Proxy reverso no nginx elimina CORS — frontend e API na mesma origem (porta 80)
- cargo-chef para cache de dependências Rust (rebuild rápido quando só código muda)
- debian-slim escolhido por compatibilidade com SQLx/openssl sem complicações de musl

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 06-docker-finalization*
*Context gathered: 2026-04-08*
