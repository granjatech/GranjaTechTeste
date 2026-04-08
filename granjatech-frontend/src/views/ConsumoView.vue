<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler)

interface Lote {
  id: number
  codigo: string
  granjaNome: string
  status: string
}

interface ConsumoItem {
  id: number
  loteId: number
  data: string
  quantidade: number
  tipo?: string
  observacoes?: string
}

interface ResumoConsumo {
  totalRacao: number
  totalAgua: number
  mediaRacaoDiaria: number
  mediaAguaDiaria: number
}

const lotes = ref<Lote[]>([])
const selectedLoteId = ref<number | null>(null)
const consumosRacao = ref<ConsumoItem[]>([])
const consumosAgua = ref<ConsumoItem[]>([])
const resumo = ref<ResumoConsumo | null>(null)
const loading = ref(true)
const loadingData = ref(false)
const error = ref('')

// Dialogs
const dialogRacao = ref(false)
const dialogAgua = ref(false)
const submitting = ref(false)

const formRacao = ref({
  data: new Date().toISOString().split('T')[0],
  quantidade: null as number | null,
  observacoes: '',
})

const formAgua = ref({
  data: new Date().toISOString().split('T')[0],
  quantidade: null as number | null,
  observacoes: '',
})

const quantidadeRules = [
  (v: number | null) => (v !== null && v !== undefined) || 'Quantidade e obrigatoria',
  (v: number | null) => (v !== null && v >= 0.01) || 'Quantidade deve ser maior que 0',
]

const dateRules = [
  (v: string) => !!v || 'Data e obrigatoria',
]

async function fetchLotes() {
  try {
    loading.value = true
    const res = await api.get('/lotes')
    const lotesAtivos = (res.data || []).filter((l: Lote) => l.status === 'Ativo')
    lotes.value = lotesAtivos
    if (lotesAtivos.length > 0 && !selectedLoteId.value) {
      selectedLoteId.value = lotesAtivos[0].id
    }
  } catch (err) {
    console.error('Erro ao buscar lotes:', err)
    error.value = 'Erro ao carregar lotes ativos'
  } finally {
    loading.value = false
  }
}

async function fetchConsumoData(loteId: number) {
  if (!loteId) return
  try {
    loadingData.value = true
    error.value = ''
    const [racaoRes, aguaRes, resumoRes] = await Promise.all([
      api.get(`/consumo/racao/${loteId}`),
      api.get(`/consumo/agua/${loteId}`),
      api.get(`/consumo/resumo/${loteId}`),
    ])
    consumosRacao.value = racaoRes.data || []
    consumosAgua.value = aguaRes.data || []
    resumo.value = resumoRes.data
  } catch (err) {
    console.error('Erro ao buscar dados de consumo:', err)
    error.value = 'Erro ao carregar dados de consumo'
  } finally {
    loadingData.value = false
  }
}

watch(selectedLoteId, (newId) => {
  if (newId) fetchConsumoData(newId)
})

onMounted(() => {
  fetchLotes()
})

async function submitRacao() {
  if (!selectedLoteId.value || !formRacao.value.quantidade || formRacao.value.quantidade < 0.01) return
  try {
    submitting.value = true
    await api.post('/consumo/racao', {
      loteId: selectedLoteId.value,
      data: formRacao.value.data,
      quantidade: Number(formRacao.value.quantidade),
      observacoes: formRacao.value.observacoes || null,
    })
    dialogRacao.value = false
    formRacao.value = { data: new Date().toISOString().split('T')[0], quantidade: null, observacoes: '' }
    await fetchConsumoData(selectedLoteId.value)
  } catch (err) {
    console.error('Erro ao registrar consumo de racao:', err)
    error.value = 'Erro ao registrar consumo de racao'
  } finally {
    submitting.value = false
  }
}

async function submitAgua() {
  if (!selectedLoteId.value || !formAgua.value.quantidade || formAgua.value.quantidade < 0.01) return
  try {
    submitting.value = true
    await api.post('/consumo/agua', {
      loteId: selectedLoteId.value,
      data: formAgua.value.data,
      quantidade: Number(formAgua.value.quantidade),
      observacoes: formAgua.value.observacoes || null,
    })
    dialogAgua.value = false
    formAgua.value = { data: new Date().toISOString().split('T')[0], quantidade: null, observacoes: '' }
    await fetchConsumoData(selectedLoteId.value)
  } catch (err) {
    console.error('Erro ao registrar consumo de agua:', err)
    error.value = 'Erro ao registrar consumo de agua'
  } finally {
    submitting.value = false
  }
}

// Table headers
const racaoHeaders = [
  { title: 'Data', key: 'data' },
  { title: 'Quantidade (kg)', key: 'quantidade' },
  { title: 'Observacoes', key: 'observacoes' },
]

const aguaHeaders = [
  { title: 'Data', key: 'data' },
  { title: 'Quantidade (L)', key: 'quantidade' },
  { title: 'Observacoes', key: 'observacoes' },
]

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString('pt-BR')
  } catch {
    return dateStr
  }
}

// Chart data
const racaoChartData = computed(() => {
  const sorted = [...consumosRacao.value].sort(
    (a, b) => new Date(a.data).getTime() - new Date(b.data).getTime()
  )
  return {
    labels: sorted.map((i) => formatDate(i.data)),
    datasets: [
      {
        label: 'Consumo de Racao (kg)',
        data: sorted.map((i) => i.quantidade),
        borderColor: '#4caf50',
        backgroundColor: 'rgba(76, 175, 80, 0.15)',
        fill: true,
        tension: 0.3,
      },
    ],
  }
})

const aguaChartData = computed(() => {
  const sorted = [...consumosAgua.value].sort(
    (a, b) => new Date(a.data).getTime() - new Date(b.data).getTime()
  )
  return {
    labels: sorted.map((i) => formatDate(i.data)),
    datasets: [
      {
        label: 'Consumo de Agua (L)',
        data: sorted.map((i) => i.quantidade),
        borderColor: '#2196f3',
        backgroundColor: 'rgba(33, 150, 243, 0.15)',
        fill: true,
        tension: 0.3,
      },
    ],
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: true },
  },
}
</script>

<template>
  <PageContainer title="Consumo" subtitle="Registro de consumo de racao e agua por lote">
    <template #action>
      <v-row align="center" class="ga-2" no-gutters>
        <v-col cols="auto">
          <v-select
            v-model="selectedLoteId"
            :items="lotes"
            item-title="codigo"
            item-value="id"
            label="Selecione um Lote"
            density="compact"
            hide-details
            variant="outlined"
            style="min-width: 220px"
          />
        </v-col>
      </v-row>
    </template>

    <v-alert v-if="error" type="error" closable class="mb-4" @click:close="error = ''">
      {{ error }}
    </v-alert>

    <v-progress-linear v-if="loading || loadingData" indeterminate color="primary" class="mb-4" />

    <v-alert v-if="!loading && lotes.length === 0" type="info" class="mb-4">
      Nenhum lote ativo encontrado. Crie um lote para registrar consumos.
    </v-alert>

    <template v-if="selectedLoteId && !loading">
      <!-- Summary Cards -->
      <v-row v-if="resumo" class="mb-4">
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Total Racao</div>
              <div class="text-h5 font-weight-bold text-success">
                {{ resumo.totalRacao?.toFixed(1) ?? '0' }} kg
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Total Agua</div>
              <div class="text-h5 font-weight-bold text-info">
                {{ resumo.totalAgua?.toFixed(1) ?? '0' }} L
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Media Racao/Dia</div>
              <div class="text-h5 font-weight-bold text-primary">
                {{ resumo.mediaRacaoDiaria?.toFixed(1) ?? '0' }} kg
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Media Agua/Dia</div>
              <div class="text-h5 font-weight-bold text-primary">
                {{ resumo.mediaAguaDiaria?.toFixed(1) ?? '0' }} L
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <!-- Data Tables -->
      <v-row class="mb-4">
        <v-col cols="12" md="6">
          <v-card>
            <v-card-title class="d-flex align-center justify-space-between">
              <span>Consumo de Racao</span>
              <v-btn color="primary" size="small" prepend-icon="mdi-plus" @click="dialogRacao = true">
                Registrar
              </v-btn>
            </v-card-title>
            <v-data-table
              :headers="racaoHeaders"
              :items="consumosRacao"
              :loading="loadingData"
              density="compact"
              no-data-text="Nenhum registro de consumo de racao"
            >
              <template #item.data="{ item }">
                {{ formatDate(item.data) }}
              </template>
              <template #item.observacoes="{ item }">
                {{ item.observacoes || '-' }}
              </template>
            </v-data-table>
          </v-card>
        </v-col>
        <v-col cols="12" md="6">
          <v-card>
            <v-card-title class="d-flex align-center justify-space-between">
              <span>Consumo de Agua</span>
              <v-btn color="primary" size="small" prepend-icon="mdi-plus" @click="dialogAgua = true">
                Registrar
              </v-btn>
            </v-card-title>
            <v-data-table
              :headers="aguaHeaders"
              :items="consumosAgua"
              :loading="loadingData"
              density="compact"
              no-data-text="Nenhum registro de consumo de agua"
            >
              <template #item.data="{ item }">
                {{ formatDate(item.data) }}
              </template>
              <template #item.observacoes="{ item }">
                {{ item.observacoes || '-' }}
              </template>
            </v-data-table>
          </v-card>
        </v-col>
      </v-row>

      <!-- Charts -->
      <v-row class="mb-4">
        <v-col v-if="consumosRacao.length > 0" cols="12" md="6">
          <v-card>
            <v-card-title>Evolucao do Consumo de Racao</v-card-title>
            <v-card-text>
              <div style="height: 300px">
                <Line :data="racaoChartData" :options="chartOptions" />
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col v-if="consumosAgua.length > 0" cols="12" md="6">
          <v-card>
            <v-card-title>Evolucao do Consumo de Agua</v-card-title>
            <v-card-text>
              <div style="height: 300px">
                <Line :data="aguaChartData" :options="chartOptions" />
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <!-- Dialog Racao -->
      <v-dialog v-model="dialogRacao" max-width="600">
        <v-card>
          <v-card-title>Registrar Consumo de Racao</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formRacao.data"
                  label="Data"
                  type="date"
                  :rules="dateRules"
                  required
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model.number="formRacao.quantidade"
                  label="Quantidade (kg)"
                  type="number"
                  :rules="quantidadeRules"
                  min="0.01"
                  step="0.1"
                  required
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="formRacao.observacoes"
                  label="Observacoes"
                  rows="3"
                />
              </v-col>
            </v-row>
          </v-card-text>
          <v-card-actions>
            <v-spacer />
            <v-btn @click="dialogRacao = false">Cancelar</v-btn>
            <v-btn color="primary" variant="elevated" :loading="submitting" @click="submitRacao">
              Registrar
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>

      <!-- Dialog Agua -->
      <v-dialog v-model="dialogAgua" max-width="600">
        <v-card>
          <v-card-title>Registrar Consumo de Agua</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formAgua.data"
                  label="Data"
                  type="date"
                  :rules="dateRules"
                  required
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model.number="formAgua.quantidade"
                  label="Quantidade (L)"
                  type="number"
                  :rules="quantidadeRules"
                  min="0.01"
                  step="0.1"
                  required
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="formAgua.observacoes"
                  label="Observacoes"
                  rows="3"
                />
              </v-col>
            </v-row>
          </v-card-text>
          <v-card-actions>
            <v-spacer />
            <v-btn @click="dialogAgua = false">Cancelar</v-btn>
            <v-btn color="primary" variant="elevated" :loading="submitting" @click="submitAgua">
              Registrar
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </template>
  </PageContainer>
</template>
