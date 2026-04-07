---
phase: 4
slug: vue-scaffold-auth
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-07
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | vitest (via Vite) |
| **Config file** | granjatech-frontend/vitest.config.ts (Wave 0 installs) |
| **Quick run command** | `cd granjatech-frontend && npx vitest run --reporter=verbose` |
| **Full suite command** | `cd granjatech-frontend && npx vitest run` |
| **Estimated runtime** | ~10 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cd granjatech-frontend && npx vitest run --reporter=verbose`
- **After every plan wave:** Run `cd granjatech-frontend && npx vitest run`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 01 | 1 | FRON-01 | — | N/A | build | `cd granjatech-frontend && npm run build` | ❌ W0 | ⬜ pending |
| 04-01-02 | 01 | 1 | FRON-02 | — | N/A | build | `cd granjatech-frontend && npm run build` | ❌ W0 | ⬜ pending |
| 04-02-01 | 02 | 1 | FRON-03 | T-04-01 | Token stored securely, JWT decoded | unit | `cd granjatech-frontend && npx vitest run` | ❌ W0 | ⬜ pending |
| 04-02-02 | 02 | 1 | FRON-04 | — | N/A | unit | `cd granjatech-frontend && npx vitest run` | ❌ W0 | ⬜ pending |
| 04-02-03 | 02 | 1 | FRON-05 | T-04-02 | 401 redirect, token injection | unit | `cd granjatech-frontend && npx vitest run` | ❌ W0 | ⬜ pending |
| 04-02-04 | 02 | 1 | FRON-06 | T-04-03 | Unauthenticated redirect to login | unit | `cd granjatech-frontend && npx vitest run` | ❌ W0 | ⬜ pending |
| 04-03-01 | 03 | 2 | FRON-07 | — | N/A | manual | Browser verification | — | ⬜ pending |
| 04-03-02 | 03 | 2 | FRON-08 | — | N/A | manual | Browser verification | — | ⬜ pending |
| 04-03-03 | 03 | 2 | VIEW-01 | T-04-04 | Login form validates, error shown | manual | Browser verification | — | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `granjatech-frontend/vitest.config.ts` — vitest configuration
- [ ] `granjatech-frontend/src/__tests__/` — test directory structure
- [ ] vitest + @vue/test-utils installed as dev dependencies

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Navigation drawer renders correctly on desktop/mobile | FRON-07 | Visual layout verification | Open app at desktop (>960px) and mobile (<960px) widths |
| PageContainer breadcrumbs display | FRON-08 | Visual verification | Navigate to a sub-page, verify breadcrumbs show |
| Login form submits and redirects | VIEW-01 | Requires running Rust backend | Open /login, enter valid credentials, verify redirect to / |
| Dark mode toggle works | FRON-04 | Visual theme change | Click dark mode icon, verify colors change |
| Font scale adjustment works | FRON-04 | Visual size change | Click font increase, verify text grows |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
