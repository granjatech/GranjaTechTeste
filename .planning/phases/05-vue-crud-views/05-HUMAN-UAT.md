---
status: partial
phase: 05-vue-crud-views
source: [05-VERIFICATION.md]
started: 2026-04-08T12:00:00Z
updated: 2026-04-08T12:00:00Z
---

## Current Test

[awaiting human testing]

## Tests

### 1. Visual parity with React frontend
expected: Side-by-side browser comparison shows equivalent layout, components, and data display between React (port 3000) and Vue (port 5173) frontends
result: [pending]

### 2. Responsive layout
expected: All views render correctly at mobile (375px), tablet (768px), and desktop (1280px) viewport widths without overflow or broken layout
result: [pending]

### 3. Dark mode and font scale
expected: Toggle dark mode via accessibility settings — all views switch themes cleanly. Font scale changes apply consistently across all views
result: [pending]

### 4. End-to-end CRUD operations
expected: With backend running, create/read/update/delete records in Granjas, Lotes, Usuarios, Financeiro, Estoque, and Sensores views. Snackbar confirms each operation. Data persists on page refresh
result: [pending]

### 5. PDF/Excel export
expected: In Relatorios view, generate a report, export to PDF (downloads valid PDF with table data), export to Excel (downloads valid .xlsx with correct columns)
result: [pending]

## Summary

total: 5
passed: 0
issues: 0
pending: 5
skipped: 0
blocked: 0

## Gaps
