# Phase 5: Vue CRUD Views - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-07
**Phase:** 05-vue-crud-views
**Areas discussed:** Padrao CRUD e Dialogs, Graficos e Visualizacao, Export PDF/Excel, Agrupamento dos Plans

---

## Padrao CRUD e Dialogs

### Q1: Como devem funcionar os formularios de criar/editar nas views CRUD?

| Option | Description | Selected |
|--------|-------------|----------|
| v-dialog modal | Mesmo padrao do React — MUI Dialog vira Vuetify v-dialog. Formulario abre em modal sobre a tabela. | ✓ |
| Pagina separada por rota | Cada create/edit e uma rota propria (/granjas/novo, /granjas/:id/editar). | |
| Voce decide | Claude escolhe a melhor abordagem baseado na paridade com React. | |

**User's choice:** v-dialog modal (Recomendado)
**Notes:** Paridade direta com React MUI Dialog pattern.

### Q2: Como lidar com v-data-table para as tabelas de dados?

| Option | Description | Selected |
|--------|-------------|----------|
| Vuetify v-data-table | Componente nativo do Vuetify com sort, pagination, search embutidos. | ✓ |
| Tabela custom com v-table | Tabela simples do Vuetify, implementando sort/pagination manualmente. | |
| Voce decide | Claude escolhe baseado na paridade. | |

**User's choice:** Vuetify v-data-table (Recomendado)
**Notes:** Equivalente direto do MUI DataGrid.

### Q3: Sobre validacao de formularios nos dialogs CRUD — como validar?

| Option | Description | Selected |
|--------|-------------|----------|
| Vuetify rules nativas | Usar a prop :rules dos v-text-field/v-select do Vuetify. | ✓ |
| Vee-validate + zod | Lib popular no ecossistema Vue para validacao com schemas. | |
| Voce decide | Claude escolhe a abordagem mais compativel com paridade. | |

**User's choice:** Vuetify rules nativas (Recomendado)
**Notes:** Sem dependencia externa. Validacao inline.

---

## Graficos e Visualizacao

### Q1: Qual biblioteca de graficos usar para substituir Recharts?

| Option | Description | Selected |
|--------|-------------|----------|
| vue-chartjs | Wrapper Vue para Chart.js — ja mencionado no PROJECT.md. | ✓ |
| Apache ECharts (vue-echarts) | Mais poderoso para dashboards complexos, mas API diferente. | |
| Voce decide | Claude escolhe baseado nos tipos de grafico necessarios. | |

**User's choice:** vue-chartjs (Recomendado)
**Notes:** Suporta todos os tipos usados no React: Line, Bar, Pie, Doughnut.

---

## Export PDF/Excel

### Q1: Quais libs usar para export de PDF e Excel?

| Option | Description | Selected |
|--------|-------------|----------|
| jsPDF + SheetJS | Mesmas libs do React — jsPDF + jspdf-autotable para PDF, xlsx para Excel. | ✓ |
| Libs alternativas Vue | vue3-pdf-export + exceljs. Mais modernas mas mudam o comportamento. | |
| Voce decide | Claude escolhe a melhor abordagem de paridade. | |

**User's choice:** jsPDF + SheetJS (Recomendado)
**Notes:** Paridade total com React.

### Q2: Onde colocar a logica de export (PDF/Excel)?

| Option | Description | Selected |
|--------|-------------|----------|
| Composable useExport() | Composable Vue reutilizavel em src/composables/useExport.ts. | ✓ |
| Utility puro (sem composable) | Funcoes puras em src/utils/export.ts. | |
| Inline nas views | Logica de export dentro de cada view, como no React. | |

**User's choice:** Composable useExport() (Recomendado)
**Notes:** Centraliza logica de export, views importam e chamam.

---

## Agrupamento dos Plans

### Q1: Como dividir as 14 views em 4 plans?

| Option | Description | Selected |
|--------|-------------|----------|
| Por complexidade crescente | Plan 1: simples, Plan 2: CRUD medio, Plan 3: dados+charts, Plan 4: complexos. | ✓ |
| Por dominio funcional | Agrupar por area de negocio: gestao, operacao, financas, analytics. | |
| Voce decide | Claude organiza da forma mais eficiente para execucao. | |

**User's choice:** Por complexidade crescente (Recomendado)
**Notes:** 4 plans: simples (4 views) -> CRUD medio (4 views) -> dados+charts (4 views) -> complexos (2 views).

---

## Claude's Discretion

- Exact v-data-table column definitions and slot customizations
- Chart configuration details (colors, tooltips, responsive options)
- v-dialog width per view
- Snackbar/notification pattern for CRUD feedback
- Composable structure for shared CRUD logic
- Role-based UI differences implementation

## Deferred Ideas

None — discussion stayed within phase scope.
