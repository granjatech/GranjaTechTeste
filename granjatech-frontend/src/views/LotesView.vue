<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

interface Granja {
  id: number
  codigo: string
  nome: string
}

interface Lote {
  id: number
  codigo: string
  granjaId: number
  granjaNome: string
  dataAlojamento: string
  quantidadeInicial: number
  quantidadeAtual: number
  linhagem: string
  status: string
  racaoTipo: string
  viabilidade: number | null
  iep: number | null
}

// LoteForm shape used inline in handleSubmit payload

interface Mortalidade {
  id: number
  data: string
  quantidade: number
  causa: string
}

const items = ref<Lote[]>([])
const granjas = ref<Granja[]>([])
const loading = ref(true)
const search = ref('')
const dialogOpen = ref(false)
const deleteDialogOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const deletingId = ref<number | null>(null)
const deletingCode = ref('')
const saving = ref(false)
const formRef = ref()

// Form fields
const formGranjaId = ref<number | null>(null)
const formDataAlojamento = ref('')
const formQuantidadeInicial = ref<number | null>(null)
const formLinhagem = ref('')
const formRacaoTipo = ref('')
const formStatus = ref('Ativo')

// Mortalidade
const mortalidadeDialogOpen = ref(false)
const selectedLote = ref<Lote | null>(null)
const mortalidades = ref<Mortalidade[]>([])
const mortalidadeLoading = ref(false)
const mortalidadeSaving = ref(false)
const mortalidadeFormRef = ref()
const mortData = ref('')
const mortQuantidade = ref<number | null>(null)
const mortCausa = ref('')

// Snackbar
const snackbar = ref({ show: false, text: '', color: 'success' })

const headers = [
  { title: 'Codigo', key: 'codigo', sortable: true },
  { title: 'Granja', key: 'granjaNome', sortable: true },
  { title: 'Data Alojamento', key: 'dataAlojamento', sortable: true },
  { title: 'Qtd Inicial', key: 'quantidadeInicial' },
  { title: 'Qtd Atual', key: 'quantidadeAtual' },
  { title: 'Linhagem', key: 'linhagem' },
  { title: 'Status', key: 'status' },
  { title: 'Viabilidade', key: 'viabilidade' },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

const mortalidadeHeaders = [
  { title: 'Data', key: 'data' },
  { title: 'Quantidade', key: 'quantidade' },
  { title: 'Causa', key: 'causa' },
]

const statusOptions = ['Ativo', 'Encerrado']

const requiredRule = [(v: any) => !!v || 'Campo obrigatorio']
const qtdRules = [
  (v: any) => !!v || 'Quantidade obrigatoria',
  (v: number) => v >= 1 || 'Quantidade deve ser no minimo 1',
]

function showSnackbar(text: string, color: string = 'success') {
  snackbar.value = { show: true, text, color }
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('pt-BR')
}

function formatViabilidade(value: number | null): string {
  if (value === null || value === undefined) return '-'
  return `${value.toFixed(1)}%`
}

function openCreate() {
  isEditMode.value = false
  editingId.value = null
  formGranjaId.value = null
  formDataAlojamento.value = new Date().toISOString().split('T')[0]
  formQuantidadeInicial.value = null
  formLinhagem.value = ''
  formRacaoTipo.value = ''
  formStatus.value = 'Ativo'
  dialogOpen.value = true
}

function openEdit(lote: Lote) {
  isEditMode.value = true
  editingId.value = lote.id
  formGranjaId.value = lote.granjaId
  formDataAlojamento.value = lote.dataAlojamento ? lote.dataAlojamento.split('T')[0] : ''
  formQuantidadeInicial.value = lote.quantidadeInicial
  formLinhagem.value = lote.linhagem || ''
  formRacaoTipo.value = lote.racaoTipo || ''
  formStatus.value = lote.status
  dialogOpen.value = true
}

function openDelete(lote: Lote) {
  deletingId.value = lote.id
  deletingCode.value = lote.codigo
  deleteDialogOpen.value = true
}

function closeDialog() {
  dialogOpen.value = false
  formRef.value?.resetValidation()
}

async function openMortalidade(lote: Lote) {
  selectedLote.value = lote
  mortalidadeDialogOpen.value = true
  mortData.value = new Date().toISOString().split('T')[0]
  mortQuantidade.value = null
  mortCausa.value = ''
  await fetchMortalidades(lote.id)
}

async function fetchMortalidades(loteId: number) {
  mortalidadeLoading.value = true
  try {
    const response = await api.get(`/lotes/${loteId}/mortalidades`)
    mortalidades.value = response.data
  } catch (err) {
    console.error('Erro ao buscar mortalidades:', err)
    showSnackbar('Erro ao carregar mortalidades.', 'error')
  } finally {
    mortalidadeLoading.value = false
  }
}

async function submitMortalidade() {
  const { valid } = await mortalidadeFormRef.value.validate()
  if (!valid || !selectedLote.value) return

  mortalidadeSaving.value = true
  try {
    await api.post(`/lotes/${selectedLote.value.id}/mortalidades`, {
      data: mortData.value,
      quantidade: mortQuantidade.value,
      causa: mortCausa.value,
    })
    showSnackbar('Mortalidade registrada com sucesso!')
    await fetchMortalidades(selectedLote.value.id)
    mortData.value = new Date().toISOString().split('T')[0]
    mortQuantidade.value = null
    mortCausa.value = ''
    mortalidadeFormRef.value?.resetValidation()
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao registrar mortalidade.',
      'error'
    )
  } finally {
    mortalidadeSaving.value = false
  }
}

async function handleSubmit() {
  const { valid } = await formRef.value.validate()
  if (!valid) return

  saving.value = true
  try {
    const payload: any = {
      granjaId: formGranjaId.value,
      dataAlojamento: formDataAlojamento.value,
      quantidadeInicial: formQuantidadeInicial.value,
      linhagem: formLinhagem.value,
      racaoTipo: formRacaoTipo.value,
    }

    if (isEditMode.value && editingId.value) {
      payload.status = formStatus.value
      await api.put(`/lotes/${editingId.value}`, payload)
      showSnackbar('Lote atualizado com sucesso!')
    } else {
      await api.post('/lotes', payload)
      showSnackbar('Lote criado com sucesso!')
    }
    dialogOpen.value = false
    await fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao salvar lote.',
      'error'
    )
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/lotes/${deletingId.value}`)
    showSnackbar('Lote excluido com sucesso!')
    deleteDialogOpen.value = false
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao excluir lote.',
      'error'
    )
    deleteDialogOpen.value = false
  }
}

async function fetchData() {
  try {
    loading.value = true
    const [lotesRes, granjasRes] = await Promise.all([
      api.get('/lotes'),
      api.get('/granjas'),
    ])
    items.value = lotesRes.data
    granjas.value = granjasRes.data
  } catch (err) {
    console.error('Erro ao buscar lotes:', err)
    showSnackbar('Erro ao carregar lista de lotes.', 'error')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer title="Lotes" subtitle="Gerenciamento de lotes de aves">
    <template #action>
      <v-btn
        color="primary"
        prepend-icon="mdi-plus"
        @click="openCreate"
      >
        Novo Lote
      </v-btn>
    </template>

    <LoadingSpinner v-if="loading" message="Carregando lotes..." />

    <v-card v-else>
      <v-card-text class="pb-0">
        <v-text-field
          v-model="search"
          prepend-inner-icon="mdi-magnify"
          label="Buscar lotes..."
          single-line
          hide-details
          density="compact"
          class="mb-4"
          clearable
        />
      </v-card-text>

      <v-data-table
        :headers="headers"
        :items="items"
        :search="search"
        no-data-text="Nenhum lote encontrado"
        items-per-page="25"
        class="elevation-0"
      >
        <template #item.dataAlojamento="{ item }">
          {{ formatDate(item.dataAlojamento) }}
        </template>

        <template #item.status="{ item }">
          <v-chip
            :color="item.status === 'Ativo' ? 'success' : 'default'"
            variant="outlined"
            size="small"
          >
            {{ item.status }}
          </v-chip>
        </template>

        <template #item.viabilidade="{ item }">
          {{ formatViabilidade(item.viabilidade) }}
        </template>

        <template #item.actions="{ item }">
          <v-btn
            icon="mdi-skull-crossbones"
            size="small"
            variant="text"
            class="mr-1"
            title="Mortalidade"
            @click="openMortalidade(item)"
          />
          <v-btn
            icon="mdi-pencil"
            size="small"
            variant="text"
            class="mr-1"
            @click="openEdit(item)"
          />
          <v-btn
            icon="mdi-delete"
            size="small"
            variant="text"
            color="error"
            @click="openDelete(item)"
          />
        </template>
      </v-data-table>
    </v-card>

    <!-- Create/Edit Dialog -->
    <v-dialog v-model="dialogOpen" max-width="700" persistent>
      <v-card>
        <v-card-title class="text-h6">
          {{ isEditMode ? 'Editar Lote' : 'Novo Lote' }}
        </v-card-title>
        <v-card-text>
          <v-form ref="formRef" @submit.prevent="handleSubmit">
            <v-select
              v-model="formGranjaId"
              label="Granja"
              :items="granjas"
              item-title="nome"
              item-value="id"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model="formDataAlojamento"
              label="Data de Alojamento"
              type="date"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formQuantidadeInicial"
              label="Quantidade Inicial"
              type="number"
              :rules="qtdRules"
              class="mb-2"
            />
            <v-text-field
              v-model="formLinhagem"
              label="Linhagem"
              class="mb-2"
            />
            <v-text-field
              v-model="formRacaoTipo"
              label="Tipo de Racao"
              class="mb-2"
            />
            <v-select
              v-if="isEditMode"
              v-model="formStatus"
              label="Status"
              :items="statusOptions"
            />
          </v-form>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer />
          <v-btn variant="text" @click="closeDialog">Cancelar</v-btn>
          <v-btn
            color="primary"
            variant="flat"
            :loading="saving"
            @click="handleSubmit"
          >
            {{ isEditMode ? 'Atualizar' : 'Criar' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialogOpen" max-width="400">
      <v-card>
        <v-card-title class="text-h6">Confirmar Exclusao</v-card-title>
        <v-card-text>
          Tem certeza que deseja excluir o lote
          <strong>{{ deletingCode }}</strong>?
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer />
          <v-btn variant="text" @click="deleteDialogOpen = false">
            Cancelar
          </v-btn>
          <v-btn color="error" variant="flat" @click="handleDelete">
            Excluir
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Mortalidade Dialog -->
    <v-dialog v-model="mortalidadeDialogOpen" max-width="700">
      <v-card>
        <v-card-title class="text-h6">
          Mortalidade - {{ selectedLote?.codigo }}
        </v-card-title>
        <v-card-text>
          <!-- Existing mortalidades table -->
          <v-data-table
            :headers="mortalidadeHeaders"
            :items="mortalidades"
            :loading="mortalidadeLoading"
            no-data-text="Nenhuma mortalidade registrada"
            density="compact"
            class="mb-4"
          >
            <template #item.data="{ item }">
              {{ formatDate(item.data) }}
            </template>
          </v-data-table>

          <!-- Register new mortalidade -->
          <v-divider class="mb-4" />
          <h3 class="text-subtitle-1 font-weight-bold mb-3">Registrar Nova Mortalidade</h3>
          <v-form ref="mortalidadeFormRef" @submit.prevent="submitMortalidade">
            <v-row>
              <v-col cols="12" sm="4">
                <v-text-field
                  v-model="mortData"
                  label="Data"
                  type="date"
                  :rules="requiredRule"
                  density="compact"
                />
              </v-col>
              <v-col cols="12" sm="4">
                <v-text-field
                  v-model.number="mortQuantidade"
                  label="Quantidade"
                  type="number"
                  :rules="qtdRules"
                  density="compact"
                />
              </v-col>
              <v-col cols="12" sm="4">
                <v-text-field
                  v-model="mortCausa"
                  label="Causa"
                  density="compact"
                />
              </v-col>
            </v-row>
          </v-form>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer />
          <v-btn variant="text" @click="mortalidadeDialogOpen = false">
            Fechar
          </v-btn>
          <v-btn
            color="primary"
            variant="flat"
            :loading="mortalidadeSaving"
            @click="submitMortalidade"
          >
            Registrar Mortalidade
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Snackbar -->
    <v-snackbar
      v-model="snackbar.show"
      :color="snackbar.color"
      :timeout="3000"
      location="bottom end"
    >
      {{ snackbar.text }}
    </v-snackbar>
  </PageContainer>
</template>
