<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { Bar, Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import { useFormatters } from '@/composables/useFormatters'

const { formatDate } = useFormatters()

ChartJS.register(CategoryScale, LinearScale, BarElement, PointElement, LineElement, Title, Tooltip, Legend, Filler)

interface Lote {
  id: number
  codigo: string
  identificador?: string
  status: string
  idadeAtualDias?: number
  quantidadeAvesAtual?: number
  viabilidade?: number
  linhagem?: string
  granjaNome?: string
}

interface Metrica {
  viabilidade: number
  ca: number
  iep: number
  pesoMedio: number
  ganhoMedioDiario: number
  idadeDias: number
  mortalidadeAcumulada: number
}

interface Alerta {
  tipo: string
  mensagem: string
  severidade: string
  tipoAlerta?: string
  descricao?: string
  valorAtual?: number
  unidade?: string
  recomendacao?: string
}

interface CurvaPonto {
  dia: number
  pesoReal: number
  pesoEsperado: number
}

interface ComparacaoMetrica {
  nome: string
  metrica?: string
  valorAtual: number
  valor?: number
  valorPadraoIndustria: number
  status: string
  unidade: string
}

interface ComparacaoIndustria {
  metricas: ComparacaoMetrica[]
  industria: ComparacaoMetrica[]
  pontuacaoGeral?: number
  classificacaoGeral?: string
  conversaoAlimentar?: ComparacaoMetrica
  ganhoMedioDiario?: ComparacaoMetrica
  viabilidade?: ComparacaoMetrica
  iep?: ComparacaoMetrica
}

interface ProjecaoAbate {
  dataProjetada: string
  pesoProjetado: number
  idadeAbate: number
}

interface Dashboard {
  metricas: Metrica
  alertas: Alerta[]
  resumoConsumo: any
  resumoPesagem: any
  resumoSanitario: any
}

const lotes = ref<Lote[]>([])
const selectedLoteId = ref<number | null>(null)
const dashboard = ref<Dashboard | null>(null)
const curvas = ref<CurvaPonto[]>([])
const comparacao = ref<ComparacaoIndustria | null>(null)
const projecao = ref<ProjecaoAbate | null>(null)
const loading = ref(true)
const loadingDetalhes = ref(false)
const error = ref('')

const selectedLote = ref<Lote | null>(null)

onMounted(async () => {
  try {
    loading.value = true
    const { data } = await api.get('/lotes')
    const ativos = (data as Lote[]).filter((l: Lote) => l.status === 'Ativo')
    lotes.value = ativos
    if (ativos.length > 0) {
      selectedLoteId.value = ativos[0].id
      selectedLote.value = ativos[0]
    }
  } catch {
    error.value = 'Erro ao carregar lotes ativos'
  } finally {
    loading.value = false
  }
})

async function fetchDashboardData(loteId: number) {
  selectedLote.value = lotes.value.find(l => l.id === loteId) || null
  try {
    loadingDetalhes.value = true
    error.value = ''

    const [dashRes, curvasRes, comparacaoRes, projecaoRes] = await Promise.all([
      api.get(`/avicultura/${loteId}/dashboard`),
      api.get(`/avicultura/${loteId}/curvas-crescimento`),
      api.get(`/avicultura/${loteId}/comparacao-industria`),
      api.get(`/avicultura/${loteId}/projecao-abate`),
    ])

    dashboard.value = dashRes.data as Dashboard
    curvas.value = (curvasRes.data as CurvaPonto[]) || []
    comparacao.value = comparacaoRes.data as ComparacaoIndustria
    projecao.value = projecaoRes.data as ProjecaoAbate
  } catch {
    error.value = 'Erro ao carregar dados do lote'
  } finally {
    loadingDetalhes.value = false
  }
}

watch(selectedLoteId, async (loteId) => {
  if (!loteId) return
  await fetchDashboardData(loteId)
})

function getSeverityColor(severidade: string): string {
  switch (severidade?.toLowerCase()) {
    case 'alta':
    case 'critica':
      return 'error'
    case 'media':
      return 'warning'
    case 'baixa':
      return 'info'
    default:
      return 'info'
  }
}

function getSeverityType(severidade: string): 'error' | 'warning' | 'info' | 'success' {
  switch (severidade?.toLowerCase()) {
    case 'alta':
    case 'critica':
      return 'error'
    case 'media':
      return 'warning'
    case 'baixa':
      return 'info'
    default:
      return 'info'
  }
}

function formatNumber(val: number | undefined, decimals = 2): string {
  if (val == null || isNaN(val)) return '-'
  return val.toLocaleString('pt-BR', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  })
}

// Chart data for growth curves
function getCurvasChartData() {
  return {
    labels: curvas.value.map(c => `Dia ${c.dia}`),
    datasets: [
      {
        label: 'Peso Real',
        data: curvas.value.map(c => c.pesoReal),
        borderColor: '#4caf50',
        backgroundColor: 'rgba(76, 175, 80, 0.1)',
        borderWidth: 2,
        fill: false,
        tension: 0.3,
      },
      {
        label: 'Peso Esperado',
        data: curvas.value.map(c => c.pesoEsperado),
        borderColor: '#9e9e9e',
        backgroundColor: 'rgba(158, 158, 158, 0.1)',
        borderWidth: 2,
        borderDash: [5, 5],
        fill: false,
        tension: 0.3,
      },
    ],
  }
}

function getCurvasChartOptions() {
  return {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { position: 'top' as const },
      title: { display: false },
    },
    scales: {
      x: { title: { display: true, text: 'Dia' } },
      y: { title: { display: true, text: 'Peso (g)' } },
    },
  }
}

// Chart data for industry comparison
function getComparacaoChartData() {
  const metricas = comparacao.value?.metricas || []
  const industria = comparacao.value?.industria || []

  // If metricas/industria arrays exist, use them
  if (metricas.length > 0) {
    return {
      labels: metricas.map((m: ComparacaoMetrica) => m.nome || m.metrica),
      datasets: [
        {
          label: 'Lote',
          data: metricas.map((m: ComparacaoMetrica) => m.valorAtual ?? m.valor),
          backgroundColor: 'rgba(46, 125, 50, 0.7)',
          borderColor: '#2E7D32',
          borderWidth: 1,
        },
        {
          label: 'Industria',
          data: industria.length > 0
            ? industria.map((m: ComparacaoMetrica) => m.valorPadraoIndustria ?? m.valor)
            : metricas.map((m: ComparacaoMetrica) => m.valorPadraoIndustria ?? 0),
          backgroundColor: 'rgba(158, 158, 158, 0.7)',
          borderColor: '#9e9e9e',
          borderWidth: 1,
        },
      ],
    }
  }

  // Fallback: use individual metric properties from comparacao
  const comp = comparacao.value
  if (comp?.conversaoAlimentar) {
    const items = [comp.conversaoAlimentar, comp.ganhoMedioDiario, comp.viabilidade, comp.iep].filter((m): m is ComparacaoMetrica => Boolean(m))
    return {
      labels: items.map((m: ComparacaoMetrica) => m.nome),
      datasets: [
        {
          label: 'Lote',
          data: items.map((m: ComparacaoMetrica) => m.valorAtual),
          backgroundColor: 'rgba(46, 125, 50, 0.7)',
          borderColor: '#2E7D32',
          borderWidth: 1,
        },
        {
          label: 'Industria',
          data: items.map((m: ComparacaoMetrica) => m.valorPadraoIndustria),
          backgroundColor: 'rgba(158, 158, 158, 0.7)',
          borderColor: '#9e9e9e',
          borderWidth: 1,
        },
      ],
    }
  }

  return { labels: [], datasets: [] }
}

function getComparacaoChartOptions() {
  return {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { position: 'top' as const },
      title: { display: false },
    },
    scales: {
      y: { beginAtZero: true },
    },
  }
}

const metricaCards = [
  { key: 'viabilidade', label: 'Viabilidade', unit: '%', icon: 'mdi-heart-pulse', color: 'red' },
  { key: 'iep', label: 'IEP', unit: '', icon: 'mdi-speedometer', color: 'blue' },
  { key: 'ca', label: 'Conversao Alimentar', unit: '', icon: 'mdi-scale-balance', color: 'orange' },
  { key: 'pesoMedio', label: 'Peso Medio', unit: 'g', icon: 'mdi-weight', color: 'green' },
  { key: 'ganhoMedioDiario', label: 'Ganho Medio Diario', unit: 'g/dia', icon: 'mdi-trending-up', color: 'teal' },
  { key: 'idadeDias', label: 'Idade', unit: 'dias', icon: 'mdi-calendar', color: 'purple' },
  { key: 'mortalidadeAcumulada', label: 'Mortalidade Acumulada', unit: '%', icon: 'mdi-alert-circle', color: 'deep-orange' },
]
</script>

<template>
  <PageContainer title="Avicultura Pro" subtitle="Dashboard de analise avicola por lote">
    <template #action>
      <div class="d-flex align-center ga-2">
        <v-select
          v-model="selectedLoteId"
          :items="lotes"
          item-title="codigo"
          item-value="id"
          label="Selecionar Lote"
          density="compact"
          hide-details
          style="min-width: 220px"
          variant="outlined"
        />
        <v-btn
          icon="mdi-refresh"
          variant="text"
          :loading="loadingDetalhes"
          @click="selectedLoteId && fetchDashboardData(selectedLoteId)"
        />
      </div>
    </template>

    <v-alert v-if="error" type="error" class="mb-4" closable @click:close="error = ''">
      {{ error }}
    </v-alert>

    <v-progress-linear v-if="loading" indeterminate color="primary" class="mb-4" />

    <v-alert v-if="!loading && lotes.length === 0" type="info" class="mb-4">
      Nenhum lote ativo encontrado. Crie um lote para visualizar as metricas de avicultura.
    </v-alert>

    <template v-if="selectedLote && !loading">
      <!-- Lote Info Card -->
      <v-card class="mb-4">
        <v-card-text>
          <v-row>
            <v-col cols="12" md="3">
              <div class="text-caption text-medium-emphasis">Lote</div>
              <div class="text-h6">{{ selectedLote.codigo || selectedLote.identificador }}</div>
            </v-col>
            <v-col cols="12" md="2">
              <div class="text-caption text-medium-emphasis">Idade</div>
              <div class="text-h6">{{ selectedLote.idadeAtualDias ?? '-' }} dias</div>
            </v-col>
            <v-col cols="12" md="2">
              <div class="text-caption text-medium-emphasis">Aves Atuais</div>
              <div class="text-h6">{{ selectedLote.quantidadeAvesAtual?.toLocaleString() ?? '-' }}</div>
            </v-col>
            <v-col cols="12" md="2">
              <div class="text-caption text-medium-emphasis">Viabilidade</div>
              <div class="text-h6" :class="(selectedLote.viabilidade ?? 0) >= 95 ? 'text-success' : 'text-warning'">
                {{ formatNumber(selectedLote.viabilidade, 1) }}%
              </div>
            </v-col>
            <v-col cols="12" md="3">
              <div class="text-caption text-medium-emphasis">Linhagem</div>
              <div class="text-h6">{{ selectedLote.linhagem || 'N/A' }}</div>
            </v-col>
          </v-row>
        </v-card-text>
      </v-card>

      <v-progress-linear v-if="loadingDetalhes" indeterminate color="primary" class="mb-4" />

      <template v-if="!loadingDetalhes && dashboard">
        <!-- Section 1: Metricas KPI Cards -->
        <v-row class="mb-4">
          <v-col
            v-for="m in metricaCards"
            :key="m.key"
            cols="12"
            sm="6"
            md="3"
            lg
          >
            <v-card :style="{ borderTop: `3px solid rgb(var(--v-theme-${m.color}))` }">
              <v-card-text>
                <div class="d-flex justify-space-between align-start mb-2">
                  <span class="text-caption text-medium-emphasis">{{ m.label }}</span>
                  <v-icon :icon="m.icon" :color="m.color" size="20" />
                </div>
                <div class="text-h5 font-weight-bold">
                  {{ formatNumber((dashboard.metricas as any)[m.key], m.key === 'iep' || m.key === 'idadeDias' ? 0 : 2) }}
                  <span v-if="m.unit" class="text-body-2 text-medium-emphasis ml-1">{{ m.unit }}</span>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>

        <!-- Section 2: Alertas -->
        <v-card v-if="dashboard.alertas && dashboard.alertas.length > 0" class="mb-4">
          <v-card-title class="d-flex align-center ga-2">
            <v-icon icon="mdi-alert" color="warning" />
            Alertas e Recomendacoes
          </v-card-title>
          <v-card-text>
            <v-row>
              <v-col
                v-for="(alerta, i) in dashboard.alertas"
                :key="i"
                cols="12"
                md="6"
              >
                <v-alert
                  :type="getSeverityType(alerta.severidade)"
                  :color="getSeverityColor(alerta.severidade)"
                  variant="tonal"
                  class="mb-2"
                >
                  <div class="font-weight-medium">
                    {{ alerta.tipoAlerta || alerta.tipo }}: {{ alerta.descricao || alerta.mensagem }}
                  </div>
                  <div v-if="alerta.valorAtual != null" class="text-body-2">
                    Valor atual: {{ alerta.valorAtual }} {{ alerta.unidade || '' }}
                  </div>
                  <div v-if="alerta.recomendacao" class="text-body-2 mt-1 font-italic">
                    {{ alerta.recomendacao }}
                  </div>
                </v-alert>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>

        <!-- Section 3: Curvas de Crescimento -->
        <v-card v-if="curvas.length > 0" class="mb-4">
          <v-card-title class="d-flex align-center ga-2">
            <v-icon icon="mdi-chart-line" color="primary" />
            Curvas de Crescimento
          </v-card-title>
          <v-card-text>
            <div style="height: 350px">
              <Line :data="getCurvasChartData()" :options="getCurvasChartOptions()" />
            </div>
          </v-card-text>
        </v-card>

        <!-- Section 4: Comparacao com Industria -->
        <v-card v-if="comparacao" class="mb-4">
          <v-card-title class="d-flex align-center ga-2">
            <v-icon icon="mdi-chart-bar" color="primary" />
            Comparacao com Industria
          </v-card-title>
          <v-card-text>
            <div v-if="comparacao.pontuacaoGeral != null" class="d-flex align-center ga-3 mb-4">
              <span class="text-h4 text-primary font-weight-bold">{{ comparacao.pontuacaoGeral }}</span>
              <v-chip v-if="comparacao.classificacaoGeral" :color="comparacao.classificacaoGeral === 'Excelente' ? 'success' : comparacao.classificacaoGeral === 'Bom' ? 'primary' : 'warning'" size="large">
                {{ comparacao.classificacaoGeral }}
              </v-chip>
            </div>
            <div style="height: 350px">
              <Bar :data="getComparacaoChartData()" :options="getComparacaoChartOptions()" />
            </div>
          </v-card-text>
        </v-card>

        <!-- Section 5: Projecao de Abate -->
        <v-card v-if="projecao" class="mb-4">
          <v-card-title class="d-flex align-center ga-2">
            <v-icon icon="mdi-calendar-clock" color="primary" />
            Projecao de Abate
          </v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="12" sm="4">
                <div class="text-caption text-medium-emphasis">Data Projetada</div>
                <div class="text-h6">{{ formatDate(projecao.dataProjetada) }}</div>
              </v-col>
              <v-col cols="12" sm="4">
                <div class="text-caption text-medium-emphasis">Peso Projetado</div>
                <div class="text-h6">{{ formatNumber(projecao.pesoProjetado) }} g</div>
              </v-col>
              <v-col cols="12" sm="4">
                <div class="text-caption text-medium-emphasis">Idade de Abate</div>
                <div class="text-h6">{{ projecao.idadeAbate }} dias</div>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>

        <!-- Section 6: Resumos -->
        <v-row v-if="dashboard.resumoConsumo || dashboard.resumoPesagem || dashboard.resumoSanitario">
          <v-col v-if="dashboard.resumoConsumo" cols="12" md="4">
            <v-card>
              <v-card-title class="d-flex align-center ga-2">
                <v-icon icon="mdi-food-drumstick" color="orange" size="20" />
                Resumo Consumo
              </v-card-title>
              <v-card-text>
                <div v-for="(val, key) in dashboard.resumoConsumo" :key="String(key)" class="d-flex justify-space-between py-1">
                  <span class="text-medium-emphasis">{{ String(key) }}</span>
                  <span class="font-weight-medium">{{ typeof val === 'number' ? formatNumber(val) : val }}</span>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
          <v-col v-if="dashboard.resumoPesagem" cols="12" md="4">
            <v-card>
              <v-card-title class="d-flex align-center ga-2">
                <v-icon icon="mdi-scale" color="blue" size="20" />
                Resumo Pesagem
              </v-card-title>
              <v-card-text>
                <div v-for="(val, key) in dashboard.resumoPesagem" :key="String(key)" class="d-flex justify-space-between py-1">
                  <span class="text-medium-emphasis">{{ String(key) }}</span>
                  <span class="font-weight-medium">{{ typeof val === 'number' ? formatNumber(val) : val }}</span>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
          <v-col v-if="dashboard.resumoSanitario" cols="12" md="4">
            <v-card>
              <v-card-title class="d-flex align-center ga-2">
                <v-icon icon="mdi-medical-bag" color="red" size="20" />
                Resumo Sanitario
              </v-card-title>
              <v-card-text>
                <div v-for="(val, key) in dashboard.resumoSanitario" :key="String(key)" class="d-flex justify-space-between py-1">
                  <span class="text-medium-emphasis">{{ String(key) }}</span>
                  <span class="font-weight-medium">{{ typeof val === 'number' ? formatNumber(val) : val }}</span>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </template>
    </template>
  </PageContainer>
</template>
