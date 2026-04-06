---
phase: 1
slug: rust-foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-06
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (built-in) |
| **Config file** | Cargo.toml `[dev-dependencies]` section |
| **Quick run command** | `cargo check && cargo test --lib` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~10 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check && cargo test --lib`
- **After every plan wave:** Run `cargo test`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | FOUND-01 | — | N/A | integration | `cargo test test_db_connection -- --ignored` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | FOUND-09 | — | N/A | unit | `cargo test test_app_error` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | AUTH-01 | T-1-01 | JWT with correct claims returned | integration | `cargo test test_login -- --ignored` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 1 | AUTH-07 | T-1-02 | BCrypt .NET hashes verified | unit | `cargo test test_bcrypt_compatibility` | ❌ W0 | ⬜ pending |
| 01-03-01 | 03 | 2 | GRAN-01 | T-1-03 | Role-based filtering enforced | integration | `cargo test test_granjas_role_filter -- --ignored` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/common/mod.rs` — shared test setup (PgPool creation with test DB)
- [ ] Unit test for BCrypt .NET hash compatibility
- [ ] Unit test for AppError response codes
- [ ] Integration test fixtures for auth and granjas

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Swagger UI accessible | FOUND-04 | Visual verification of rendered docs | Navigate to /swagger-ui/ and verify endpoints listed |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
