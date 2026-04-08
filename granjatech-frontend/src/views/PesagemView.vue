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

interface Pesagem {
  id: number
  loteId: number
  data: string
  pesoMedio: number
  quantidadeAmostras: number
  observacoes?: string
}

interface ResumoPesagem {
  totalPesagens: number
  pesoMedioAtual: number
  ganhoMedioDiario: number
}

const lotes = ref<Lote[]>([])
const selectedLoteId = ref<number | null>(null)
const pesagens = ref<Pesagem[]>([])
const resumo = ref<ResumoPesagem | null>(null)
const loading = ref(true)
const loadingData = ref(false)
const error = ref('')

const dialog = ref(false)
const submitting = ref(false)

const formData = ref({
  data: new Date().toISOString().split('T')[0],
  pesoMedio: null as number | null,
  quantidadeAmostras: null as number | null,
  observacoes: '',
})

const pesoRules = [
  (v: number | null) => (v !== null && v !== undefined) || 'Peso medio e obrigatorio',
  (v: number | null) => (v !== null && v > 0) || 'Peso deve ser maior que 0',
]

const amostrasRules = [
  (v: number | null) => (v !== null && v !== undefined) || 'Quantidade de amostras e obrigatoria',
  (v: number | null) => (v !== null && v >= 1) || 'Minimo 1 amostra',
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

async function fetchPesagemData(loteId: number) {
  if (!loteId) return
  try {
    loadingData.value = true
    error.value = ''
    const [pesagensRes, resumoRes] = await Promise.all([
      api.get(`/pesagem/${loteId}`),
      api.get(`/pesagem/resumo/${loteId}`),
    ])
    pesagens.value = pesagensRes.data || []
    resumo.value = resumoRes.data
  } catch (err) {
    console.error('Erro ao buscar pesagens:', err)
    error.value = 'Erro ao carregar pesagens'
  } finally {
    loadingData.value = false
  }
}

watch(selectedLoteId, (newId) => {
  if (newId) fetchPesagemData(newId)
})

onMounted(() => {
  fetchLotes()
})

async function submitPesagem() {
  if (!selectedLoteId.value || !formData.value.pesoMedio || !formData.value.quantidadeAmostras) return
  try {
    submitting.value = true
    await api.post('/pesagem', {
      loteId: selectedLoteId.value,
      data: formData.value.data,
      pesoMedio: Number(formData.value.pesoMedio),
      quantidadeAmostras: Number(formData.value.quantidadeAmostras),
      observacoes: formData.value.observacoes || null,
    })
    dialog.value = false
    formData.value = {
      data: new Date().toISOString().split('T')[0],
      pesoMedio: null,
      quantidadeAmostras: null,
      observacoes: '',
    }
    await fetchPesagemData(selectedLoteId.value)
  } catch (err) {
    console.error('Erro ao registrar pesagem:', err)
    error.value = 'Erro ao registrar pesagem'
  } finally {
    submitting.value = false
  }
}

const tableHeaders = [
  { title: 'Data', key: 'data' },
  { title: 'Peso Medio (g)', key: 'pesoMedio' },
  { title: 'Qtd Amostras', key: 'quantidadeAmostras' },
  { title: 'Observacoes', key: 'observacoes' },
]

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString('pt-BR')
  } catch {
    return dateStr
  }
}

const chartData = computed(() => {
  const sorted = [...pesagens.value].sort(
    (a, b) => new Date(a.data).getTime() - new Date(b.data).getTime()
  )
  return {
    labels: sorted.map((i) => formatDate(i.data)),
    datasets: [
      {
        label: 'Peso Medio (g)',
        data: sorted.map((i) => i.pesoMedio),
        borderColor: '#ff9800',
        backgroundColor: 'rgba(255, 152, 0, 0.15)',
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
  <PageContainer title="Pesagens" subtitle="Registro de pesagens semanais por lote">
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
      Nenhum lote ativo encontrado. Crie um lote para registrar pesagens.
    </v-alert>

    <template v-if="selectedLoteId && !loading">
      <!-- Summary Cards -->
      <v-row v-if="resumo" class="mb-4">
        <v-col cols="12" md="4">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Total Pesagens</div>
              <div class="text-h5 font-weight-bold text-primary">
                {{ resumo.totalPesagens ?? 0 }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="4">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Peso Medio Atual</div>
              <div class="text-h5 font-weight-bold text-success">
                {{ resumo.pesoMedioAtual?.toFixed(0) ?? '0' }} g
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="4">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Ganho Medio Diario</div>
              <div class="text-h5 font-weight-bold text-warning">
                {{ resumo.ganhoMedioDiario?.toFixed(1) ?? '0' }} g/dia
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <!-- Data Table -->
      <v-card class="mb-4">
        <v-card-title class="d-flex align-center justify-space-between">
          <span>Historico de Pesagens</span>
          <v-btn color="primary" size="small" prepend-icon="mdi-plus" @click="dialog = true">
            Nova Pesagem
          </v-btn>
        </v-card-title>
        <v-data-table
          :headers="tableHeaders"
          :items="pesagens"
          :loading="loadingData"
          density="compact"
          no-data-text="Nenhuma pesagem registrada para este lote"
        >
          <template #item.data="{ item }">
            {{ formatDate(item.data) }}
          </template>
          <template #item.observacoes="{ item }">
            {{ item.observacoes || '-' }}
          </template>
        </v-data-table>
      </v-card>

      <!-- Chart -->
      <v-card v-if="pesagens.length > 0" class="mb-4">
        <v-card-title>Curva de Peso Medio</v-card-title>
        <v-card-text>
          <div style="height: 300px">
            <Line :data="chartData" :options="chartOptions" />
          </div>
        </v-card-text>
      </v-card>

      <!-- Dialog -->
      <v-dialog v-model="dialog" max-width="600">
        <v-card>
          <v-card-title>Registrar Nova Pesagem</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formData.data"
                  label="Data"
                  type="date"
                  required
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model.number="formData.pesoMedio"
                  label="Peso Medio (g)"
                  type="number"
                  :rules="pesoRules"
                  min="1"
                  required
                />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model.number="formData.quantidadeAmostras"
                  label="Quantidade de Amostras"
                  type="number"
                  :rules="amostrasRules"
                  min="1"
                  required
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="formData.observacoes"
                  label="Observacoes"
                  rows="3"
                />
              </v-col>
            </v-row>
          </v-card-text>
          <v-card-actions>
            <v-spacer />
            <v-btn @click="dialog = false">Cancelar</v-btn>
            <v-btn color="primary" variant="elevated" :loading="submitting" @click="submitPesagem">
              Registrar
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </template>
  </PageContainer>
</template>
