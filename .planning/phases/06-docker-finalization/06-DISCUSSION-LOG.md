# Phase 6: Docker & Finalization - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-08
**Phase:** 06-docker-finalization
**Areas discussed:** Estratégia de Dockerfile Rust, Mapeamento de portas e variáveis, Nginx e proxy para API, Docker Compose dev vs prod

---

## Estratégia de Dockerfile Rust

### Imagem base

| Option | Description | Selected |
|--------|-------------|----------|
| debian-slim | Multi-stage: rust:1-slim-bookworm build + debian:bookworm-slim runtime. ~80-100MB. | ✓ |
| Alpine | musl libc, ~30-50MB, possíveis problemas com sqlx/openssl | |
| Distroless | Máxima segurança, sem shell, difícil de debugar | |
| Você decide | Claude escolhe | |

**User's choice:** debian-slim (Recomendado)
**Notes:** Compatibilidade com SQLx e openssl foi o fator decisivo.

### Cache de dependências

| Option | Description | Selected |
|--------|-------------|----------|
| Dummy build | Copia Cargo.toml, cria main.rs vazio, compila deps. Simples. | |
| cargo-chef | Ferramenta dedicada para cache de deps Rust no Docker. Mais robusto. | ✓ |
| Sem cache | Copia tudo e compila. Rebuild completo a cada mudança. | |

**User's choice:** cargo-chef
**Notes:** Preferiu robustez do cargo-chef sobre simplicidade do dummy build.

---

## Mapeamento de portas e variáveis

### Portas externas

| Option | Description | Selected |
|--------|-------------|----------|
| Manter (5099 + 3000) | Mesmas portas do .NET/React. Compatibilidade com docs existentes. | |
| Alterar (8080 + 80) | Portas mais convencionais. Mais limpo. | ✓ |
| Você decide | Claude escolhe | |

**User's choice:** Alterar (8080 + 80)
**Notes:** Portas convencionais preferidas para o novo stack.

### Injeção de API URL no Vue build

| Option | Description | Selected |
|--------|-------------|----------|
| Build arg no Dockerfile | ARG VITE_API_URL passado via docker-compose. Baked no bundle. | ✓ |
| Nginx env substitution | envsubst em runtime. Mais flexível, mais complexo. | |
| Hardcoded relativo | Frontend chama '/api', nginx faz proxy. | |

**User's choice:** Build arg no Dockerfile (Recomendado)
**Notes:** Com proxy reverso decidido depois, VITE_API_URL será fixado como `/api` no Dockerfile.

---

## Nginx e proxy para API

| Option | Description | Selected |
|--------|-------------|----------|
| Sim, proxy reverso | nginx com location /api { proxy_pass backend:8080 }. Elimina CORS. | ✓ |
| Não, acesso direto | Frontend chama backend direto na 8080. Precisa de CORS. | |
| Você decide | Claude escolhe | |

**User's choice:** Sim, proxy reverso (Recomendado)
**Notes:** Eliminar CORS e simplificar configuração foi o motivador principal.

---

## Docker Compose dev vs prod

| Option | Description | Selected |
|--------|-------------|----------|
| Manter só PostgreSQL+pgAdmin | Dev compose fica como hoje. Hot-reload nativo para Rust e Vue. | ✓ |
| Adicionar hot-reload containers | Volume mounts + cargo-watch/vite dev. Mais complexo. | |
| Remover docker-compose.dev.yml | Só manter compose de produção. | |

**User's choice:** Manter só PostgreSQL+pgAdmin (Recomendado)
**Notes:** Hot-reload nativo via cargo run e npm run dev é preferido sobre containers de dev.

---

## Claude's Discretion

- Health check implementation details (interval, timeout, retries)
- Exact cargo-chef recipe structure
- Whether to keep/remove old .NET Dockerfile
- Non-root user configuration in runtime image

## Deferred Ideas

None — discussion stayed within phase scope
