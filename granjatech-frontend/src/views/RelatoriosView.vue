<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import { useExport } from '@/composables/useExport'
import { useFormatters } from '@/composables/useFormatters'

const { formatDate, formatCurrency } = useFormatters()
import type { ExportColumn } from '@/composables/useExport'

const { exportToPdf, exportToExcel } = useExport()

interface Granja {
  id: number
  nome: string
}

interface Lote {
  id: number
  codigo: string
  status: string
}

const activeTab = ref(0)
const reportData = ref<any>(null)
const loading = ref(false)
const error = ref('')
const granjas = ref<Granja[]>([])
const lotes = ref<Lote[]>([])
const loadingGranjas = ref(false)
const loadingLotes = ref(false)

// Filters
const granjaId = ref<number | null>(null)
const loteId = ref<number | null>(null)
const dataInicio = ref('')
const dataFim = ref('')
const tipoAvancado = ref('financeiro')

const tiposAvancado = [
  { title: 'Financeiro', value: 'financeiro' },
  { title: 'Producao', value: 'producao' },
  { title: 'Avicultura', value: 'avicultura' },
]

const tabs = [
  { title: 'Financeiro Simplificado', icon: 'mdi-cash-fast' },
  { title: 'Financeiro Completo', icon: 'mdi-cash-multiple' },
  { title: 'Producao', icon: 'mdi-factory' },
  { title: 'Avicultura', icon: 'mdi-bird' },
  { title: 'Desempenho Lote', icon: 'mdi-chart-areaspline' },
  { title: 'Avancado', icon: 'mdi-cog-outline' },
]

onMounted(async () => {
  await Promise.all([fetchGranjas(), fetchLotes()])
})

async function fetchGranjas() {
  try {
    loadingGranjas.value = true
    const { data } = await api.get('/granjas')
    granjas.value = data as Granja[]
    if (granjas.value.length === 1) {
      granjaId.value = granjas.value[0].id
    }
  } catch {
    error.value = 'Erro ao carregar granjas'
  } finally {
    loadingGranjas.value = false
  }
}

async function fetchLotes() {
  try {
    loadingLotes.value = true
    const { data } = await api.get('/lotes')
    lotes.value = data as Lote[]
    if (lotes.value.length === 1) {
      loteId.value = lotes.value[0].id
    }
  } catch {
    error.value = 'Erro ao carregar lotes'
  } finally {
    loadingLotes.value = false
  }
}

function clearReport() {
  reportData.value = null
  error.value = ''
}

async function gerarRelatorio() {
  reportData.value = null
  error.value = ''

  // Validate before setting loading state
  switch (activeTab.value) {
    case 0: // Financeiro Simplificado
      if (!granjaId.value) { error.value = 'Selecione uma granja'; return }
      break
    case 1: // Financeiro Completo
      if (!granjaId.value) { error.value = 'Selecione uma granja'; return }
      if (!dataInicio.value || !dataFim.value) { error.value = 'Selecione as datas'; return }
      break
    case 2: // Producao
      if (!granjaId.value) { error.value = 'Selecione uma granja'; return }
      break
    case 3: // Avicultura
      if (!loteId.value) { error.value = 'Selecione um lote'; return }
      break
    case 4: // Desempenho Lote
      if (!loteId.value) { error.value = 'Selecione um lote'; return }
      break
    case 5: // Avancado
      if (!granjaId.value) { error.value = 'Selecione uma granja'; return }
      if (!dataInicio.value || !dataFim.value) { error.value = 'Selecione as datas'; return }
      break
  }

  loading.value = true
  try {
    let response: any
    switch (activeTab.value) {
      case 0:
        response = await api.get('/relatorios/financeiro-simplificado', { params: { granjaId: granjaId.value } })
        break
      case 1:
        response = await api.get('/relatorios/financeiro', { params: { granjaId: granjaId.value, dataInicio: dataInicio.value, dataFim: dataFim.value } })
        break
      case 2:
        response = await api.get('/relatorios/producao', { params: { granjaId: granjaId.value } })
        break
      case 3:
        response = await api.get('/relatorios/avicultura', { params: { loteId: loteId.value } })
        break
      case 4:
        response = await api.get('/relatorios/desempenho-lote', { params: { loteId: loteId.value } })
        break
      case 5:
        response = await api.get('/relatorios/avancado', { params: { tipo: tipoAvancado.value, granjaId: granjaId.value, dataInicio: dataInicio.value, dataFim: dataFim.value } })
        break
    }
    reportData.value = response?.data ?? null
    if (!reportData.value) {
      error.value = 'Nenhum dado encontrado'
    }
  } catch (e: any) {
    error.value = `Erro ao gerar relatorio: ${e.response?.data?.message || e.message}`
  } finally {
    loading.value = false
  }
}

// Needs granja filter
const needsGranja = computed(() => [0, 1, 2, 5].includes(activeTab.value))
// Needs lote filter
const needsLote = computed(() => [3, 4].includes(activeTab.value))
// Needs date range
const needsDates = computed(() => [1, 5].includes(activeTab.value))
// Needs tipo avancado
const needsTipo = computed(() => activeTab.value === 5)

// Safe number formatting
function n(v: any): number {
  const x = Number(v)
  return Number.isFinite(x) ? x : 0
}

// Export columns per report type
function getExportColumns(): ExportColumn[] {
  switch (activeTab.value) {
    case 0:
    case 1:
      return [
        { header: 'Data', key: 'data' },
        { header: 'Tipo', key: 'tipo' },
        { header: 'Categoria', key: 'categoria' },
        { header: 'Descricao', key: 'descricao' },
        { header: 'Valor (R$)', key: 'valor' },
      ]
    case 2:
      return [
        { header: 'Codigo', key: 'codigo' },
        { header: 'Granja', key: 'granjaNome' },
        { header: 'Status', key: 'status' },
        { header: 'Viabilidade', key: 'viabilidade' },
        { header: 'IEP', key: 'iep' },
      ]
    case 3:
    case 4:
      return [
        { header: 'Metrica', key: 'nome' },
        { header: 'Valor', key: 'valor' },
        { header: 'Unidade', key: 'unidade' },
      ]
    case 5:
      return [
        { header: 'Item', key: 'item' },
        { header: 'Valor', key: 'valor' },
      ]
    default:
      return []
  }
}

function getExportData(): Record<string, any>[] {
  if (!reportData.value) return []
  const rd = reportData.value

  switch (activeTab.value) {
    case 0:
    case 1:
      return (rd.transacoes || rd.itens || []).map((t: any) => ({
        ...t,
        data: formatDate(t.data),
        valor: n(t.valor).toFixed(2),
      }))
    case 2:
      return rd.lotes || []
    case 3:
    case 4: {
      // Flatten metricas object into rows
      const metricas = rd.metricas || rd.indicadores || {}
      return Object.entries(metricas).map(([k, v]) => ({
        nome: k,
        valor: typeof v === 'number' ? (v as number).toFixed(2) : v,
        unidade: '',
      }))
    }
    case 5:
      if (Array.isArray(rd)) return rd
      return Object.entries(rd).map(([k, v]) => ({
        item: k,
        valor: typeof v === 'object' ? JSON.stringify(v) : v,
      }))
    default:
      return []
  }
}

function getReportTitle(): string {
  return tabs[activeTab.value]?.title || 'Relatorio'
}

function handleExportPdf() {
  exportToPdf(getReportTitle(), getExportColumns(), getExportData())
}

function handleExportExcel() {
  exportToExcel(getReportTitle().toLowerCase().replace(/\s+/g, '-'), getExportColumns(), getExportData())
}

// Table headers for financial reports
const financeiroHeaders = [
  { title: 'Data', key: 'data', value: (item: any) => formatDate(item.data) },
  { title: 'Tipo', key: 'tipo' },
  { title: 'Categoria', key: 'categoria' },
  { title: 'Descricao', key: 'descricao' },
  { title: 'Valor (R$)', key: 'valor', align: 'end' as const, value: (item: any) => formatCurrency(item.valor) },
]

const producaoHeaders = [
  { title: 'Codigo', key: 'codigo' },
  { title: 'Granja', key: 'granjaNome' },
  { title: 'Status', key: 'status' },
  { title: 'Viabilidade', key: 'viabilidade' },
  { title: 'IEP', key: 'iep' },
]

// Computed table items for current report
const tableItems = computed(() => {
  if (!reportData.value) return []
  const rd = reportData.value
  switch (activeTab.value) {
    case 0:
    case 1:
      return rd.transacoes || rd.itens || []
    case 2:
      return rd.lotes || []
    default:
      return []
  }
})

const currentHeaders = computed(() => {
  switch (activeTab.value) {
    case 0:
    case 1:
      return financeiroHeaders
    case 2:
      return producaoHeaders
    default:
      return []
  }
})

// Whether current report has data table display
const hasTableData = computed(() => [0, 1, 2].includes(activeTab.value) && tableItems.value.length > 0)

// Whether current report has card-style display (avicultura / desempenho)
const hasCardData = computed(() => [3, 4].includes(activeTab.value) && reportData.value)
</script>

<template>
  <PageContainer title="Relatorios" subtitle="Gere e exporte relatorios do sistema">
    <v-alert v-if="error" type="error" class="mb-4" closable @click:close="error = ''">
      {{ error }}
    </v-alert>

    <v-card>
      <v-tabs v-model="activeTab" color="primary" show-arrows @update:model-value="clearReport">
        <v-tab v-for="(tab, i) in tabs" :key="i" :value="i" :prepend-icon="tab.icon">
          {{ tab.title }}
        </v-tab>
      </v-tabs>

      <v-divider />

      <v-card-text>
        <v-tabs-window v-model="activeTab">
          <!-- Each tab window item shares same filter + generate pattern -->
          <v-tabs-window-item v-for="(_tab, i) in tabs" :key="i" :value="i">
            <!-- Filters -->
            <v-row class="mb-4" align="center">
              <!-- Granja selector -->
              <v-col v-if="needsGranja" cols="12" sm="4" md="3">
                <v-select
                  v-model="granjaId"
                  :items="granjas"
                  item-title="nome"
                  item-value="id"
                  label="Granja"
                  density="compact"
                  variant="outlined"
                  hide-details
                  :loading="loadingGranjas"
                />
              </v-col>

              <!-- Lote selector -->
              <v-col v-if="needsLote" cols="12" sm="4" md="3">
                <v-select
                  v-model="loteId"
                  :items="lotes"
                  item-title="codigo"
                  item-value="id"
                  label="Lote"
                  density="compact"
                  variant="outlined"
                  hide-details
                  :loading="loadingLotes"
                />
              </v-col>

              <!-- Date range -->
              <v-col v-if="needsDates" cols="12" sm="3" md="2">
                <v-text-field
                  v-model="dataInicio"
                  label="Data Inicio"
                  type="date"
                  density="compact"
                  variant="outlined"
                  hide-details
                />
              </v-col>
              <v-col v-if="needsDates" cols="12" sm="3" md="2">
                <v-text-field
                  v-model="dataFim"
                  label="Data Fim"
                  type="date"
                  density="compact"
                  variant="outlined"
                  hide-details
                />
              </v-col>

              <!-- Tipo avancado -->
              <v-col v-if="needsTipo" cols="12" sm="3" md="2">
                <v-select
                  v-model="tipoAvancado"
                  :items="tiposAvancado"
                  item-title="title"
                  item-value="value"
                  label="Tipo"
                  density="compact"
                  variant="outlined"
                  hide-details
                />
              </v-col>

              <!-- Generate button -->
              <v-col cols="12" sm="3" md="2">
                <v-btn
                  color="primary"
                  :loading="loading"
                  prepend-icon="mdi-file-chart-outline"
                  @click="gerarRelatorio"
                  block
                >
                  Gerar Relatorio
                </v-btn>
              </v-col>
            </v-row>

            <v-progress-linear v-if="loading" indeterminate color="primary" class="mb-4" />

            <!-- Report Results -->
            <template v-if="reportData && !loading">
              <!-- Financial summary cards -->
              <v-row v-if="[0, 1].includes(activeTab) && (reportData.totalEntradas != null || reportData.totalSaidas != null)" class="mb-4">
                <v-col cols="12" sm="4">
                  <v-card color="green-lighten-5" variant="tonal">
                    <v-card-text class="text-center">
                      <div class="text-caption text-medium-emphasis">Total Entradas</div>
                      <div class="text-h6 text-success font-weight-bold">{{ formatCurrency(reportData.totalEntradas) }}</div>
                    </v-card-text>
                  </v-card>
                </v-col>
                <v-col cols="12" sm="4">
                  <v-card color="red-lighten-5" variant="tonal">
                    <v-card-text class="text-center">
                      <div class="text-caption text-medium-emphasis">Total Saidas</div>
                      <div class="text-h6 text-error font-weight-bold">{{ formatCurrency(reportData.totalSaidas) }}</div>
                    </v-card-text>
                  </v-card>
                </v-col>
                <v-col cols="12" sm="4">
                  <v-card :color="n(reportData.saldo) >= 0 ? 'green-lighten-5' : 'red-lighten-5'" variant="tonal">
                    <v-card-text class="text-center">
                      <div class="text-caption text-medium-emphasis">Saldo</div>
                      <div class="text-h6 font-weight-bold" :class="n(reportData.saldo) >= 0 ? 'text-success' : 'text-error'">
                        {{ formatCurrency(reportData.saldo) }}
                      </div>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>

              <!-- Production summary -->
              <v-row v-if="activeTab === 2 && reportData.indicadores" class="mb-4">
                <v-col v-for="(val, key) in reportData.indicadores" :key="String(key)" cols="12" sm="4" md="3">
                  <v-card variant="tonal">
                    <v-card-text class="text-center">
                      <div class="text-caption text-medium-emphasis">{{ String(key) }}</div>
                      <div class="text-h6 font-weight-bold">{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</div>
                    </v-card-text>
                  </v-card>
                </v-col>
              </v-row>

              <!-- Data table for financial / production reports -->
              <v-data-table
                v-if="hasTableData"
                :headers="currentHeaders"
                :items="tableItems"
                :items-per-page="15"
                class="mb-4"
                density="compact"
              >
                <template #item.valor="{ item }: { item: any }">
                  {{ formatCurrency(item.valor) }}
                </template>
                <template #item.categoria="{ item }: { item: any }">
                  <v-chip
                    :color="item.categoria === 'entrada' || item.tipo === 'Entrada' ? 'success' : 'error'"
                    size="small"
                  >
                    {{ item.categoria }}
                  </v-chip>
                </template>
                <template #item.data="{ item }: { item: any }">
                  {{ formatDate(item.data) }}
                </template>
              </v-data-table>

              <!-- Card display for avicultura / desempenho -->
              <template v-if="hasCardData">
                <!-- Lote info -->
                <v-card v-if="reportData.lote" class="mb-4" variant="tonal">
                  <v-card-title>Lote: {{ reportData.lote.codigo || reportData.lote.identificador }}</v-card-title>
                  <v-card-text>
                    <v-row>
                      <v-col v-for="(val, key) in reportData.lote" :key="String(key)" cols="6" sm="4" md="3">
                        <div class="text-caption text-medium-emphasis">{{ String(key) }}</div>
                        <div class="font-weight-medium">{{ typeof val === 'number' ? n(val).toFixed(2) : val ?? '-' }}</div>
                      </v-col>
                    </v-row>
                  </v-card-text>
                </v-card>

                <!-- Metricas / indicadores -->
                <v-card v-if="reportData.metricas || reportData.indicadores" class="mb-4">
                  <v-card-title>Metricas</v-card-title>
                  <v-card-text>
                    <v-row>
                      <v-col
                        v-for="(val, key) in (reportData.metricas || reportData.indicadores)"
                        :key="String(key)"
                        cols="6"
                        sm="4"
                        md="3"
                      >
                        <v-card variant="outlined" class="pa-3 text-center">
                          <div class="text-caption text-medium-emphasis">{{ String(key) }}</div>
                          <div class="text-h6 font-weight-bold">{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</div>
                        </v-card>
                      </v-col>
                    </v-row>
                  </v-card-text>
                </v-card>

                <!-- Historico tables if present -->
                <v-card v-if="reportData.historicoPesagens && reportData.historicoPesagens.length > 0" class="mb-4">
                  <v-card-title>Historico de Pesagens</v-card-title>
                  <v-card-text>
                    <v-data-table
                      :headers="[
                        { title: 'Data', key: 'data', value: (item: any) => formatDate(item.data) },
                        { title: 'Peso Medio (kg)', key: 'pesoMedioKg' },
                        { title: 'Amostra', key: 'amostra' },
                      ]"
                      :items="reportData.historicoPesagens"
                      :items-per-page="10"
                      density="compact"
                    >
                      <template #item.data="{ item }: { item: any }">{{ formatDate(item.data) }}</template>
                    </v-data-table>
                  </v-card-text>
                </v-card>

                <v-card v-if="reportData.historicoConsumo && reportData.historicoConsumo.length > 0" class="mb-4">
                  <v-card-title>Historico de Consumo</v-card-title>
                  <v-card-text>
                    <v-data-table
                      :headers="[
                        { title: 'Data', key: 'data', value: (item: any) => formatDate(item.data) },
                        { title: 'Racao (kg)', key: 'racaoKg' },
                        { title: 'Agua (L)', key: 'aguaLitros' },
                      ]"
                      :items="reportData.historicoConsumo"
                      :items-per-page="10"
                      density="compact"
                    >
                      <template #item.data="{ item }: { item: any }">{{ formatDate(item.data) }}</template>
                    </v-data-table>
                  </v-card-text>
                </v-card>

                <!-- Consumo / pesagens / sanitario summaries for avicultura report -->
                <v-row v-if="reportData.consumo || reportData.pesagens || reportData.sanitario">
                  <v-col v-if="reportData.consumo" cols="12" md="4">
                    <v-card>
                      <v-card-title class="text-body-1">Consumo</v-card-title>
                      <v-card-text>
                        <div v-if="Array.isArray(reportData.consumo)">{{ reportData.consumo.length }} registros</div>
                        <div v-else v-for="(val, key) in reportData.consumo" :key="String(key)" class="d-flex justify-space-between py-1">
                          <span class="text-medium-emphasis">{{ String(key) }}</span>
                          <span>{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</span>
                        </div>
                      </v-card-text>
                    </v-card>
                  </v-col>
                  <v-col v-if="reportData.pesagens" cols="12" md="4">
                    <v-card>
                      <v-card-title class="text-body-1">Pesagens</v-card-title>
                      <v-card-text>
                        <div v-if="Array.isArray(reportData.pesagens)">{{ reportData.pesagens.length }} registros</div>
                        <div v-else v-for="(val, key) in reportData.pesagens" :key="String(key)" class="d-flex justify-space-between py-1">
                          <span class="text-medium-emphasis">{{ String(key) }}</span>
                          <span>{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</span>
                        </div>
                      </v-card-text>
                    </v-card>
                  </v-col>
                  <v-col v-if="reportData.sanitario" cols="12" md="4">
                    <v-card>
                      <v-card-title class="text-body-1">Sanitario</v-card-title>
                      <v-card-text>
                        <div v-if="Array.isArray(reportData.sanitario)">{{ reportData.sanitario.length }} registros</div>
                        <div v-else v-for="(val, key) in reportData.sanitario" :key="String(key)" class="d-flex justify-space-between py-1">
                          <span class="text-medium-emphasis">{{ String(key) }}</span>
                          <span>{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</span>
                        </div>
                      </v-card-text>
                    </v-card>
                  </v-col>
                </v-row>
              </template>

              <!-- Advanced report - generic display -->
              <template v-if="activeTab === 5 && reportData">
                <v-card class="mb-4">
                  <v-card-title>Resultado do Relatorio Avancado</v-card-title>
                  <v-card-text>
                    <v-row>
                      <v-col
                        v-for="(val, key) in reportData"
                        :key="String(key)"
                        cols="12"
                        sm="6"
                        md="4"
                      >
                        <template v-if="typeof val !== 'object'">
                          <v-card variant="outlined" class="pa-3 text-center">
                            <div class="text-caption text-medium-emphasis">{{ String(key) }}</div>
                            <div class="text-h6 font-weight-bold">{{ typeof val === 'number' ? n(val).toFixed(2) : val }}</div>
                          </v-card>
                        </template>
                      </v-col>
                    </v-row>
                  </v-card-text>
                </v-card>
              </template>

              <!-- Export buttons -->
              <v-divider class="my-4" />
              <div class="d-flex ga-3">
                <v-btn
                  color="red"
                  variant="tonal"
                  prepend-icon="mdi-file-pdf-box"
                  @click="handleExportPdf"
                >
                  Exportar PDF
                </v-btn>
                <v-btn
                  color="green"
                  variant="tonal"
                  prepend-icon="mdi-file-excel"
                  @click="handleExportExcel"
                >
                  Exportar Excel
                </v-btn>
              </div>
            </template>
          </v-tabs-window-item>
        </v-tabs-window>
      </v-card-text>
    </v-card>
  </PageContainer>
</template>
