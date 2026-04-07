# Phase 2: Complete Backend CRUD - Research

**Researched:** 2026-04-07
**Domain:** Rust (Actix-web + SQLx) CRUD endpoints -- migrating remaining .NET controllers
**Confidence:** HIGH

## Summary

Phase 2 implements all remaining CRUD endpoints in the Rust backend to achieve full parity with the .NET system. The codebase is well-prepared: all 16 entity models and all 14 DTO modules already exist from Phase 1. The established patterns (GranjaService + handlers) provide a clear template. The work is primarily replication -- translating .NET service logic and controller routing into the established Rust patterns.

The main complexity areas are: (1) Lote calculated properties (viabilidade, IEP, conversao alimentar) which require fetching related ConsumoRacao and PesagemSemanal data, (2) Dashboard aggregation queries translating EF Core LINQ to raw SQL, (3) Financas 5-minute edit window business rule, and (4) computed properties in response DTOs (ConsumoPorAveGramas, ConsumoPorAveMl, GanhoMedioDiario) that must be calculated in Rust code since they are C# getters in .NET.

**Primary recommendation:** Follow the Phase 1 GranjaService/handler pattern exactly. Each service is a stateless struct with static async methods taking `&PgPool`. Computed properties go in service-layer mapping functions, not in models.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Split into 3 plans by complexity:
  - Plan 1: Lotes (complex, calculated properties, mortalidade) + Dashboard (aggregation queries)
  - Plan 2: Financas + Consumo + Pesagem + Sanitario (standard CRUD, lote-dependent)
  - Plan 3: Sensores + Estoque + Auditoria + Profile (simple CRUD, read-only, independent)
- **D-02:** IEP, conversao alimentar, and viabilidade computed in Rust code after fetching raw data, matching the .NET pattern where `Lote` entity has calculated properties (`CalcularIEP()`, `Viabilidade` getter). Not computed in SQL.
- **D-03:** Explicit `AuditoriaService::registrar_log()` calls in each service method after CUD operations, same as Phase 1 `GranjaService` pattern. No abstraction layer or auto-logging middleware.
- **D-04:** All Phase 1 decisions (D-01 through D-08) carry forward unchanged:
  - Single crate, flat module layout
  - Runtime SQLx queries with `query_as` and `.bind()`
  - Portuguese naming matching DB columns
  - Simple `AppError` enum with 5 variants
  - JWT claims matching .NET format (nameid, email, role)
  - Stateless service structs, `PgPool` passed as parameter
  - Normalized API responses (201 for creates, proper HTTP semantics)
  - `.env` config via dotenvy

### Claude's Discretion
- Internal service method organization within each file
- Query optimization for Dashboard aggregation endpoints
- DTO field ordering and grouping within struct definitions
- Handler grouping in `main.rs` route configuration

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| LOTE-01 | Listar lotes filtrado por role | Role-filtering pattern from GranjaService; JOIN through Granjas for Financeiro |
| LOTE-02 | Buscar lote por ID | get_by_id pattern with access check |
| LOTE-03 | Criar lote | Sequential code generation (LT-XXX), QuantidadeAvesAtual = QuantidadeAvesInicial |
| LOTE-04 | Atualizar lote | Delta logic for QuantidadeAvesInicial changes affecting QuantidadeAvesAtual |
| LOTE-05 | Deletar lote | Standard delete with Financeiro block |
| LOTE-06 | Registrar mortalidade | Clamp quantity to QuantidadeAvesAtual, decrement, audit log |
| LOTE-07 | Listar mortalidades | Sub-resource query with computed fields (IdadeDias, PercentualMortalidadeDia) |
| LOTE-08 | Calculated properties | Viabilidade, IEP, ConversaoAlimentar computed in Rust service layer |
| DASH-01 | KPIs | Aggregation SQL: SUM entradas/saidas, COUNT lotes ativos, role-filtered |
| DASH-02 | Resumo mensal | GROUP BY year/month with pt-BR month formatting |
| FINA-01 | Listar transacoes | Role-filtered via Lote->Granja->UsuarioId join chain |
| FINA-02 | Criar transacao | Set UsuarioId from claims, handle LoteId=0 as NULL |
| FINA-03 | Atualizar transacao | 5-minute edit window for non-Admin; Financeiro cannot edit Admin's transactions |
| FINA-04 | Deletar transacao | Admin-only delete |
| CONS-01 | Registrar consumo racao | Validate lote active, aves_vivas <= quantidade_atual |
| CONS-02 | Registrar consumo agua | Same validation as racao |
| CONS-03 | Listar consumo racao | Filter by loteId with optional date range, compute ConsumoPorAveGramas |
| CONS-04 | Listar consumo agua | Filter by loteId with optional date range, compute ConsumoPorAveMl |
| CONS-05 | Resumo consumo | Aggregate totals, averages, last record dates |
| PESA-01 | Registrar pesagem | Validate lote active |
| PESA-02 | Listar pesagens | Order by SemanaVida, compute GanhoMedioDiario |
| PESA-03 | Resumo pesagens | Growth curve data, current weight, uniformity |
| SANI-01 | Registrar evento sanitario | Validate lote active |
| SANI-02 | Listar eventos sanitarios | Filter by loteId with optional tipoEvento filter |
| SANI-03 | Resumo sanitario | Aggregation by type, cost per bird, upcoming actions |
| SANI-04 | Cronograma vacinacao | Static hardcoded data, no DB query |
| SENS-01 | Listar sensores | Role-filtered via Granja join (Admin/Produtor only) |
| SENS-02 | Criar sensor | Unique IdentificadorUnico check, granja permission |
| SENS-03 | Deletar sensor | Permission check via granja ownership |
| SENS-04 | Listar leituras sensor | Permission check, limit 100, order by timestamp desc |
| SENS-05 | Registrar leitura | Public endpoint, lookup sensor by IdentificadorUnico |
| ESTO-01 | Listar estoque | Role-filtered via Granja join (Admin/Produtor only) |
| ESTO-02 | Criar produto | Granja permission check |
| ESTO-03 | Atualizar produto | Granja ownership permission check |
| ESTO-04 | Deletar produto | Granja ownership permission check |
| AUDI-01 | Listar audit logs | Admin-only, return all logs |
| AUDI-02 | Registrar acoes | Already implemented (AuditoriaService), ensure called from all CUD ops |
| PERF-01 | Ver perfil | Fetch user + perfil + associados (Financeiro/Produtor relationships) |
| PERF-02 | Editar perfil | Duplicate email check, audit log |
| PERF-03 | Trocar senha | BCrypt verify old + hash new |
</phase_requirements>

## Standard Stack

### Core (already in Cargo.toml)
| Library | Version | Purpose | Status |
|---------|---------|---------|--------|
| actix-web | 4 | HTTP framework | Already configured [VERIFIED: Cargo.toml] |
| sqlx | 0.8 | PostgreSQL async queries | Already configured [VERIFIED: Cargo.toml] |
| serde/serde_json | 1 | JSON serialization | Already configured [VERIFIED: Cargo.toml] |
| validator | 0.20 | DTO validation | Already configured [VERIFIED: Cargo.toml] |
| chrono | 0.4 | Date/time handling | Already configured [VERIFIED: Cargo.toml] |
| rust_decimal | 1 | Decimal math | Already configured [VERIFIED: Cargo.toml] |
| bcrypt | 0.17 | Password hashing (PERF-03) | Already configured [VERIFIED: Cargo.toml] |
| utoipa | 5 | OpenAPI annotations | Already configured [VERIFIED: Cargo.toml] |
| tracing | 0.1 | Structured logging | Already configured [VERIFIED: Cargo.toml] |

**No new dependencies needed.** All required crates are already in `Cargo.toml`.

## Architecture Patterns

### Established Project Structure (from Phase 1)
```
granjatech-api/src/
├── main.rs              # Route registration, Swagger, server setup
├── config.rs            # Environment config
├── db.rs                # PgPool creation
├── errors.rs            # AppError enum (5 variants)
├── models/              # 16 entity structs (all exist)
│   ├── mod.rs
│   ├── lote.rs, granja.rs, transacao_financeira.rs, ...
├── dto/                 # 14 DTO modules (all exist)
│   ├── mod.rs
│   ├── lote.rs, dashboard.rs, financeiro.rs, ...
├── services/            # Business logic (3 exist, ~10 new needed)
│   ├── mod.rs
│   ├── granja_service.rs    # TEMPLATE
│   ├── auth_service.rs
│   ├── auditoria_service.rs # Called by all CUD operations
├── handlers/            # HTTP handlers (2 exist, ~10 new needed)
│   ├── mod.rs
│   ├── granjas.rs           # TEMPLATE
│   ├── auth.rs
├── middleware/
│   └── jwt.rs           # Claims extractor
```

### New Files to Create

**Services (in `services/`):**
- `lote_service.rs` -- Lotes CRUD + mortalidade + computed properties
- `dashboard_service.rs` -- KPIs + monthly summary aggregation
- `financas_service.rs` -- Transactions CRUD with business rules
- `consumo_service.rs` -- Feed/water consumption CRUD + summary
- `pesagem_service.rs` -- Weighing CRUD + growth summary
- `sanitario_service.rs` -- Sanitary events + summary + vaccination schedule
- `sensor_service.rs` -- Sensors CRUD + readings
- `estoque_service.rs` -- Stock products CRUD
- `profile_service.rs` -- Profile view/edit + password change

**Handlers (in `handlers/`):**
- `lotes.rs` -- 7 endpoints (CRUD + mortalidade x2)
- `dashboard.rs` -- 2 endpoints (KPIs + resumo mensal)
- `financas.rs` -- 4 endpoints (CRUD)
- `consumo.rs` -- 5 endpoints (racao create/list + agua create/list + resumo)
- `pesagem.rs` -- 3 endpoints (create + list + resumo)
- `sanitario.rs` -- 4 endpoints (create + list + resumo + cronograma)
- `sensores.rs` -- 4 endpoints (list + create + delete + leituras)
- `leituras.rs` -- 1 endpoint (post leitura, public)
- `estoque.rs` -- 4 endpoints (CRUD)
- `auditoria.rs` -- 1 endpoint (list, admin-only)
- `profile.rs` -- 3 endpoints (get + update + change-password)

### Pattern: Service Struct (FOLLOW EXACTLY)
```rust
// Source: granjatech-api/src/services/granja_service.rs [VERIFIED: codebase]
pub struct LoteService;

impl LoteService {
    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<LoteResponseDto>, AppError> {
        // 1. Role-filtered SQL query
        // 2. Map raw model to response DTO with computed fields
        // 3. Return
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateLoteDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<LoteResponseDto, AppError> {
        // 1. Permission check
        // 2. Business logic (generate code, set defaults)
        // 3. INSERT with RETURNING
        // 4. AuditoriaService::registrar_log(pool, user_id, user_email, ...)
        // 5. Return DTO
    }
}
```

### Pattern: Handler (FOLLOW EXACTLY)
```rust
// Source: granjatech-api/src/handlers/granjas.rs [VERIFIED: codebase]
#[utoipa::path(
    get,
    path = "/api/lotes",
    responses(
        (status = 200, description = "Lista de lotes", body = Vec<LoteResponseDto>),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn get_lotes(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let lotes = LoteService::get_all(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(lotes))
}
```

### Pattern: Route Registration
```rust
// Source: granjatech-api/src/handlers/mod.rs [VERIFIED: codebase]
// Add new scopes inside configure_routes
.service(
    web::scope("/lotes")
        .route("", web::get().to(lotes::get_lotes))
        .route("/{id}", web::get().to(lotes::get_lote))
        .route("", web::post().to(lotes::create_lote))
        .route("/{id}", web::put().to(lotes::update_lote))
        .route("/{id}", web::delete().to(lotes::delete_lote))
        .route("/{id}/mortalidades", web::post().to(lotes::registrar_mortalidade))
        .route("/{id}/mortalidades", web::get().to(lotes::listar_mortalidades)),
)
```

### Anti-Patterns to Avoid
- **Returning raw models as JSON:** Always map to response DTOs with computed fields. The .NET system returns entities with computed C# getters; Rust must compute these in service layer.
- **Forgetting audit log:** Every CUD operation must call `AuditoriaService::registrar_log()`. Check .NET source for exact action strings.
- **SQL column aliasing with sqlx(rename):** In Phase 1, SQL aliases (like `g."Id" AS "GranjaId"`) conflicted with `sqlx(rename)` on structs. Always SELECT columns matching the exact PascalCase DB column names and let `sqlx(rename)` on the model struct handle mapping. [VERIFIED: git log commit 86d76bf]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Decimal arithmetic | Float math | `rust_decimal::Decimal` | Financial calculations need exact precision |
| Password hashing | Custom hash | `bcrypt::verify` / `bcrypt::hash` | Must be compatible with .NET BCrypt hashes |
| Date difference | Manual day calculation | `chrono::Utc::now() - data_entrada` `.num_days()` | Handles timezone, leap years correctly |
| pt-BR month format | Manual month names | `chrono` format or hardcoded pt-BR map | DASH-02 needs "jan/26" format |
| Role-based query filtering | Manual if/else per endpoint | Extract pattern from GranjaService | Consistent role-filtering across all services |

## Common Pitfalls

### Pitfall 1: Lote Calculated Properties Need Related Data
**What goes wrong:** Trying to compute IEP or ConversaoAlimentar from just the Lote row
**Why it happens:** These calculations need ConsumosRacao and PesagensSemanais data
**How to avoid:** For LoteResponseDto list endpoints, compute only simple properties (viabilidade, idade_dias, densidade_atual) which use only Lote columns. IEP and CA are only needed for detailed views or avicultura endpoints (Phase 3).
**Warning signs:** Service returning 0 for all computed metrics

### Pitfall 2: Financas 5-Minute Edit Window
**What goes wrong:** Forgetting the time-based edit restriction or computing it wrong
**Why it happens:** .NET uses `DateTime.UtcNow - TimestampCriacao > TimeSpan.FromMinutes(5)`
**How to avoid:** In Rust: `Utc::now() - transacao.timestamp_criacao > chrono::Duration::minutes(5)`. Admin bypasses this check entirely. Also must check that Financeiro cannot edit Admin-created transactions (requires JOIN to get creator's role).
**Warning signs:** All edits succeeding or failing regardless of time

### Pitfall 3: Mortalidade Quantity Clamping
**What goes wrong:** Allowing negative QuantidadeAvesAtual
**Why it happens:** Not clamping mortalidade quantity to min(requested, current_aves)
**How to avoid:** `let qtd = dto.quantidade.min(lote.quantidade_aves_atual);` then subtract from lote
**Warning signs:** Negative bird counts in database

### Pitfall 4: Lote Update Delta Logic
**What goes wrong:** QuantidadeAvesAtual getting wrong value after QuantidadeAvesInicial changes
**Why it happens:** .NET applies a delta: if initial changes by +10, atual also increases by +10 (if result >= 0)
**How to avoid:** Replicate exactly: `delta = new_initial - old_initial; if (atual + delta >= 0) { atual += delta; }`
**Warning signs:** Bird counts drifting from expected values

### Pitfall 5: ConsumoPorAveGramas / ConsumoPorAveMl Computed Fields
**What goes wrong:** Returning 0 or omitting computed fields from list responses
**Why it happens:** .NET entities have C# getter properties (`ConsumoPorAveGramas => AvesVivas > 0 ? (QuantidadeKg * 1000) / AvesVivas : 0`). Rust models don't have these.
**How to avoid:** Compute in service layer when mapping to response DTOs:
- `consumo_por_ave_gramas = if aves_vivas > 0 { (quantidade_kg * 1000) / aves_vivas } else { Decimal::ZERO }`
- `consumo_por_ave_ml = if aves_vivas > 0 { (quantidade_litros * 1000) / aves_vivas } else { Decimal::ZERO }`
- `ganho_medio_diario = ganho_semanal.map(|g| g / 7).unwrap_or(Decimal::ZERO)`

### Pitfall 6: Leituras Endpoint is Public
**What goes wrong:** Adding JWT auth to POST /api/leituras
**Why it happens:** Assuming all endpoints need auth
**How to avoid:** The .NET LeiturasController has NO `[Authorize]` attribute. POST /api/leituras is designed for IoT devices. Keep it public in Rust -- no Claims extractor.
**Warning signs:** IoT devices getting 401 errors

### Pitfall 7: Role Restrictions Vary Per Controller
**What goes wrong:** Applying uniform auth rules across all endpoints
**Why it happens:** Not checking each .NET controller's `[Authorize]` attributes carefully
**How to avoid:** Map from .NET source:
- **LotesController:** `[Authorize]` (any user), mortalidade routes `[Authorize(Roles = "Administrador,Produtor")]`
- **DashboardController:** `[Authorize]` (any user)
- **FinancasController:** `[Authorize]` class-level, but GET/POST/PUT are `[Authorize(Roles = "Administrador,Financeiro")]`, DELETE is `[Authorize(Roles = "Administrador")]`
- **ConsumoController:** `[Authorize(Roles = "Administrador,Produtor")]`
- **PesagemController:** `[Authorize(Roles = "Administrador,Produtor")]`
- **SanitarioController:** `[Authorize(Roles = "Administrador,Produtor")]`
- **SensoresController:** `[Authorize(Roles = "Administrador,Produtor")]`
- **LeiturasController:** No auth (public)
- **EstoqueController:** `[Authorize(Roles = "Administrador,Produtor")]`
- **AuditoriaController:** `[Authorize(Roles = "Administrador")]`
- **ProfileController:** `[Authorize]` (any user)
**Warning signs:** Wrong roles accessing endpoints

### Pitfall 8: Dashboard Monthly Summary pt-BR Formatting
**What goes wrong:** Returning English month names or wrong format
**Why it happens:** .NET uses `CultureInfo.GetCultureInfo("pt-BR")` with `"MMM/yy"` format
**How to avoid:** chrono doesn't have built-in pt-BR locale. Use a hardcoded month name map: `["jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"]` and format as `"{}/{:02}"` (e.g., "jan/26")
**Warning signs:** Frontend showing "Jan" instead of "jan"

## Code Examples

### Computed Properties for Lote Response
```rust
// [VERIFIED: .NET source GranjaTech.Domain/Lote.cs]
// These must be computed in Rust service layer, NOT stored in DB
fn map_lote_to_response(lote: &Lote) -> LoteResponseDto {
    let idade_atual_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days() as i32;
    let mortalidade_acumulada = lote.quantidade_aves_inicial - lote.quantidade_aves_atual;
    let pct_mortalidade = if lote.quantidade_aves_inicial > 0 {
        Decimal::from(mortalidade_acumulada) / Decimal::from(lote.quantidade_aves_inicial) * Decimal::from(100)
    } else {
        Decimal::ZERO
    };
    let viabilidade = Decimal::from(100) - pct_mortalidade;
    let densidade_atual = match lote.area_galpao {
        Some(area) if area > Decimal::ZERO => Decimal::from(lote.quantidade_aves_atual) / area,
        _ => Decimal::ZERO,
    };

    LoteResponseDto {
        id: lote.id,
        codigo: lote.codigo.clone(),
        // ... all fields ...
        idade_atual_dias,
        viabilidade,
        densidade_atual,
    }
}
```

### Dashboard KPI Aggregation SQL
```rust
// [VERIFIED: .NET source DashboardService.cs]
// For Administrador (no filter):
let kpis = sqlx::query_as::<_, DashboardKpiRaw>(r#"
    SELECT
        COALESCE(SUM(CASE WHEN t."Tipo" = 'Entrada' THEN t."Valor" ELSE 0 END), 0) as total_entradas,
        COALESCE(SUM(CASE WHEN t."Tipo" = 'Saida' THEN t."Valor" ELSE 0 END), 0) as total_saidas,
        (SELECT COUNT(*) FROM "Lotes" WHERE "DataSaida" IS NULL) as lotes_ativos
    FROM "TransacoesFinanceiras" t
"#).fetch_one(pool).await?;
// Lucro = total_entradas - total_saidas (computed in Rust)
```

### Mortalidade with Computed Fields
```rust
// [VERIFIED: .NET source RegistroMortalidade.cs + LotesController.cs]
// When returning mortalidade records, compute:
// idade_dias = (mortalidade.data - lote.data_entrada).num_days()
// percentual_mortalidade_dia = mortalidade.quantidade / lote.quantidade_aves_inicial * 100
// These are NOT stored in DB -- they are [NotMapped] in .NET
```

### Role-Based Service Check (Financas pattern)
```rust
// [VERIFIED: .NET source FinancasService.cs]
// Financas has different role logic than other services:
// - GET/POST/PUT: Administrador + Financeiro
// - DELETE: Administrador only
// - Update has 5-minute window + hierarchy check
pub async fn update(
    pool: &PgPool,
    id: i32,
    dto: &UpdateTransacaoDto,
    user_id: i32,
    user_role: &str,
    user_email: &str,
) -> Result<(), AppError> {
    let transacao = /* fetch with user info */;

    // Rule 1: 5-minute edit window (Admin exempt)
    if user_role != "Administrador" {
        let elapsed = Utc::now() - transacao.timestamp_criacao;
        if elapsed > chrono::Duration::minutes(5) {
            return Err(AppError::BadRequest(
                "O tempo para edicao expirou. A transacao so pode ser editada nos primeiros 5 minutos.".into()
            ));
        }
    }

    // Rule 2: Financeiro cannot edit Admin's transactions
    if user_role == "Financeiro" && creator_role == "Administrador" {
        return Err(AppError::BadRequest(
            "Permissao negada. Um utilizador Financeiro nao pode editar uma transacao criada por um Administrador.".into()
        ));
    }
    // ... update fields ...
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| EF Core LINQ | Raw SQL via sqlx | Phase 1 decision | All queries hand-written; no ORM magic for joins/includes |
| C# computed getters | Rust service-layer functions | Phase 1 decision | Must explicitly compute every derived field |
| DI-injected services | Static struct methods | Phase 1 decision | No constructor injection; pool passed as parameter |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Lote list endpoint returns simple computed props only (viabilidade, idade, densidade) without IEP/CA | Pitfall 1 | If frontend expects IEP/CA in list view, need extra queries per lote |
| A2 | POST /api/leituras should remain public (no auth) matching .NET | Pitfall 6 | If it should be authenticated, IoT integration pattern changes |
| A3 | The `RegistroMortalidade.PercentualMortalidadeDia` and `IdadeDias` fields need Lote data to compute | Code Examples | If these are stored in DB, computation is unnecessary |

## Open Questions

1. **Lote list endpoint: which computed properties are returned?**
   - What we know: .NET returns full Lote entity including `IdadeAtualDias`, `Viabilidade`, `DensidadeAtual`, `MortalidadeTotalAcumulada`, `PercentualMortalidadeAcumulada` -- all computable from Lote row alone. IEP and CA require related data (ConsumosRacao, PesagensSemanais).
   - What's unclear: Does the .NET list endpoint also eager-load and return IEP/CA? The EF includes are `Granja` + `Granja.Usuario` only, NOT consumos/pesagens.
   - Recommendation: For list view, return only Lote-row-computable fields. IEP/CA are Phase 3 (avicultura endpoints). [LOW risk -- .NET GetAllAsync only includes Granja, not consumption/weighing data]

2. **Financas update: how to get creator's role for hierarchy check?**
   - What we know: .NET uses `transacaoExistente.Usuario.Perfil.Nome` via Include chain
   - What's unclear: Need a JOIN query in SQLx to get the creator's profile name
   - Recommendation: Use `SELECT p."Nome" FROM "Usuarios" u JOIN "Perfis" p ON u."PerfilId" = p."Id" WHERE u."Id" = $1` to fetch creator role

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual HTTP testing (curl/httpie) against running server |
| Config file | None -- no automated test framework configured |
| Quick run command | `cargo build --manifest-path granjatech-api/Cargo.toml` |
| Full suite command | `cargo build --manifest-path granjatech-api/Cargo.toml && cargo test --manifest-path granjatech-api/Cargo.toml` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| LOTE-01..08 | Lote CRUD + mortalidade + computed props | manual | curl against running server | N/A |
| DASH-01..02 | Dashboard aggregation | manual | curl against running server | N/A |
| FINA-01..04 | Financas CRUD + business rules | manual | curl against running server | N/A |
| CONS-01..05 | Consumo CRUD + computed fields | manual | curl against running server | N/A |
| PESA-01..03 | Pesagem CRUD + growth summary | manual | curl against running server | N/A |
| SANI-01..04 | Sanitario CRUD + summary + schedule | manual | curl against running server | N/A |
| SENS-01..05 | Sensores CRUD + leituras | manual | curl against running server | N/A |
| ESTO-01..04 | Estoque CRUD | manual | curl against running server | N/A |
| AUDI-01..02 | Auditoria list + logging from all services | manual | curl against running server | N/A |
| PERF-01..03 | Profile view/edit/password | manual | curl against running server | N/A |

### Sampling Rate
- **Per task commit:** `cargo build --manifest-path granjatech-api/Cargo.toml` (compilation check)
- **Per wave merge:** Full compilation + manual smoke test of new endpoints
- **Phase gate:** All endpoints respond correctly with expected status codes

### Wave 0 Gaps
None -- no automated test infrastructure to set up. Validation is compilation success + manual HTTP testing per REQUIREMENTS.md "Out of Scope" which states: "Testes automatizados extensivos -- Verificacao manual de paridade suficiente para migracao"

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | Yes | JWT via `Claims` extractor (Phase 1) |
| V3 Session Management | No | Stateless JWT, no sessions |
| V4 Access Control | Yes | Role-based filtering in service layer per .NET parity |
| V5 Input Validation | Yes | `validator` crate on all input DTOs |
| V6 Cryptography | Yes (PERF-03) | `bcrypt` crate for password change |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| SQL injection | Tampering | Parameterized queries via sqlx `.bind()` [VERIFIED: established pattern] |
| Broken access control | Elevation of Privilege | Role-check in every service method before data access [VERIFIED: GranjaService pattern] |
| IDOR on lote/granja IDs | Information Disclosure | Ownership verification via Granja.UsuarioId chain [VERIFIED: .NET pattern] |
| Mass assignment | Tampering | Explicit DTO fields, no raw entity binding [VERIFIED: all DTOs use specific fields] |
| Password brute force | Spoofing | BCrypt cost factor provides rate limiting [ASSUMED -- same as .NET behavior] |

## Sources

### Primary (HIGH confidence)
- `granjatech-api/src/services/granja_service.rs` -- Established Rust service pattern
- `granjatech-api/src/handlers/granjas.rs` -- Established Rust handler pattern
- `granjatech-api/src/errors.rs` -- AppError enum
- `granjatech-api/src/middleware/jwt.rs` -- Claims extractor
- `granjatech-api/src/models/` -- All 16 entity models verified
- `granjatech-api/src/dto/` -- All 14 DTO modules verified
- All .NET controllers and services in `GranjaTech.Api/Controllers/` and `GranjaTech.Infrastructure/Services/Implementations/` -- Source of truth for behavior
- `GranjaTech.Domain/Lote.cs` -- Computed properties (Viabilidade, IEP, CA, DensidadeAtual)
- `GranjaTech.Domain/ConsumoRacao.cs` -- ConsumoPorAveGramas computed property
- `GranjaTech.Domain/ConsumoAgua.cs` -- ConsumoPorAveMl computed property
- `GranjaTech.Domain/PesagemSemanal.cs` -- GanhoMedioDiario computed property
- `GranjaTech.Domain/RegistroMortalidade.cs` -- IdadeDias, PercentualMortalidadeDia computed properties
- `granjatech-api/Cargo.toml` -- All dependencies verified present

### Secondary (MEDIUM confidence)
- None needed -- all information from codebase inspection

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all crates already in Cargo.toml, no new dependencies
- Architecture: HIGH -- patterns established and verified in Phase 1 codebase
- Pitfalls: HIGH -- all derived from direct .NET source code comparison
- Business logic: HIGH -- all rules traced to specific .NET service/controller methods

**Research date:** 2026-04-07
**Valid until:** 2026-05-07 (stable -- no external dependency changes expected)
