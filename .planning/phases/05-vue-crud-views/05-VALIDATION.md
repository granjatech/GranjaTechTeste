---
phase: 05
slug: vue-crud-views
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-08
---

# Phase 05 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Vitest (via Vite) |
| **Config file** | `granjatech-frontend/vite.config.ts` |
| **Quick run command** | `cd granjatech-frontend && npx vitest run --reporter=verbose` |
| **Full suite command** | `cd granjatech-frontend && npx vitest run` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cd granjatech-frontend && npx vitest run --reporter=verbose`
- **After every plan wave:** Run `cd granjatech-frontend && npx vitest run`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 05-01-01 | 01 | 1 | VIEW-02 | — | N/A | manual | Visual check: Dashboard KPIs render | ❌ W0 | ⬜ pending |
| 05-01-02 | 01 | 1 | VIEW-02 | — | N/A | manual | Visual check: Charts render with data | ❌ W0 | ⬜ pending |
| 05-01-03 | 01 | 1 | VIEW-15 | — | N/A | manual | Visual check: Profile page loads | ❌ W0 | ⬜ pending |
| 05-01-04 | 01 | 1 | VIEW-14 | — | N/A | manual | Visual check: Auditoria table loads | ❌ W0 | ⬜ pending |
| 05-02-01 | 02 | 1 | VIEW-03 | — | N/A | manual | CRUD ops: Granjas create/edit/delete | ❌ W0 | ⬜ pending |
| 05-02-02 | 02 | 1 | VIEW-04 | — | N/A | manual | CRUD ops: Lotes create/edit/delete | ❌ W0 | ⬜ pending |
| 05-02-03 | 02 | 1 | VIEW-06 | — | N/A | manual | CRUD ops: Estoque create/edit/delete | ❌ W0 | ⬜ pending |
| 05-02-04 | 02 | 1 | VIEW-05 | — | N/A | manual | CRUD ops: Financeiro create/edit/delete | ❌ W0 | ⬜ pending |
| 05-03-01 | 03 | 2 | VIEW-07 | — | N/A | manual | Data + Charts: Consumo renders | ❌ W0 | ⬜ pending |
| 05-03-02 | 03 | 2 | VIEW-08 | — | N/A | manual | Data + Charts: Pesagem renders | ❌ W0 | ⬜ pending |
| 05-03-03 | 03 | 2 | VIEW-09 | — | N/A | manual | Data + Charts: Sensores renders | ❌ W0 | ⬜ pending |
| 05-03-04 | 03 | 2 | VIEW-10 | — | N/A | manual | Data + Charts: Sanitario renders | ❌ W0 | ⬜ pending |
| 05-04-01 | 04 | 2 | VIEW-11 | — | N/A | manual | Avicultura dashboard renders | ❌ W0 | ⬜ pending |
| 05-04-02 | 04 | 2 | VIEW-12,13 | — | N/A | manual | Relatorios + PDF/Excel export | ❌ W0 | ⬜ pending |
| 05-04-03 | 04 | 2 | VIEW-03-15 | — | N/A | manual | Responsive + theme/font scale | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Install `vue-chartjs` + `chart.js` — chart rendering dependency
- [ ] Install `jspdf` + `jspdf-autotable` — PDF export
- [ ] Install `xlsx` — Excel export
- [ ] Verify Vitest is configured for Vue SFC testing

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Dashboard KPIs display correct values | VIEW-02 | Requires running backend with seeded data | Start backend + frontend, navigate to /dashboard |
| CRUD dialogs open/close/submit correctly | VIEW-03-06 | Interactive UI flows | Create, edit, delete records in each CRUD view |
| Charts render with correct data | VIEW-07-11 | Visual verification of chart rendering | Navigate to data views, verify chart axes/labels |
| PDF/Excel exports produce valid files | VIEW-12,13 | File download verification | Generate reports, open downloaded files |
| Responsive layout works on mobile | VIEW-03-15 | Viewport-dependent testing | Resize browser to mobile widths |
| Dark mode + font scale applied | VIEW-03-15 | Visual theme verification | Toggle dark mode, change font scale |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
