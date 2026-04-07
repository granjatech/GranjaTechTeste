# Phase 3: Reports & Business Logic - Research

**Researched:** 2026-04-07
**Domain:** Rust/Actix-web avicultura analytics, report endpoints, moka cache
**Confidence:** HIGH

## Summary

Phase 3 migrates the most computation-heavy part of the GranjaTech system: 9 avicultura analytics endpoints, 6 report endpoints, a health check, and an in-memory cache layer. The .NET reference has both fully-implemented methods (IEP, CA, GMD, alerts, industry comparison) and stub methods returning defaults (growth curves, consumption analysis, sanitary summary, slaughter projection, weight estimation). The Rust implementation must mirror this exact behavior -- stubs return equivalent empty/zero responses.

All 14 avicultura DTOs and 8 report DTOs already exist in the Rust codebase (`dto/avicultura.rs`, `dto/relatorios.rs`, `dto/financeiro.rs`). The existing service pattern (stateless structs, PgPool parameter, AppError returns) and handler pattern (utoipa annotations, Claims extractor) are well-established from Phases 1-2. The primary challenge is translating complex multi-query business logic from EF Core (.Include() chains) into multiple sequential SQLx queries, and correctly wiring `moka::future::Cache` as a shared Actix-web app data resource.

**Primary recommendation:** Follow the existing service/handler patterns exactly. Implement avicultura service first (most complex logic), then reports (heavy SQL), then cache layer last (wraps existing endpoints). Use moka 0.12.x with `future::Cache<String, String>` storing JSON-serialized values, matching the .NET MemoryCacheService pattern.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Split into 3 plans: Plan 1 (Avicultura 9 endpoints), Plan 2 (Reports 6 endpoints), Plan 3 (Cache + Health + wiring)
- **D-02:** Mirror .NET individual compute method pattern -- separate service methods making own DB queries
- **D-03:** Industry benchmarks hardcoded as Rust constants
- **D-04:** Full 5-method CacheService API wrapping moka::future::Cache: get, set, remove, remove_by_pattern, get_or_set
- **D-05:** Cache TTLs: dashboard KPIs 5 min, avicultura dashboard 5 min, heavy reports 10 min
- **D-06:** TTL-only expiration, no write-through invalidation, no modifications to Phase 2 services
- **D-07:** Multiple queries per report (no large JOINs), matching EF Core .Include() pattern
- **D-08:** Health check only at /health, skip debug endpoints
- **D-09:** Role parity: Avicultura = Administrador+Produtor only, Reports = all 3 roles, no data filtering in reports, no ownership checks on avicultura loteId
- **D-10:** Typed DTOs for all composite responses (no serde_json::json!())
- **D-11:** Normalize errors to AppError enum, consistent with Phase 1 D-04
- **D-12:** All Phase 1/2 decisions carry forward (single crate, flat modules, runtime SQLx, Portuguese naming, etc.)

### Claude's Discretion
- Internal query optimization within multiple-query approach
- Moka cache configuration (max capacity, eviction policy)
- Service method ordering and grouping
- Report DTO field ordering/naming for anonymous .NET objects

### Deferred Ideas (OUT OF SCOPE)
None
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| AVIC-01 | Metricas do lote endpoint | AviculturaService methods: IEP, CA, GMD, Viabilidade, Uniformidade, Densidade |
| AVIC-02 | Analise de consumo endpoint | AnaliseConsumoDto already defined; .NET returns stub/empty |
| AVIC-03 | Curvas de crescimento endpoint | CurvasCrescimentoDto defined; .NET returns stub/empty |
| AVIC-04 | Resumo sanitario endpoint | ResumoSanitarioDto defined; .NET returns stub/empty |
| AVIC-05 | Alertas endpoint | Full .NET implementation: mortality, density, NH3, temperature checks |
| AVIC-06 | Comparacao industria endpoint | Full .NET implementation with 4 metrics + scoring |
| AVIC-07 | Projecao abate endpoint | ProjecaoAbateDto defined; .NET returns stub/empty |
| AVIC-08 | Estimar peso endpoint | .NET returns stub (0m) |
| AVIC-09 | Dashboard completo endpoint | Composite: metricas + alertas + comparacao + resumo + projecao |
| RELA-01 | Health check publico | Simple /health endpoint returning status JSON |
| RELA-02 | Relatorio financeiro simplificado | Multi-query with role-based filtering, simplified projections |
| RELA-03 | Relatorio financeiro completo | Complex multi-query with Include chains for lote/granja/usuario |
| RELA-04 | Relatorio de producao | Lote query with granja include, role-filtered |
| RELA-05 | Relatorio de avicultura | Heavy endpoint: lotes + all sub-entities, computed metrics |
| RELA-06 | Relatorio desempenho por lote | Single lote with all related entities, detailed breakdown |
| RELA-07 | Relatorio avancado com filtros | 3 tipos (financeiro/geral/setor) x 4 setores, granja-scoped |
| CACH-01 | Cache in-memory moka | CacheService wrapping moka::future::Cache with 5-method API |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| moka | 0.12.15 | In-memory async cache | Industry-standard Rust cache, Caffeine-inspired, `future::Cache` for async [VERIFIED: cargo search] |
| sqlx | 0.8.x | Database queries | Already in Cargo.toml, runtime queries with `query_as` [VERIFIED: Cargo.toml] |
| actix-web | 4.x | HTTP framework | Already in Cargo.toml [VERIFIED: Cargo.toml] |
| rust_decimal | 1.x | Decimal arithmetic | Already in use for financial/metric calculations [VERIFIED: Cargo.toml] |
| chrono | 0.4.x | Date/time handling | Already in use [VERIFIED: Cargo.toml] |
| serde_json | 1.x | JSON serialization for cache values | Already in use [VERIFIED: Cargo.toml] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| utoipa | 5.x | OpenAPI/Swagger docs | All new handlers need utoipa path annotations [VERIFIED: Cargo.toml] |

**Installation (add to Cargo.toml):**
```toml
moka = { version = "0.12", features = ["future"] }
```

## Architecture Patterns

### New Files to Create
```
granjatech-api/src/
├── services/
│   ├── avicultura_service.rs    # 9+ methods matching AviculturaService.cs
│   ├── relatorio_service.rs     # Financial + production reports
│   ├── relatorio_avancado_service.rs  # Advanced reports (financeiro/geral/setor)
│   └── cache_service.rs         # CacheService wrapping moka::future::Cache
├── handlers/
│   ├── avicultura.rs            # 9 handler functions
│   └── relatorios.rs            # 7 handler functions (6 reports + health)
└── (update) main.rs             # New routes + swagger + Cache app_data
```

### Pattern 1: Avicultura Service (Complex Business Logic)
**What:** Stateless service struct with methods that each make independent DB queries, compute metrics, return typed DTOs
**When to use:** For all 9 avicultura endpoints
**Example:**
```rust
// Source: existing lote_service.rs pattern + .NET AviculturaService.cs
pub struct AviculturaService;

/// Industry benchmark constants matching .NET PadroesIndustria dictionary
const CONVERSAO_ALIMENTAR_PADRAO: Decimal = dec!(1.75);
const CONVERSAO_ALIMENTAR_EXCELENCIA: Decimal = dec!(1.60);
const GMD_PADRAO: Decimal = dec!(55);
// ... etc

impl AviculturaService {
    pub async fn calcular_iep(pool: &PgPool, lote_id: i32) -> Result<Decimal, AppError> {
        // Fetch lote
        let lote = sqlx::query_as::<_, Lote>("SELECT ... FROM \"Lotes\" WHERE \"Id\" = $1")
            .bind(lote_id).fetch_optional(pool).await?;
        let lote = match lote { Some(l) => l, None => return Ok(Decimal::ZERO) };

        // Fetch most recent weighing
        let pesagem = sqlx::query_as::<_, PesagemSemanal>(
            "SELECT ... FROM \"PesagensSemanais\" WHERE \"LoteId\" = $1 ORDER BY \"DataPesagem\" DESC LIMIT 1"
        ).bind(lote_id).fetch_optional(pool).await?;
        // ... compute IEP matching .NET formula
    }
}
```

### Pattern 2: Report Service (Multi-Query Role-Filtered)
**What:** Report methods that fetch base entities then related data in separate queries, with role-based filtering per Claims
**When to use:** For all 6 report endpoints
**Example:**
```rust
// Source: existing dashboard_service.rs role-filtering pattern + .NET RelatorioService.cs
pub struct RelatorioService;

impl RelatorioService {
    pub async fn financeiro_simplificado(
        pool: &PgPool, user_id: i32, user_role: &str,
        data_inicio: DateTime<Utc>, data_fim: DateTime<Utc>,
        granja_id: Option<i32>,
    ) -> Result<RelatorioFinanceiroSimplificadoDto, AppError> {
        // Build query based on role (match user_role pattern from dashboard_service)
        // Execute, compute totals, return DTO
    }
}
```

### Pattern 3: Cache Service (moka wrapper)
**What:** CacheService struct holding `moka::future::Cache<String, String>` that stores JSON-serialized values
**When to use:** Wrap dashboard and report handler calls
**Example:**
```rust
// Source: .NET ICacheService interface + moka docs
use moka::future::Cache;
use std::time::Duration;

pub struct CacheService {
    cache: Cache<String, String>,
}

impl CacheService {
    pub fn new(max_capacity: u64) -> Self {
        Self {
            cache: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(Duration::from_secs(30 * 60)) // default 30 min
                .build(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.cache.get(key).await
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    pub async fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        if let Ok(json) = serde_json::to_string(value) {
            // Use insert with per-entry TTL via policy
            self.cache.insert(key.to_string(), json).await;
        }
    }

    pub async fn get_or_set<T, F, Fut>(&self, key: &str, f: F, ttl: Duration) -> Result<T, AppError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        if let Some(cached) = self.get::<T>(key).await {
            return Ok(cached);
        }
        let value = f().await?;
        self.set(key, &value, ttl).await;
        Ok(value)
    }

    pub async fn remove(&self, key: &str) {
        self.cache.remove(key).await;
    }

    pub async fn remove_by_pattern(&self, _pattern: &str) {
        // .NET implementation is also a no-op/warning for MemoryCache
        // moka doesn't support pattern removal natively
        tracing::warn!("remove_by_pattern not fully supported with moka");
    }
}
```

### Pattern 4: Avicultura Handler with Role Guard
**What:** Handler that rejects non-Administrador/Produtor roles before calling service
**When to use:** All 9 avicultura handlers
**Example:**
```rust
// Source: .NET AviculturaController [Authorize(Roles = "Administrador,Produtor")]
pub async fn get_metricas(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    // Role guard matching D-09
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor".into()));
    }
    let lote_id = path.into_inner();
    // call service methods...
}
```

### Anti-Patterns to Avoid
- **Large JOIN queries:** Decision D-07 explicitly requires multiple separate queries. Do NOT try to fetch lote + all sub-entities in a single JOIN.
- **serde_json::json!() for responses:** Decision D-10 requires typed DTOs. The .NET code uses anonymous objects in controllers -- these must become Rust structs.
- **Shared mutable state in service:** Services are stateless structs. Cache is shared via `web::Data<CacheService>`, not embedded in services.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| In-memory cache | Custom HashMap + TTL logic | moka::future::Cache | Thread-safe, async, automatic eviction, battle-tested |
| Decimal arithmetic | f64 for financial/metric values | rust_decimal::Decimal | Exact decimal representation, matches .NET decimal behavior |
| Per-entry TTL | Multiple Cache instances per TTL | Single Cache + per-entry expiry via `policy::EvictionPolicy` or separate caches per TTL group | moka 0.12 supports configurable TTL |

**Key insight:** moka 0.12's `time_to_live()` sets a global TTL per cache instance. For different TTLs (5 min vs 10 min per D-05), either (a) create separate Cache instances per TTL tier, or (b) use the `Expiry` trait for per-entry TTL. Separate instances is simpler and recommended for this use case. [ASSUMED]

## Common Pitfalls

### Pitfall 1: .NET Stub Methods Returning Defaults
**What goes wrong:** Implementing full business logic for methods that are stubs in .NET (returning `Task.FromResult(0m)` or empty DTOs)
**Why it happens:** The .NET AviculturaService has ~15 methods declared but only 5-6 are implemented (IEP, CA, GMD, alerts, industry comparison). The rest return defaults.
**How to avoid:** Check each method in AviculturaService.cs -- lines 354-367 are all stubs. Mirror these as Rust methods returning `Ok(Decimal::ZERO)` or empty DTO defaults.
**Warning signs:** Spending time writing complex SQL for a method that returns 0 in .NET

### Pitfall 2: Decimal vs f64 Mismatch in Reports
**What goes wrong:** Using f64 where Decimal is needed, or vice versa, causing compilation errors or precision loss
**Why it happens:** .NET reports mix `decimal` (financial values) with computed properties. The existing Rust DTOs use both `Decimal` and `f64`.
**How to avoid:** Follow existing DTO types exactly. Financial values = Decimal. Sensor/weight report values that already use f64 in DTOs (like `PesagemResumoDto.peso_medio_kg: f64`) stay as f64.
**Warning signs:** Compilation errors about type conversion between Decimal and f64

### Pitfall 3: PostgreSQL Column Name Casing
**What goes wrong:** SQLx query fails at runtime because column names don't match (PascalCase in DB, snake_case in Rust)
**Why it happens:** EF Core auto-maps PascalCase C# properties to PascalCase PostgreSQL columns. SQLx requires explicit column naming in queries.
**How to avoid:** Always use double-quoted PascalCase column names in SQL: `"LoteId"`, `"DataEntrada"`, etc. Use `sqlx(rename = "...")` on FromRow structs. Follow exact patterns from existing services.
**Warning signs:** "column not found" runtime errors

### Pitfall 4: Anonymous .NET Response Objects Need Typed DTOs
**What goes wrong:** The .NET avicultura controller's `GetMetricasLote` and `GetDashboardCompleto` use `new { ... }` anonymous objects. These need Rust struct equivalents.
**Why it happens:** C# anonymous objects serialize to JSON automatically. Rust requires explicit types.
**How to avoid:** Create `MetricasLoteDto` and `DashboardCompletoDto` structs. The existing `dto/avicultura.rs` may need additions for these composite response types.
**Warning signs:** Missing DTO types for controller endpoints that use anonymous objects in .NET

### Pitfall 5: Moka Cache Key Collisions
**What goes wrong:** Cache returns wrong data because key scheme doesn't distinguish different users/roles for role-filtered reports
**Why it happens:** If cache key is only `report_financeiro_{inicio}_{fim}`, different users see same cached result
**How to avoid:** Include user_id and user_role in cache keys for role-filtered endpoints: `report_financeiro_{user_id}_{inicio}_{fim}`. For avicultura endpoints (no role filtering per D-09), key on lote_id only.
**Warning signs:** Users seeing other users' report data

### Pitfall 6: Date Validation Parity
**What goes wrong:** Report endpoints accept invalid date ranges that .NET rejects
**Why it happens:** .NET controllers validate: dataInicio < dataFim, period <= 365 days, dataInicio not future
**How to avoid:** Replicate all date validations from RelatoriosController.cs in Rust handlers
**Warning signs:** Reports returning empty results for edge case dates

## Code Examples

### Industry Benchmark Constants (from .NET AviculturaService.cs lines 23-42)
```rust
// Source: GranjaTech.Infrastructure/Services/Implementations/AviculturaService.cs
use rust_decimal_macros::dec;

pub const CONVERSAO_ALIMENTAR_PADRAO: Decimal = dec!(1.75);
pub const CONVERSAO_ALIMENTAR_EXCELENCIA: Decimal = dec!(1.60);
pub const GMD_PADRAO: Decimal = dec!(55);
pub const GMD_EXCELENCIA: Decimal = dec!(60);
pub const VIABILIDADE_PADRAO: Decimal = dec!(95);
pub const VIABILIDADE_EXCELENCIA: Decimal = dec!(97);
pub const IEP_PADRAO: Decimal = dec!(350);
pub const IEP_EXCELENCIA: Decimal = dec!(400);
pub const CONSUMO_AGUA_POR_AVE: Decimal = dec!(200);
pub const CONSUMO_RACAO_POR_AVE: Decimal = dec!(100);
pub const RELACAO_AGUA_RACAO: Decimal = dec!(2.0);
pub const MORTALIDADE_MAXIMA: Decimal = dec!(5);
pub const DENSIDADE_MAXIMA: Decimal = dec!(18);
pub const PESO_ABATE_42_DIAS: Decimal = dec!(2400);
pub const TEMPERATURA_IDEAL: Decimal = dec!(24);
pub const UMIDADE_IDEAL: Decimal = dec!(60);
pub const NH3_MAXIMO: Decimal = dec!(25);
pub const CO2_MAXIMO: Decimal = dec!(3000);
```

### IEP Calculation (from .NET AviculturaService.cs lines 51-84)
```rust
// Source: AviculturaService.cs CalcularIEPAsync
pub async fn calcular_iep(pool: &PgPool, lote_id: i32) -> Result<Decimal, AppError> {
    let lote = sqlx::query_as::<_, Lote>(
        r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                  "DataAbatePrevista", "DataSaida",
                  "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                  "AreaGalpao", "Linhagem", "OrigemPintinhos",
                  "Status", "Observacoes", "GranjaId",
                  "DataCriacao", "DataAtualizacao"
           FROM "Lotes" WHERE "Id" = $1"#
    ).bind(lote_id).fetch_optional(pool).await?;

    let lote = match lote {
        Some(l) => l,
        None => return Ok(Decimal::ZERO),
    };

    let pesagem = sqlx::query_scalar::<_, Decimal>(
        r#"SELECT "PesoMedioGramas" FROM "PesagensSemanais"
           WHERE "LoteId" = $1 ORDER BY "DataPesagem" DESC LIMIT 1"#
    ).bind(lote_id).fetch_optional(pool).await?;

    let peso_medio = match pesagem {
        Some(p) => p,
        None => return Ok(Decimal::ZERO),
    };

    let idade = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days() as i32;
    if idade == 0 { return Ok(Decimal::ZERO); }

    let ganho_peso_kg = (peso_medio - dec!(45)) / dec!(1000); // 45g initial weight
    let viabilidade = /* compute from lote */ ;
    let ca = Self::calcular_conversao_alimentar(pool, lote_id).await?;

    if ca == Decimal::ZERO { return Ok(Decimal::ZERO); }

    let iep = (ganho_peso_kg * viabilidade * dec!(100)) / (ca * Decimal::from(idade));
    Ok(iep.round_dp(2))
}
```

### Health Check Handler
```rust
// Source: .NET RelatoriosController.cs Health() -- matches existing /health in main.rs
// D-08: only /health, skip debug endpoints
// Note: main.rs already has /health route. Phase 3 can enhance or keep as-is.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "relatorios",
        "timestamp": Utc::now()
    }))
}
```

### Moka Cache Integration in main.rs
```rust
// Source: moka docs + D-04/D-05
use crate::services::cache_service::CacheService;

// In main():
let cache_service = CacheService::new(10_000); // max 10k entries
let cache_data = web::Data::new(cache_service);

// In HttpServer::new closure:
.app_data(cache_data.clone())
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| moka 0.11 sync API | moka 0.12 unified async/sync API | 2024 | Use `future::Cache` for async Actix context [VERIFIED: cargo search] |
| rust_decimal_macros separate crate | Included in rust_decimal features | Recent | May need `features = ["macros"]` for `dec!()` macro [ASSUMED] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Separate moka Cache instances per TTL tier (5min vs 10min) is simpler than per-entry Expiry trait | Don't Hand-Roll | Minor: would need refactor to Expiry trait, both work |
| A2 | rust_decimal_macros dec!() macro available via feature flag | Code Examples | Low: can use Decimal::from_str or Decimal::new instead |
| A3 | moka 0.12 Cache::insert uses the instance's configured time_to_live | Architecture Patterns | Medium: if per-entry TTL needed, architecture changes slightly |

## Open Questions

1. **Missing DTOs for composite responses**
   - What we know: .NET `GetMetricasLote` and `GetDashboardCompleto` use anonymous objects. Existing `dto/avicultura.rs` has 14 structs but not these two composite types.
   - What's unclear: Whether these should be added to `dto/avicultura.rs` or a separate file
   - Recommendation: Add `MetricasLoteDto` and `DashboardAviculturaDto` to `dto/avicultura.rs` since they are avicultura-specific

2. **Report avicultura/desempenho-lote DTOs**
   - What we know: .NET RelatoriosController returns large anonymous objects for avicultura and desempenho-lote reports
   - What's unclear: Whether to create dedicated report DTOs or nest existing avicultura DTOs
   - Recommendation: Create `RelatorioAviculturaDto` and `RelatorioDesempenhoLoteDto` with nested sub-structs, add to `dto/relatorios.rs`

3. **QualidadeAr.ParametrosAceitaveis**
   - What we know: .NET QualidadeAr has a computed property `ParametrosAceitaveis` used in desempenho-lote report. Rust model does not have this field.
   - What's unclear: Whether this is a DB column or computed property
   - Recommendation: It's a computed property in .NET (line 80 of QualidadeAr.cs). Compute in Rust service layer.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | Cargo.toml (workspace test settings) |
| Quick run command | `cargo check` (type-check without full build) |
| Full suite command | `cargo build` (compilation is the primary gate for Rust) |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| AVIC-01 | Metricas endpoint returns IEP/CA/GMD/etc | smoke | `curl localhost:8080/api/avicultura/1/metricas` | N/A - manual |
| AVIC-05 | Alertas checks mortality/density/NH3/temp | unit | `cargo test avicultura` | Wave 0 |
| AVIC-06 | Industry comparison scoring | unit | `cargo test avicultura` | Wave 0 |
| RELA-01 | Health check responds | smoke | `curl localhost:8080/health` | N/A - manual |
| RELA-02-07 | Report endpoints return data | smoke | manual curl | N/A - manual |
| CACH-01 | Cache reduces response time | manual | manual timing comparison | N/A - manual |

### Sampling Rate
- **Per task commit:** `cargo check` (fast type-checking)
- **Per wave merge:** `cargo build` (full compilation)
- **Phase gate:** `cargo build` + manual endpoint smoke tests

### Wave 0 Gaps
- None critical -- Rust's type system is the primary validation. Compilation success verifies struct correctness, query type alignment, and handler signatures. Manual smoke testing against running server validates runtime behavior.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | JWT middleware already in place (Phase 1) |
| V3 Session Management | no | Stateless JWT (Phase 1) |
| V4 Access Control | yes | Role guards in handlers (D-09): Avicultura = Admin+Produtor, Reports = all roles |
| V5 Input Validation | yes | Date range validation (max 365 days, start < end), loteId path param validation |
| V6 Cryptography | no | No crypto in this phase |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| SQL injection via query params | Tampering | SQLx parameterized queries ($1, $2) -- already established pattern |
| Unauthorized data access | Elevation of privilege | Role guard in every handler + role-filtered queries in reports |
| Cache poisoning | Tampering | Cache is server-side only, not user-controlled keys. Include user_id in role-filtered cache keys |
| DoS via heavy report queries | Denial of service | LIMIT 1000 on queries (matching .NET .Take(1000)), moka cache reduces repeated load |

## Sources

### Primary (HIGH confidence)
- `.NET source code` - AviculturaService.cs, RelatorioService.cs, RelatorioAvancadoService.cs, AviculturaController.cs, RelatoriosController.cs (source of truth for behavior)
- `Existing Rust codebase` - services/, handlers/, dto/, models/, errors.rs, main.rs (patterns to follow)
- `cargo search moka` - Version 0.12.15 confirmed [VERIFIED: cargo search]

### Secondary (MEDIUM confidence)
- `ICacheService.cs + MemoryCacheService.cs` - Cache API design reference

### Tertiary (LOW confidence)
- moka per-entry TTL behavior (A1, A3) -- needs verification against moka 0.12 docs

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - moka version verified, all other deps already in Cargo.toml
- Architecture: HIGH - patterns directly derived from existing Rust code + .NET reference
- Pitfalls: HIGH - identified from direct code analysis of both codebases
- Business logic: HIGH - .NET source code is the authoritative reference, fully read

**Research date:** 2026-04-07
**Valid until:** 2026-05-07 (stable stack, no fast-moving dependencies)
