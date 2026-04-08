<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'

interface Lote {
  id: number
  codigo: string
  granjaNome: string
  status: string
}

interface EventoSanitario {
  id: number
  loteId: number
  data: string
  tipo: string
  descricao: string
  responsavel?: string
  observacoes?: string
}

interface ResumoSanitario {
  totalEventos: number
  ultimoEvento?: string
  vacinasAplicadas: number
  medicamentosAdministrados: number
}

interface CronogramaVacina {
  vacina: string
  idadeRecomendada: string
  descricao: string
}

const lotes = ref<Lote[]>([])
const selectedLoteId = ref<number | null>(null)
const eventos = ref<EventoSanitario[]>([])
const resumo = ref<ResumoSanitario | null>(null)
const cronograma = ref<CronogramaVacina[]>([])
const loading = ref(true)
const loadingData = ref(false)
const error = ref('')

const dialog = ref(false)
const submitting = ref(false)

const tiposEvento = ['Vacinacao', 'Medicacao', 'Exame', 'Tratamento', 'Outro']

const formData = ref({
  data: new Date().toISOString().split('T')[0],
  tipo: 'Vacinacao',
  descricao: '',
  responsavel: '',
  observacoes: '',
})

const descricaoRules = [
  (v: string) => !!v || 'Descricao e obrigatoria',
]

const eventosHeaders = [
  { title: 'Data', key: 'data' },
  { title: 'Tipo', key: 'tipo' },
  { title: 'Descricao', key: 'descricao' },
  { title: 'Responsavel', key: 'responsavel' },
  { title: 'Observacoes', key: 'observacoes' },
]

const cronogramaHeaders = [
  { title: 'Vacina', key: 'vacina' },
  { title: 'Idade Recomendada', key: 'idadeRecomendada' },
  { title: 'Descricao', key: 'descricao' },
]

function getTipoColor(tipo: string): string {
  switch (tipo) {
    case 'Vacinacao': return 'success'
    case 'Medicacao': return 'warning'
    case 'Exame': return 'info'
    case 'Tratamento': return 'primary'
    default: return 'default'
  }
}

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString('pt-BR')
  } catch {
    return dateStr
  }
}

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

async function fetchSanitarioData(loteId: number) {
  if (!loteId) return
  try {
    loadingData.value = true
    error.value = ''
    const [eventosRes, resumoRes] = await Promise.all([
      api.get(`/sanitario/${loteId}`),
      api.get(`/sanitario/resumo/${loteId}`),
    ])
    eventos.value = eventosRes.data || []
    resumo.value = resumoRes.data
  } catch (err) {
    console.error('Erro ao buscar eventos sanitarios:', err)
    error.value = 'Erro ao carregar eventos sanitarios'
  } finally {
    loadingData.value = false
  }
}

async function fetchCronograma() {
  try {
    const res = await api.get('/sanitario/cronograma-vacinacao')
    cronograma.value = res.data || []
  } catch (err) {
    console.error('Erro ao buscar cronograma de vacinacao:', err)
  }
}

watch(selectedLoteId, (newId) => {
  if (newId) fetchSanitarioData(newId)
})

onMounted(() => {
  fetchLotes()
  fetchCronograma()
})

async function submitEvento() {
  if (!selectedLoteId.value || !formData.value.descricao) return
  try {
    submitting.value = true
    await api.post('/sanitario', {
      loteId: selectedLoteId.value,
      data: formData.value.data,
      tipo: formData.value.tipo,
      descricao: formData.value.descricao,
      responsavel: formData.value.responsavel || null,
      observacoes: formData.value.observacoes || null,
    })
    dialog.value = false
    formData.value = {
      data: new Date().toISOString().split('T')[0],
      tipo: 'Vacinacao',
      descricao: '',
      responsavel: '',
      observacoes: '',
    }
    await fetchSanitarioData(selectedLoteId.value)
  } catch (err) {
    console.error('Erro ao registrar evento sanitario:', err)
    error.value = 'Erro ao registrar evento sanitario'
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <PageContainer title="Sanitario" subtitle="Eventos sanitarios e cronograma de vacinacao">
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
        <v-col cols="auto">
          <v-btn color="primary" prepend-icon="mdi-plus" @click="dialog = true">
            Novo Evento
          </v-btn>
        </v-col>
      </v-row>
    </template>

    <v-alert v-if="error" type="error" closable class="mb-4" @click:close="error = ''">
      {{ error }}
    </v-alert>

    <v-progress-linear v-if="loading || loadingData" indeterminate color="primary" class="mb-4" />

    <v-alert v-if="!loading && lotes.length === 0" type="info" class="mb-4">
      Nenhum lote ativo encontrado. Crie um lote para registrar eventos sanitarios.
    </v-alert>

    <template v-if="selectedLoteId && !loading">
      <!-- Summary Cards -->
      <v-row v-if="resumo" class="mb-4">
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Total Eventos</div>
              <div class="text-h5 font-weight-bold text-primary">
                {{ resumo.totalEventos ?? 0 }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Ultimo Evento</div>
              <div class="text-h6 font-weight-bold">
                {{ resumo.ultimoEvento ? formatDate(resumo.ultimoEvento) : 'N/A' }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Vacinas Aplicadas</div>
              <div class="text-h5 font-weight-bold text-success">
                {{ resumo.vacinasAplicadas ?? 0 }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" md="3">
          <v-card>
            <v-card-text class="text-center">
              <div class="text-subtitle-2 text-medium-emphasis">Medicamentos</div>
              <div class="text-h5 font-weight-bold text-warning">
                {{ resumo.medicamentosAdministrados ?? 0 }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <!-- Events Table -->
      <v-card class="mb-4">
        <v-card-title class="d-flex align-center justify-space-between">
          <span>Eventos Sanitarios</span>
          <v-btn size="small" color="primary" prepend-icon="mdi-plus" @click="dialog = true">
            Novo Evento
          </v-btn>
        </v-card-title>
        <v-data-table
          :headers="eventosHeaders"
          :items="eventos"
          :loading="loadingData"
          density="compact"
          no-data-text="Nenhum evento sanitario registrado"
        >
          <template #item.data="{ item }">
            {{ formatDate(item.data) }}
          </template>
          <template #item.tipo="{ item }">
            <v-chip :color="getTipoColor(item.tipo)" size="small">
              {{ item.tipo }}
            </v-chip>
          </template>
          <template #item.responsavel="{ item }">
            {{ item.responsavel || '-' }}
          </template>
          <template #item.observacoes="{ item }">
            {{ item.observacoes || '-' }}
          </template>
        </v-data-table>
      </v-card>
    </template>

    <!-- Vaccination Schedule (always visible, not lote-dependent) -->
    <v-card v-if="cronograma.length > 0">
      <v-card-title>Cronograma de Vacinacao</v-card-title>
      <v-data-table
        :headers="cronogramaHeaders"
        :items="cronograma"
        density="compact"
        no-data-text="Nenhum cronograma disponivel"
      />
    </v-card>

    <!-- Register event dialog -->
    <v-dialog v-model="dialog" max-width="600">
      <v-card>
        <v-card-title>Registrar Evento Sanitario</v-card-title>
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
              <v-select
                v-model="formData.tipo"
                :items="tiposEvento"
                label="Tipo"
                required
              />
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="formData.descricao"
                label="Descricao"
                :rules="descricaoRules"
                required
              />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="formData.responsavel"
                label="Responsavel"
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
          <v-btn color="primary" variant="elevated" :loading="submitting" @click="submitEvento">
            Registrar
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </PageContainer>
</template>
