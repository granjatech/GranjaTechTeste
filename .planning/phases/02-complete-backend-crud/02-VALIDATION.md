---
phase: 2
slug: complete-backend-crud
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-07
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual HTTP testing (curl/httpie) + cargo build/test |
| **Config file** | None — no automated test framework configured |
| **Quick run command** | `cargo build --manifest-path granjatech-api/Cargo.toml` |
| **Full suite command** | `cargo build --manifest-path granjatech-api/Cargo.toml && cargo test --manifest-path granjatech-api/Cargo.toml` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo build --manifest-path granjatech-api/Cargo.toml`
- **After every plan wave:** Run `cargo build --manifest-path granjatech-api/Cargo.toml && cargo test --manifest-path granjatech-api/Cargo.toml`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | LOTE-01..08 | T-02-01 | Role-based lote filtering, ownership check | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-01-02 | 01 | 1 | DASH-01..02 | T-02-02 | Role-filtered aggregation queries | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-02-01 | 02 | 2 | FINA-01..04 | T-02-03 | 5-min edit window, hierarchy check, admin-only delete | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-02-02 | 02 | 2 | CONS-01..05 | T-02-04 | Active lote validation, computed fields | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-02-03 | 02 | 2 | PESA-01..03 | T-02-05 | Active lote validation | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-02-04 | 02 | 2 | SANI-01..04 | T-02-06 | Active lote validation | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-03-01 | 03 | 2 | SENS-01..05 | T-02-07 | Granja ownership check, public leitura endpoint | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-03-02 | 03 | 2 | ESTO-01..04 | T-02-08 | Granja ownership check | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-03-03 | 03 | 2 | AUDI-01..02 | T-02-09 | Admin-only access | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |
| 02-03-04 | 03 | 2 | PERF-01..03 | T-02-10 | BCrypt password verify/hash | compilation | `cargo build --manifest-path granjatech-api/Cargo.toml` | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. No new test framework setup needed.

*Per REQUIREMENTS.md "Out of Scope": "Testes automatizados extensivos — Verificacao manual de paridade suficiente para migracao"*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Lote computed properties accuracy | LOTE-08 | Requires running server + seeded data | Start server, create lote, verify viabilidade/densidade values in GET response |
| Dashboard KPI aggregation | DASH-01 | Requires populated transactions/lotes | Start server, verify KPI values against manual DB query |
| Financas 5-minute edit window | FINA-03 | Time-dependent business rule | Create transaction, try edit within 5 min (success), wait 5 min, try again (fail) |
| Mortalidade quantity clamping | LOTE-06 | Requires live data validation | Register mortalidade with quantity > current aves, verify clamping |
| pt-BR month formatting | DASH-02 | Locale-specific output format | Check monthly summary returns "jan/26" format, not "Jan/26" |
| Public leitura endpoint | SENS-05 | Auth bypass verification | POST to /api/leituras without JWT token, verify 200 response |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
