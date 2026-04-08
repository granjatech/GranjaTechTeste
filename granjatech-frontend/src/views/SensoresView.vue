<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
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

interface Sensor {
  id: number
  nome: string
  tipo: string
  localizacao: string
  granjaId: number
  granjaNome: string
  status: string
}

interface Granja {
  id: number
  nome: string
}

interface Leitura {
  id: number
  sensorId: number
  valor: number
  dataHora: string
}

const sensores = ref<Sensor[]>([])
const granjas = ref<Granja[]>([])
const leituras = ref<Leitura[]>([])
const selectedSensor = ref<Sensor | null>(null)
const loading = ref(true)
const loadingLeituras = ref(false)
const error = ref('')

// Create sensor dialog
const dialogCreate = ref(false)
const submittingCreate = ref(false)
const formCreate = ref({
  nome: '',
  tipo: 'Temperatura',
  localizacao: '',
  granjaId: null as number | null,
})

// Register reading dialog
const dialogLeitura = ref(false)
const submittingLeitura = ref(false)
const formLeitura = ref({
  valor: null as number | null,
})

const tiposSensor = ['Temperatura', 'Umidade', 'CO2', 'Amonia', 'Luminosidade']

const valorRules = [
  (v: number | null) => (v !== null && v !== undefined) || 'Valor e obrigatorio',
  (v: number | null) => (v !== null && !isNaN(v)) || 'Valor deve ser um numero',
]

const nomeRules = [
  (v: string) => !!v || 'Nome e obrigatorio',
]

const sensoresHeaders = [
  { title: 'Nome', key: 'nome' },
  { title: 'Tipo', key: 'tipo' },
  { title: 'Localizacao', key: 'localizacao' },
  { title: 'Granja', key: 'granjaNome' },
  { title: 'Status', key: 'status' },
  { title: 'Acoes', key: 'actions', sortable: false },
]

const leiturasHeaders = [
  { title: 'Data/Hora', key: 'dataHora' },
  { title: 'Valor', key: 'valor' },
]

async function fetchData() {
  try {
    loading.value = true
    error.value = ''
    const [sensoresRes, granjasRes] = await Promise.all([
      api.get('/sensores'),
      api.get('/granjas'),
    ])
    sensores.value = sensoresRes.data || []
    granjas.value = granjasRes.data || []
  } catch (err) {
    console.error('Erro ao buscar sensores:', err)
    error.value = 'Erro ao carregar sensores'
  } finally {
    loading.value = false
  }
}

async function fetchLeituras(sensorId: number) {
  try {
    loadingLeituras.value = true
    const res = await api.get(`/sensores/${sensorId}/leituras`)
    leituras.value = (res.data || []).sort(
      (a: Leitura, b: Leitura) => new Date(a.dataHora).getTime() - new Date(b.dataHora).getTime()
    )
  } catch (err) {
    console.error('Erro ao buscar leituras:', err)
    error.value = 'Erro ao carregar leituras'
    leituras.value = []
  } finally {
    loadingLeituras.value = false
  }
}

function selectSensor(sensor: Sensor) {
  selectedSensor.value = sensor
  leituras.value = []
  fetchLeituras(sensor.id)
}

async function createSensor() {
  if (!formCreate.value.nome) return
  try {
    submittingCreate.value = true
    await api.post('/sensores', {
      nome: formCreate.value.nome,
      tipo: formCreate.value.tipo,
      localizacao: formCreate.value.localizacao || null,
      granjaId: formCreate.value.granjaId,
    })
    dialogCreate.value = false
    formCreate.value = { nome: '', tipo: 'Temperatura', localizacao: '', granjaId: null }
    await fetchData()
  } catch (err) {
    console.error('Erro ao criar sensor:', err)
    error.value = 'Erro ao criar sensor'
  } finally {
    submittingCreate.value = false
  }
}

async function deleteSensor(id: number) {
  try {
    await api.delete(`/sensores/${id}`)
    if (selectedSensor.value?.id === id) {
      selectedSensor.value = null
      leituras.value = []
    }
    await fetchData()
  } catch (err) {
    console.error('Erro ao excluir sensor:', err)
    error.value = 'Erro ao excluir sensor'
  }
}

async function submitLeitura() {
  if (!selectedSensor.value || formLeitura.value.valor === null) return
  try {
    submittingLeitura.value = true
    await api.post('/leituras', {
      sensorId: selectedSensor.value.id,
      valor: Number(formLeitura.value.valor),
    })
    dialogLeitura.value = false
    formLeitura.value = { valor: null }
    await fetchLeituras(selectedSensor.value.id)
  } catch (err) {
    console.error('Erro ao registrar leitura:', err)
    error.value = 'Erro ao registrar leitura'
  } finally {
    submittingLeitura.value = false
  }
}

function formatDateTime(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleString('pt-BR')
  } catch {
    return dateStr
  }
}

const chartData = computed(() => {
  return {
    labels: leituras.value.map((l) => formatDateTime(l.dataHora)),
    datasets: [
      {
        label: selectedSensor.value ? `${selectedSensor.value.tipo} - Leituras` : 'Leituras',
        data: leituras.value.map((l) => l.valor),
        borderColor: '#9c27b0',
        backgroundColor: 'rgba(156, 39, 176, 0.15)',
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

const confirmDelete = ref(false)
const deleteTargetId = ref<number | null>(null)

function askDelete(id: number) {
  deleteTargetId.value = id
  confirmDelete.value = true
}

function doDelete() {
  if (deleteTargetId.value !== null) {
    deleteSensor(deleteTargetId.value)
  }
  confirmDelete.value = false
  deleteTargetId.value = null
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer title="Sensores" subtitle="Monitoramento de sensores e leituras">
    <template #action>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="dialogCreate = true">
        Novo Sensor
      </v-btn>
    </template>

    <v-alert v-if="error" type="error" closable class="mb-4" @click:close="error = ''">
      {{ error }}
    </v-alert>

    <v-progress-linear v-if="loading" indeterminate color="primary" class="mb-4" />

    <v-row>
      <!-- Sensors table -->
      <v-col cols="12" :md="selectedSensor ? 6 : 12">
        <v-card>
          <v-card-title class="d-flex align-center justify-space-between">
            <span>Lista de Sensores</span>
            <v-btn size="small" variant="outlined" prepend-icon="mdi-refresh" @click="fetchData">
              Atualizar
            </v-btn>
          </v-card-title>
          <v-data-table
            :headers="sensoresHeaders"
            :items="sensores"
            :loading="loading"
            density="compact"
            no-data-text="Nenhum sensor cadastrado"
            @click:row="(_event: any, row: any) => selectSensor(row.item)"
            class="cursor-pointer"
          >
            <template #item.status="{ item }">
              <v-chip
                :color="item.status === 'Ativo' ? 'success' : 'default'"
                size="small"
              >
                {{ item.status || 'N/A' }}
              </v-chip>
            </template>
            <template #item.actions="{ item }">
              <v-btn
                icon="mdi-delete"
                size="small"
                variant="text"
                color="error"
                @click.stop="askDelete(item.id)"
              />
            </template>
          </v-data-table>
        </v-card>
      </v-col>

      <!-- Readings panel -->
      <v-col v-if="selectedSensor" cols="12" md="6">
        <v-card class="mb-4">
          <v-card-title class="d-flex align-center justify-space-between">
            <div>
              <div>Leituras: {{ selectedSensor.nome }}</div>
              <div class="text-subtitle-2 text-medium-emphasis">
                {{ selectedSensor.tipo }} - {{ selectedSensor.localizacao || 'Sem localizacao' }}
              </div>
            </div>
            <v-btn size="small" color="primary" prepend-icon="mdi-plus" @click="dialogLeitura = true">
              Nova Leitura
            </v-btn>
          </v-card-title>

          <v-progress-linear v-if="loadingLeituras" indeterminate color="purple" />

          <v-data-table
            :headers="leiturasHeaders"
            :items="leituras"
            :loading="loadingLeituras"
            density="compact"
            no-data-text="Nenhuma leitura encontrada"
          >
            <template #item.dataHora="{ item }">
              {{ formatDateTime(item.dataHora) }}
            </template>
          </v-data-table>
        </v-card>

        <!-- Readings chart -->
        <v-card v-if="leituras.length > 0">
          <v-card-title>Grafico de Leituras</v-card-title>
          <v-card-text>
            <div style="height: 300px">
              <Line :data="chartData" :options="chartOptions" />
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <!-- Create sensor dialog -->
    <v-dialog v-model="dialogCreate" max-width="500">
      <v-card>
        <v-card-title>Novo Sensor</v-card-title>
        <v-card-text>
          <v-row>
            <v-col cols="12">
              <v-text-field
                v-model="formCreate.nome"
                label="Nome"
                :rules="nomeRules"
                required
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-select
                v-model="formCreate.tipo"
                :items="tiposSensor"
                label="Tipo"
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-select
                v-model="formCreate.granjaId"
                :items="granjas"
                item-title="nome"
                item-value="id"
                label="Granja"
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="formCreate.localizacao"
                label="Localizacao"
              />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="dialogCreate = false">Cancelar</v-btn>
          <v-btn color="primary" variant="elevated" :loading="submittingCreate" @click="createSensor">
            Criar
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Register reading dialog -->
    <v-dialog v-model="dialogLeitura" max-width="400">
      <v-card>
        <v-card-title>Registrar Leitura</v-card-title>
        <v-card-text>
          <v-text-field
            v-model.number="formLeitura.valor"
            label="Valor"
            type="number"
            :rules="valorRules"
            required
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="dialogLeitura = false">Cancelar</v-btn>
          <v-btn color="primary" variant="elevated" :loading="submittingLeitura" @click="submitLeitura">
            Registrar
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete confirmation -->
    <v-dialog v-model="confirmDelete" max-width="400">
      <v-card>
        <v-card-title>Confirmar Exclusao</v-card-title>
        <v-card-text>Tem certeza que deseja excluir este sensor?</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="confirmDelete = false">Cancelar</v-btn>
          <v-btn color="error" variant="elevated" @click="doDelete">Excluir</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </PageContainer>
</template>
