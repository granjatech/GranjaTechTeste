---
phase: 3
slug: reports-business-logic
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-07
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (built-in) |
| **Config file** | Cargo.toml (workspace test settings) |
| **Quick run command** | `cargo check` |
| **Full suite command** | `cargo build` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check`
- **After every plan wave:** Run `cargo build`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | AVIC-01 | T-3-01 / — | Role guard: Admin+Produtor only | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-02 | 01 | 1 | AVIC-02 | — | Stub returns empty | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-03 | 01 | 1 | AVIC-03 | — | Stub returns empty | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-04 | 01 | 1 | AVIC-04 | — | Stub returns empty | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-05 | 01 | 1 | AVIC-05 | T-3-02 | Parameterized queries, role guard | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-06 | 01 | 1 | AVIC-06 | — | Industry comparison uses hardcoded constants | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-07 | 01 | 1 | AVIC-07 | — | Stub returns defaults | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-08 | 01 | 1 | AVIC-08 | — | Stub returns defaults | compile | `cargo check` | ✅ | ⬜ pending |
| 03-01-09 | 01 | 1 | AVIC-09 | T-3-01 | Full dashboard aggregation, role guard | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-01 | 02 | 1 | RELA-01 | — | Health check at /health | smoke | `curl localhost:8080/health` | N/A | ⬜ pending |
| 03-02-02 | 02 | 1 | RELA-02 | T-3-03 | All 3 roles, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-03 | 02 | 1 | RELA-03 | T-3-03 | All 3 roles, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-04 | 02 | 1 | RELA-04 | T-3-03 | All 3 roles, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-05 | 02 | 1 | RELA-05 | T-3-03 | All 3 roles, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-06 | 02 | 1 | RELA-06 | T-3-03 | All 3 roles, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-02-07 | 02 | 1 | RELA-07 | T-3-03 | Date range validation, parameterized queries | compile | `cargo check` | ✅ | ⬜ pending |
| 03-03-01 | 03 | 2 | CACH-01 | T-3-04 | Server-side cache, user_id in keys | compile | `cargo check` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. Rust's type system is the primary validation — compilation success verifies struct correctness, query type alignment, and handler signatures.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Endpoint returns correct JSON | AVIC-01 through AVIC-09 | Requires running server + database | `curl -H "Authorization: Bearer $TOKEN" localhost:8080/api/avicultura/...` |
| Report data matches .NET | RELA-02 through RELA-07 | Business logic parity check | Compare JSON output with .NET backend for same DB state |
| Cache reduces response time | CACH-01 | Requires timing comparison | Time 2 consecutive requests to cached endpoint, second must be faster |
| Health check responds | RELA-01 | Requires running server | `curl localhost:8080/health` returns 200 |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
