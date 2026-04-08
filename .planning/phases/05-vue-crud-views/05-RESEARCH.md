# Phase 5: Vue CRUD Views - Research

**Researched:** 2026-04-08
**Domain:** Vue 3 + Vuetify 3 CRUD views, vue-chartjs charting, jsPDF/SheetJS export
**Confidence:** HIGH

## Summary

This phase implements 14 Vue views to achieve full parity with the React frontend. The existing Vue scaffold (Phase 4) provides all infrastructure: Axios API service with token injection, Pinia auth/accessibility stores, Vue Router with 16 routes (all pointing to PlaceholderView), PageContainer and LoadingSpinner components, and a Vuetify theme matching the React MUI palette.

The work is predominantly UI translation: converting React hooks (`useState`, `useEffect`, `useCallback`) to Vue Composition API (`ref`, `onMounted`, `computed`), MUI components to Vuetify equivalents (`Table` to `v-data-table`, `Dialog` to `v-dialog`, `TextField` to `v-text-field`), and Recharts to vue-chartjs. The Rust backend API is already complete with all 60+ endpoints, and all DTOs use `camelCase` serialization matching frontend expectations.

**Primary recommendation:** Install vue-chartjs + chart.js, jspdf + jspdf-autotable, and xlsx. Implement views in 4 waves by complexity (simple, CRUD, charts, complex). Use a shared `useExport.ts` composable for PDF/Excel export and a `useSnackbar.ts` composable for CRUD feedback notifications. All views follow the same pattern: `<script setup lang="ts">` with `api.get/post/put/delete` calls, wrapped in `<PageContainer>`.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Create/Edit forms use Vuetify `v-dialog` modals over the data table -- same pattern as React MUI Dialog. Single dialog per view toggling between create/edit mode.
- **D-02:** Delete operations use a confirmation `v-dialog` before executing.
- **D-03:** All data tables use Vuetify `v-data-table` with built-in sort, pagination, search, and loading state.
- **D-04:** Form validation uses Vuetify native `:rules` prop on form fields. No external validation library.
- **D-05:** Chart library is `vue-chartjs` + `chart.js`.
- **D-06:** Chart types by view: Dashboard (Bar receita mensal), Consumo (Line racao/agua), Pesagem (Line peso medio), Avicultura (Line crescimento, Bar comparacao industria), Sensores (Line leituras).
- **D-07:** PDF export uses `jsPDF` + `jspdf-autotable`. Excel export uses `xlsx` (SheetJS).
- **D-08:** Export logic centralized in composable `src/composables/useExport.ts`.
- **D-09:** Plan 1 -- DashboardView, ProfileView, AuditoriaView, UsuariosView (+ install vue-chartjs + chart.js).
- **D-10:** Plan 2 -- GranjasView, LotesView, EstoqueView, FinanceiroView (standard CRUD).
- **D-11:** Plan 3 -- ConsumoView, PesagemView, SensoresView, SanitarioView (data + charts).
- **D-12:** Plan 4 -- AviculturaView, RelatoriosView (complex + useExport composable).
- **D-13:** All Phase 4 decisions carry forward (Composition API, PascalCase files, folder layout, theme, API service, auth store, accessibility store, PageContainer, LoadingSpinner).

### Claude's Discretion
- Exact v-data-table column definitions and slot customizations per view
- Chart configuration details (colors, tooltips, responsive options)
- v-dialog width/max-width per view based on form complexity
- Snackbar/notification pattern for CRUD success/error feedback
- Composable structure for shared CRUD logic (if beneficial)
- How to handle role-based UI differences within views (show/hide buttons, filter data)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| VIEW-02 | DashboardView.vue com KPIs e graficos (vue-chartjs) | vue-chartjs Bar component + api.get('/dashboard/kpis') and api.get('/dashboard/resumo-mensal') |
| VIEW-03 | GranjasView.vue com CRUD e dialogs | v-data-table + v-dialog + api CRUD on /granjas |
| VIEW-04 | LotesView.vue com CRUD e campos extras | v-data-table + v-dialog + /lotes + /lotes/{id}/mortalidades |
| VIEW-05 | UsuariosView.vue com admin CRUD | v-data-table + v-dialog + /auth/usuarios, admin-only |
| VIEW-06 | FinanceiroView.vue com transacoes e resumo | v-data-table + v-dialog + /financas CRUD |
| VIEW-07 | EstoqueView.vue com produtos CRUD | v-data-table + v-dialog + /estoque CRUD |
| VIEW-08 | ProfileView.vue com perfil e troca de senha | v-card forms + /profile GET/PUT + /profile/change-password POST |
| VIEW-09 | AuditoriaView.vue com tabela read-only | v-data-table read-only + /auditoria GET |
| VIEW-10 | SensoresView.vue com sensores, leituras e graficos | v-data-table + v-dialog + /sensores + /leituras + vue-chartjs Line |
| VIEW-11 | ConsumoView.vue com racao/agua e graficos | Lote selector + /consumo/racao/{id} + /consumo/agua/{id} + vue-chartjs Line |
| VIEW-12 | PesagemView.vue com pesagens e graficos | Lote selector + /pesagem/{loteId} + vue-chartjs Line |
| VIEW-13 | SanitarioView.vue com eventos e cronograma | Lote selector + /sanitario/{loteId} + /sanitario/cronograma-vacinacao + /sanitario/resumo/{loteId} |
| VIEW-14 | AviculturaView.vue com dashboard de lote | Lote selector + 4 parallel API calls to /avicultura/{loteId}/* + multiple vue-chartjs charts |
| VIEW-15 | RelatoriosView.vue com relatorios, export PDF e Excel | Tab-based UI + 6 report endpoints + useExport composable (jsPDF + xlsx) |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| vue-chartjs | 5.3.3 | Vue 3 wrapper for Chart.js | [VERIFIED: npm registry] Only maintained Vue 3 chart wrapper with Chart.js 4 support |
| chart.js | 4.5.1 | Canvas-based charting engine | [VERIFIED: npm registry] Peer dependency of vue-chartjs, required |
| jspdf | 4.2.1 | PDF generation in browser | [VERIFIED: npm registry] Same library used in React version, direct parity |
| jspdf-autotable | 5.0.7 | Table generation plugin for jsPDF | [VERIFIED: npm registry] Same library used in React version |
| xlsx | 0.18.5 | Excel spreadsheet generation | [VERIFIED: npm registry] Same library used in React version (SheetJS) |

### Already Installed (from Phase 4)
| Library | Version | Purpose |
|---------|---------|---------|
| vue | ^3.5.0 | Framework |
| vuetify | ^3.7.0 | Component library (v-data-table, v-dialog, v-form, etc.) |
| vue-router | ^4.4.0 | Routing (16 routes already defined) |
| pinia | ^2.2.0 | State management (auth + accessibility stores) |
| axios | ^1.7.0 | HTTP client (api.ts service ready) |
| jwt-decode | ^4.0.0 | JWT token decoding |
| @mdi/font | ^7.4.0 | Material Design Icons |

**Installation:**
```bash
cd granjatech-frontend && npm install vue-chartjs chart.js jspdf jspdf-autotable xlsx
```

## Architecture Patterns

### Recommended Project Structure
```
granjatech-frontend/src/
  views/
    DashboardView.vue         # KPIs + bar chart
    GranjasView.vue           # CRUD table + dialog
    LotesView.vue             # CRUD table + dialog + mortalidade
    UsuariosView.vue          # Admin CRUD table + dialog
    FinanceiroView.vue        # CRUD table + dialog + summary cards
    EstoqueView.vue           # CRUD table + dialog
    ProfileView.vue           # Profile edit + password change forms
    AuditoriaView.vue         # Read-only data table
    SensoresView.vue          # Sensors CRUD + readings table + line chart
    ConsumoView.vue           # Lote selector + consumption data + line charts
    PesagemView.vue           # Lote selector + weighings + line chart
    SanitarioView.vue         # Lote selector + events + vaccination schedule
    AviculturaView.vue        # Lote selector + full analytics dashboard
    RelatoriosView.vue        # Tab-based reports + PDF/Excel export
    LoginView.vue             # (existing from Phase 4)
  composables/
    useExport.ts              # PDF/Excel export functions
    useSnackbar.ts            # Notification feedback (optional)
  components/
    PageContainer.vue         # (existing from Phase 4)
    LoadingSpinner.vue        # (existing from Phase 4)
    ResponsiveNavigation.vue  # (existing from Phase 4)
  stores/
    auth.ts                   # (existing)
    accessibility.ts          # (existing)
  services/
    api.ts                    # (existing)
  router/
    index.ts                  # (existing -- update imports from PlaceholderView to real views)
```

### Pattern 1: Standard CRUD View
**What:** The repeatable pattern for all CRUD views (Granjas, Lotes, Usuarios, Financeiro, Estoque)
**When to use:** Any view with create/read/update/delete operations on a data table
**Example:**
```typescript
// Source: Derived from existing LoginView.vue pattern + CONTEXT.md decisions D-01..D-04
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

// Types
interface Granja {
  id: number
  codigo: string
  nome: string
  localizacao: string | null
  usuarioId: number
}

interface GranjaForm {
  nome: string
  localizacao: string
  usuarioId: number | null
}

const auth = useAuthStore()
const items = ref<Granja[]>([])
const loading = ref(true)
const dialogOpen = ref(false)
const deleteDialogOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const deletingId = ref<number | null>(null)
const snackbar = ref({ show: false, text: '', color: 'success' })

const initialForm: GranjaForm = { nome: '', localizacao: '', usuarioId: null }
const form = ref<GranjaForm>({ ...initialForm })

const headers = [
  { title: 'Codigo', key: 'codigo', sortable: true },
  { title: 'Nome', key: 'nome', sortable: true },
  { title: 'Localizacao', key: 'localizacao', sortable: true },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

async function fetchData() {
  loading.value = true
  try {
    const { data } = await api.get('/granjas')
    items.value = data
  } catch {
    snackbar.value = { show: true, text: 'Erro ao carregar dados', color: 'error' }
  } finally {
    loading.value = false
  }
}

function openCreate() {
  isEditMode.value = false
  form.value = { ...initialForm }
  dialogOpen.value = true
}

function openEdit(item: Granja) {
  isEditMode.value = true
  editingId.value = item.id
  form.value = { nome: item.nome, localizacao: item.localizacao || '', usuarioId: item.usuarioId }
  dialogOpen.value = true
}

async function handleSubmit() {
  try {
    if (isEditMode.value && editingId.value) {
      await api.put(`/granjas/${editingId.value}`, form.value)
      snackbar.value = { show: true, text: 'Atualizado com sucesso', color: 'success' }
    } else {
      await api.post('/granjas', form.value)
      snackbar.value = { show: true, text: 'Criado com sucesso', color: 'success' }
    }
    dialogOpen.value = false
    await fetchData()
  } catch {
    snackbar.value = { show: true, text: 'Erro ao salvar', color: 'error' }
  }
}

function confirmDelete(id: number) {
  deletingId.value = id
  deleteDialogOpen.value = true
}

async function executeDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/granjas/${deletingId.value}`)
    snackbar.value = { show: true, text: 'Excluido com sucesso', color: 'success' }
    deleteDialogOpen.value = false
    await fetchData()
  } catch {
    snackbar.value = { show: true, text: 'Erro ao excluir', color: 'error' }
  }
}

onMounted(fetchData)
</script>
```

### Pattern 2: Chart View with Lote Selector
**What:** Views that display data/charts for a selected lote (Consumo, Pesagem, Sensores, Avicultura)
**When to use:** Data visualization views requiring a lote selection dropdown
**Example:**
```typescript
// Source: Derived from AviculturaPage.js pattern
<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import api from '@/services/api'
import { Bar, Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale, LinearScale, BarElement, PointElement, LineElement,
  Title, Tooltip, Legend, Filler
} from 'chart.js'

// Register Chart.js components once
ChartJS.register(
  CategoryScale, LinearScale, BarElement, PointElement, LineElement,
  Title, Tooltip, Legend, Filler
)

const lotes = ref([])
const selectedLoteId = ref<number | null>(null)
const chartData = ref(null)
const loading = ref(true)

async function fetchLotes() {
  const { data } = await api.get('/lotes')
  lotes.value = data.filter((l: any) => l.status === 'Ativo')
  if (lotes.value.length > 0) {
    selectedLoteId.value = lotes.value[0].id
  }
}

async function fetchChartData(loteId: number) {
  // fetch endpoint specific data
}

watch(selectedLoteId, (id) => { if (id) fetchChartData(id) })
onMounted(fetchLotes)
</script>
```

### Pattern 3: useExport Composable
**What:** Centralized PDF/Excel export logic per D-08
**When to use:** RelatoriosView (primary), but available to any view needing export
**Example:**
```typescript
// Source: Derived from RelatoriosPage.js export logic + D-07/D-08
// src/composables/useExport.ts
import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import * as XLSX from 'xlsx'

interface ExportColumn {
  header: string
  key: string
  width?: number
}

export function useExport() {
  function exportToPdf(title: string, columns: ExportColumn[], data: Record<string, any>[]) {
    const doc = new jsPDF()
    doc.setFontSize(16)
    doc.text(title, 14, 20)
    doc.setFontSize(10)
    doc.text(`Gerado em: ${new Date().toLocaleDateString('pt-BR')}`, 14, 28)

    autoTable(doc, {
      startY: 35,
      head: [columns.map(c => c.header)],
      body: data.map(row => columns.map(c => String(row[c.key] ?? ''))),
    })

    doc.save(`${title.toLowerCase().replace(/\s+/g, '-')}.pdf`)
  }

  function exportToExcel(filename: string, columns: ExportColumn[], data: Record<string, any>[]) {
    const wsData = [
      columns.map(c => c.header),
      ...data.map(row => columns.map(c => row[c.key] ?? '')),
    ]
    const ws = XLSX.utils.aoa_to_sheet(wsData)
    const wb = XLSX.utils.book_new()
    XLSX.utils.book_append_sheet(wb, ws, 'Dados')
    XLSX.writeFile(wb, `${filename}.xlsx`)
  }

  return { exportToPdf, exportToExcel }
}
```

### Pattern 4: Snackbar Notification
**What:** Consistent CRUD feedback using Vuetify v-snackbar
**When to use:** After every create/update/delete operation
**Example:**
```vue
<!-- Template snippet for all CRUD views -->
<v-snackbar v-model="snackbar.show" :color="snackbar.color" :timeout="3000" location="bottom end">
  {{ snackbar.text }}
  <template #actions>
    <v-btn variant="text" @click="snackbar.show = false">Fechar</v-btn>
  </template>
</v-snackbar>
```

### Pattern 5: Role-Based UI Filtering
**What:** Show/hide CRUD buttons and columns based on user role
**When to use:** GranjasView, FinanceiroView, UsuariosView, LotesView
**Example:**
```typescript
// Source: Derived from GranjasPage.js role checks
const auth = useAuthStore()
const canCreate = computed(() => auth.user?.role !== 'Financeiro')
const isAdmin = computed(() => auth.user?.role === 'Administrador')

// In template:
// <v-btn v-if="canCreate" ... />
// <template v-if="isAdmin" #item.owner="{ item }">...</template>
```

### Anti-Patterns to Avoid
- **Importing all Chart.js components globally:** Register only needed chart types (Bar, Line) per view to keep bundle size down. Use tree-shaking-friendly individual imports from `chart.js`.
- **Duplicating export logic across views:** All export logic must live in `useExport.ts` composable (D-08).
- **Using window.confirm for delete:** React version uses `window.confirm()` -- Vue version must use `v-dialog` confirmation (D-02).
- **Forgetting to update router/index.ts:** Each new view file must replace its PlaceholderView import in the router.
- **Creating separate API service methods:** The Vue `api.ts` is a raw Axios instance, not a method-per-endpoint wrapper like React's `apiService.js`. Call `api.get('/granjas')` directly -- do not create a facade.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Data tables with sort/pagination/search | Custom table component | `v-data-table` (Vuetify) | Handles loading, empty states, slot customization out of the box |
| Form validation | Custom validation logic | Vuetify `:rules` prop | Built into every Vuetify input component, handles display and blocking submit |
| Modal dialogs | Custom overlay/portal | `v-dialog` (Vuetify) | Handles focus trap, scroll lock, overlay, responsive sizing |
| Chart rendering | Canvas/SVG manual drawing | vue-chartjs + chart.js | Responsive, themed, tooltip, legend all built-in |
| PDF table generation | Manual PDF coordinate math | jspdf-autotable | Handles page breaks, column widths, cell formatting |
| Excel generation | CSV string building | xlsx (SheetJS) | Proper .xlsx format with formatting support |

**Key insight:** This phase is a 1:1 translation -- every React component has a direct Vuetify equivalent. No novel UI patterns needed.

## Common Pitfalls

### Pitfall 1: Chart.js Component Registration
**What goes wrong:** vue-chartjs components throw "X is not a registered element" errors
**Why it happens:** Chart.js 4.x uses tree-shakeable architecture; components must be explicitly registered
**How to avoid:** Register all used Chart.js components before using vue-chartjs wrapper components:
```typescript
import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, PointElement, LineElement, Title, Tooltip, Legend } from 'chart.js'
ChartJS.register(CategoryScale, LinearScale, BarElement, PointElement, LineElement, Title, Tooltip, Legend)
```
**Warning signs:** Runtime error "X is not a registered scale/element"

### Pitfall 2: v-data-table Headers Format
**What goes wrong:** Table columns don't render or sort incorrectly
**Why it happens:** Vuetify 3 v-data-table uses `{ title, key, sortable }` format, not `{ text, value }` from Vuetify 2
**How to avoid:** Use the Vuetify 3 headers format:
```typescript
const headers = [
  { title: 'Nome', key: 'nome', sortable: true },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' },
]
```
**Warning signs:** Empty columns, console warnings about unknown keys [VERIFIED: Vuetify 3 docs]

### Pitfall 3: v-data-table Item Slots
**What goes wrong:** Custom cell rendering doesn't work
**Why it happens:** Vuetify 3 uses `#item.columnKey` slot naming, different from Vuetify 2
**How to avoid:** Use dynamic slot names matching the `key` in headers:
```vue
<v-data-table :headers="headers" :items="items">
  <template #item.actions="{ item }">
    <v-btn icon size="small" @click="openEdit(item)">
      <v-icon>mdi-pencil</v-icon>
    </v-btn>
  </template>
</v-data-table>
```
**Warning signs:** Slot content not rendering, default cell rendering instead [VERIFIED: Vuetify 3 docs]

### Pitfall 4: Reactive Chart Data
**What goes wrong:** Charts don't update when data changes
**Why it happens:** vue-chartjs components are reactive by default in v5 but the data object reference must change
**How to avoid:** Always assign a new object to chartData rather than mutating properties:
```typescript
// Good: new reference triggers re-render
chartData.value = { labels: [...], datasets: [...] }

// Bad: mutation does NOT trigger re-render
chartData.value.labels.push('new')
```
**Warning signs:** Chart shows stale data after API refetch [ASSUMED]

### Pitfall 5: jsPDF + autotable Import
**What goes wrong:** `autoTable is not a function` or missing method on doc
**Why it happens:** jspdf-autotable v5 changed its import pattern
**How to avoid:** Import as a function, not a plugin:
```typescript
import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
// Usage: autoTable(doc, { ... })  -- NOT doc.autoTable(...)
```
**Warning signs:** TypeError at runtime when generating PDF [VERIFIED: npm jspdf-autotable 5.x changelog]

### Pitfall 6: Vuetify Form Validation Rules
**What goes wrong:** Form submits despite validation errors
**Why it happens:** `:rules` only shows validation UI -- it doesn't block form submission automatically
**How to avoid:** Use `v-form` with a template ref and call `validate()` before submit:
```typescript
const formRef = ref()
async function handleSubmit() {
  const { valid } = await formRef.value.validate()
  if (!valid) return
  // proceed with API call
}
```
```vue
<v-form ref="formRef" @submit.prevent="handleSubmit">
```
**Warning signs:** Invalid data reaches the API [VERIFIED: Vuetify 3 docs]

## Code Examples

### vue-chartjs Bar Chart (Dashboard)
```typescript
// Source: vue-chartjs docs + DashboardPage.js reference
<script setup lang="ts">
import { Bar } from 'vue-chartjs'
import {
  Chart as ChartJS, CategoryScale, LinearScale, BarElement,
  Title, Tooltip, Legend
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

// Props or computed from API data
const chartData = computed(() => ({
  labels: monthlyData.value.map((d: any) => d.mes),
  datasets: [
    {
      label: 'Entradas',
      data: monthlyData.value.map((d: any) => Number(d.entradas)),
      backgroundColor: '#4caf50',
      borderRadius: 4,
    },
    {
      label: 'Saidas',
      data: monthlyData.value.map((d: any) => Number(d.saidas)),
      backgroundColor: '#f44336',
      borderRadius: 4,
    },
  ],
}))

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: 'top' as const },
    tooltip: {
      callbacks: {
        label: (ctx: any) => `R$ ${Number(ctx.raw).toLocaleString('pt-BR', { minimumFractionDigits: 2 })}`,
      },
    },
  },
  scales: {
    y: {
      ticks: {
        callback: (value: any) => `R$${(Number(value) / 1000).toFixed(0)}k`,
      },
    },
  },
}
</script>

<template>
  <v-card>
    <v-card-text>
      <h2 class="text-h5 font-weight-bold mb-4">Resumo Mensal</h2>
      <div style="height: 400px">
        <Bar :data="chartData" :options="chartOptions" />
      </div>
    </v-card-text>
  </v-card>
</template>
```

### vue-chartjs Line Chart (Consumo/Pesagem/Sensores)
```typescript
// Source: vue-chartjs docs + ConsumoPage.js reference
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement,
  Title, Tooltip, Legend, Filler
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler)

const lineChartData = computed(() => ({
  labels: consumoData.value.map((d: any) => new Date(d.data).toLocaleDateString('pt-BR')),
  datasets: [
    {
      label: 'Racao (kg)',
      data: consumoData.value.map((d: any) => Number(d.racaoKg)),
      borderColor: '#FF6F00',
      fill: false,
      tension: 0.3,
    },
  ],
}))
```

### v-data-table with CRUD Actions
```vue
<!-- Source: Vuetify 3 v-data-table docs + GranjasPage.js reference -->
<v-data-table
  :headers="headers"
  :items="items"
  :loading="loading"
  :search="search"
  items-per-page="10"
  class="elevation-1"
>
  <template #top>
    <v-toolbar flat>
      <v-text-field
        v-model="search"
        prepend-inner-icon="mdi-magnify"
        label="Buscar"
        variant="outlined"
        density="compact"
        hide-details
        class="mx-4"
        style="max-width: 300px"
      />
      <v-spacer />
      <v-btn v-if="canCreate" color="primary" prepend-icon="mdi-plus" @click="openCreate">
        Novo
      </v-btn>
    </v-toolbar>
  </template>

  <template #item.actions="{ item }">
    <v-btn icon="mdi-pencil" size="small" variant="text" @click="openEdit(item)" />
    <v-btn icon="mdi-delete" size="small" variant="text" color="error" @click="confirmDelete(item.id)" />
  </template>

  <template #no-data>
    <div class="d-flex flex-column align-center pa-8">
      <v-icon size="48" color="grey">mdi-database-off</v-icon>
      <p class="text-grey mt-2">Nenhum registro encontrado</p>
    </div>
  </template>
</v-data-table>
```

### v-dialog for Create/Edit Form
```vue
<!-- Source: Vuetify 3 v-dialog docs + D-01 decision -->
<v-dialog v-model="dialogOpen" max-width="600" persistent>
  <v-card>
    <v-card-title class="d-flex align-center ga-2">
      <v-icon color="primary">mdi-pencil</v-icon>
      {{ isEditMode ? 'Editar' : 'Cadastrar' }}
    </v-card-title>
    <v-card-text>
      <v-form ref="formRef" @submit.prevent="handleSubmit">
        <v-text-field
          v-model="form.nome"
          label="Nome"
          :rules="[v => !!v || 'Nome obrigatorio']"
          variant="outlined"
          class="mb-2"
        />
        <!-- more fields -->
      </v-form>
    </v-card-text>
    <v-card-actions class="pa-4">
      <v-spacer />
      <v-btn variant="outlined" @click="dialogOpen = false">Cancelar</v-btn>
      <v-btn color="primary" @click="handleSubmit">{{ isEditMode ? 'Atualizar' : 'Criar' }}</v-btn>
    </v-card-actions>
  </v-card>
</v-dialog>
```

### Delete Confirmation Dialog
```vue
<!-- Source: D-02 decision -->
<v-dialog v-model="deleteDialogOpen" max-width="400">
  <v-card>
    <v-card-title>Confirmar Exclusao</v-card-title>
    <v-card-text>Tem certeza que deseja excluir este registro?</v-card-text>
    <v-card-actions>
      <v-spacer />
      <v-btn variant="outlined" @click="deleteDialogOpen = false">Cancelar</v-btn>
      <v-btn color="error" @click="executeDelete">Excluir</v-btn>
    </v-card-actions>
  </v-card>
</v-dialog>
```

## API Endpoint Map (for View Implementation)

All endpoints are on the Rust backend at `http://localhost:5099/api`. All use camelCase JSON. [VERIFIED: Rust handler mod.rs]

| View | Endpoints | Method |
|------|-----------|--------|
| Dashboard | `/dashboard/kpis`, `/dashboard/resumo-mensal` | GET |
| Granjas | `/granjas` (CRUD), `/granjas/{id}` | GET/POST/PUT/DELETE |
| Lotes | `/lotes` (CRUD), `/lotes/{id}`, `/lotes/{id}/mortalidades` | GET/POST/PUT/DELETE |
| Usuarios | `/auth/usuarios`, `/auth/usuarios/{id}`, `/auth/registrar` | GET/POST/PUT/DELETE |
| Financeiro | `/financas` (CRUD), `/financas/{id}` | GET/POST/PUT/DELETE |
| Estoque | `/estoque` (CRUD), `/estoque/{id}` | GET/POST/PUT/DELETE |
| Profile | `/profile`, `/profile/change-password` | GET/PUT/POST |
| Auditoria | `/auditoria` | GET |
| Sensores | `/sensores` (CRUD), `/sensores/{id}/leituras`, `/leituras` | GET/POST/DELETE |
| Consumo | `/consumo/racao/{loteId}`, `/consumo/agua/{loteId}`, `/consumo/resumo/{loteId}`, `/consumo/racao`, `/consumo/agua` | GET/POST |
| Pesagem | `/pesagem/{loteId}`, `/pesagem/resumo/{loteId}`, `/pesagem` | GET/POST |
| Sanitario | `/sanitario/{loteId}`, `/sanitario/resumo/{loteId}`, `/sanitario/cronograma-vacinacao`, `/sanitario` | GET/POST |
| Avicultura | `/avicultura/{loteId}/metricas`, `/avicultura/{loteId}/alertas`, `/avicultura/{loteId}/comparacao-industria`, `/avicultura/{loteId}/curvas-crescimento`, `/avicultura/{loteId}/dashboard` | GET |
| Relatorios | `/relatorios/financeiro-simplificado`, `/relatorios/financeiro`, `/relatorios/producao`, `/relatorios/avicultura`, `/relatorios/desempenho-lote/{loteId}`, `/relatorios/avancado` | GET |

## React to Vue Component Mapping

| React (MUI) | Vue (Vuetify) | Notes |
|-------------|---------------|-------|
| `<Table>` + `<TableContainer>` | `<v-data-table>` | Vuetify includes pagination, sort, search built-in |
| `<Dialog>` | `<v-dialog>` | Use `persistent` prop to prevent backdrop close |
| `<TextField>` | `<v-text-field>` | Add `:rules` for validation (D-04) |
| `<Select>` + `<MenuItem>` | `<v-select>` | Use `:items` + `item-title`/`item-value` props |
| `<Button>` | `<v-btn>` | Use `prepend-icon` instead of `startIcon` |
| `<IconButton>` | `<v-btn icon variant="text" size="small">` | Different API |
| `<Alert>` | `<v-alert>` | Same concept, slightly different props |
| `<Grid container/item>` | `<v-row>` + `<v-col>` | Use `cols`, `sm`, `md`, `lg` props |
| `<Card>` + `<CardContent>` | `<v-card>` + `<v-card-text>` | Direct equivalent |
| `<Chip>` | `<v-chip>` | Direct equivalent |
| `<Tabs>` + `<Tab>` | `<v-tabs>` + `<v-tab>` | Use `v-model` for active tab |
| `<CircularProgress>` | `<v-progress-circular>` | Or use LoadingSpinner component |
| `<LinearProgress>` | `<v-progress-linear>` | Direct equivalent |
| `<Tooltip>` | `<v-tooltip>` | Use with activator slot |
| `<Accordion>` | `<v-expansion-panels>` + `<v-expansion-panel>` | Different structure |
| Recharts `<BarChart>` | vue-chartjs `<Bar>` | Different API, same visual result |
| Recharts `<LineChart>` | vue-chartjs `<Line>` | Different API, same visual result |
| Recharts `<ResponsiveContainer>` | Chart.js `responsive: true` option | Built into chart options |
| `useContext(AuthContext)` | `useAuthStore()` | Pinia store already set up |
| `useState` + `useEffect` | `ref` + `onMounted`/`watch` | Vue Composition API equivalents |
| `useCallback` | Not needed | Vue's reactivity system handles this |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| vue-chartjs v3 (Options API) | vue-chartjs v5 (Composition API) | 2023 | Use `<Bar>`, `<Line>` as components with `:data` and `:options` props |
| Chart.js v3 global registration | Chart.js v4 tree-shakeable imports | 2023 | Must register each scale/element explicitly |
| jspdf-autotable as plugin (doc.autoTable) | jspdf-autotable v5 as function (autoTable(doc, ...)) | 2024 | Different import/call pattern |
| Vuetify 2 v-data-table `{ text, value }` | Vuetify 3 `{ title, key }` | 2023 | Header definition format changed |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | vue-chartjs v5 reactivity works by replacing the data object reference | Pitfall 4 | Charts may not update; would need explicit `ref` key change |
| A2 | xlsx 0.18.5 works with ES module imports in Vite | Standard Stack | May need `import * as XLSX` syntax or Vite config |

## Open Questions

1. **Exact column definitions per view**
   - What we know: React pages define column structure in JSX
   - What's unclear: Some columns may need date formatting or currency formatting helpers
   - Recommendation: Each plan task should read its corresponding React page and replicate columns exactly

2. **Error message display consistency**
   - What we know: React uses `window.alert()` for some errors, `Alert` component for others
   - What's unclear: Whether to standardize on snackbar for all feedback
   - Recommendation: Use `v-snackbar` for success/error feedback consistently (cleaner than alert)

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual visual UAT (no automated test framework in Vue project) |
| Config file | none |
| Quick run command | `cd granjatech-frontend && npm run build` (type-check + build) |
| Full suite command | `cd granjatech-frontend && npm run build` (type-check + build) |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| VIEW-02 | Dashboard KPIs + chart render | manual-only | `npm run build` (type-check) | N/A |
| VIEW-03 | Granjas CRUD operations | manual-only | `npm run build` | N/A |
| VIEW-04 | Lotes CRUD + mortalidade | manual-only | `npm run build` | N/A |
| VIEW-05 | Usuarios admin CRUD | manual-only | `npm run build` | N/A |
| VIEW-06 | Financeiro transactions CRUD | manual-only | `npm run build` | N/A |
| VIEW-07 | Estoque products CRUD | manual-only | `npm run build` | N/A |
| VIEW-08 | Profile edit + password change | manual-only | `npm run build` | N/A |
| VIEW-09 | Auditoria read-only table | manual-only | `npm run build` | N/A |
| VIEW-10 | Sensores CRUD + charts | manual-only | `npm run build` | N/A |
| VIEW-11 | Consumo data + charts | manual-only | `npm run build` | N/A |
| VIEW-12 | Pesagem data + charts | manual-only | `npm run build` | N/A |
| VIEW-13 | Sanitario events + schedule | manual-only | `npm run build` | N/A |
| VIEW-14 | Avicultura analytics dashboard | manual-only | `npm run build` | N/A |
| VIEW-15 | Relatorios PDF/Excel export | manual-only | `npm run build` | N/A |

### Sampling Rate
- **Per task commit:** `npm run build` (TypeScript type-check + Vite build)
- **Per wave merge:** Full build + manual spot check in browser
- **Phase gate:** Full build green + UAT of all 14 views against running Rust backend

### Wave 0 Gaps
None -- no test infrastructure needed. Build command (`vue-tsc --noEmit && vite build`) already provides type-checking validation. Manual UAT is the verification method per project conventions.

## Sources

### Primary (HIGH confidence)
- Rust backend `granjatech-api/src/handlers/mod.rs` -- all 60+ endpoint routes verified
- Rust backend `granjatech-api/src/dto/*.rs` -- all DTO shapes with camelCase serialization verified
- Vue scaffold `granjatech-frontend/src/` -- all Phase 4 artifacts verified in place
- React reference `frontend/src/pages/*.js` -- all 14 page implementations read
- npm registry -- vue-chartjs 5.3.3, chart.js 4.5.1, jspdf 4.2.1, jspdf-autotable 5.0.7, xlsx 0.18.5 versions verified

### Secondary (MEDIUM confidence)
- Vuetify 3 v-data-table header format `{ title, key }` -- from training knowledge of Vuetify 3 API
- vue-chartjs v5 component registration pattern -- from training knowledge

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all libraries verified via npm registry, same libs as React version
- Architecture: HIGH - patterns derived from existing Phase 4 code + React reference
- Pitfalls: HIGH - based on verified API differences (Vuetify 2->3, Chart.js 3->4, jspdf-autotable 4->5)
- API endpoints: HIGH - verified from Rust handler source code

**Research date:** 2026-04-08
**Valid until:** 2026-05-08 (stable libraries, no fast-moving concerns)
