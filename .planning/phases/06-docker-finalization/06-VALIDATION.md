---
phase: 6
slug: docker-finalization
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-08
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Docker Compose + curl/shell scripts |
| **Config file** | `granjatech-backend/docker-compose.yml` (to be created) |
| **Quick run command** | `docker-compose -f granjatech-backend/docker-compose.yml ps` |
| **Full suite command** | `docker-compose -f granjatech-backend/docker-compose.yml up -d && curl -f http://localhost:5099/health` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `docker-compose config --quiet` (validate compose syntax)
- **After every plan wave:** Run full suite command (build + health check)
- **Before `/gsd-verify-work`:** Full suite must be green — all containers healthy
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 06-01-01 | 01 | 1 | DOCK-01 | — | N/A | integration | `docker build -f granjatech-backend/Dockerfile granjatech-backend/` | ❌ W0 | ⬜ pending |
| 06-01-02 | 01 | 1 | DOCK-02 | — | N/A | integration | `docker build -f granjatech-frontend/Dockerfile granjatech-frontend/` | ❌ W0 | ⬜ pending |
| 06-02-01 | 02 | 2 | DOCK-03 | — | N/A | integration | `docker-compose up -d && docker-compose ps` | ❌ W0 | ⬜ pending |
| 06-02-02 | 02 | 2 | DOCK-04 | — | N/A | integration | `curl -f http://localhost:5099/health` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `granjatech-backend/Dockerfile` — Rust multi-stage build
- [ ] `granjatech-frontend/Dockerfile` — Vue multi-stage build with nginx
- [ ] `docker-compose.yml` — orchestration for all 3 services

*Infrastructure is created during plan execution — no pre-existing test framework needed.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| End-to-end user workflow | DOCK-04 | Requires browser interaction | Navigate to frontend, login, verify CRUD operations work through the containerized stack |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
