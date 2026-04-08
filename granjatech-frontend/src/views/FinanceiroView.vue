<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

interface Granja {
  id: number
  nome: string
}

interface Transacao {
  id: number
  tipo: string
  categoria: string
  descricao: string
  valor: number
  data: string
  granjaId: number | null
  granjaNome: string | null
  usuarioNome: string
  dataCriacao: string
}

const items = ref<Transacao[]>([])
const granjas = ref<Granja[]>([])
const loading = ref(true)
const search = ref('')
const dialogOpen = ref(false)
const deleteDialogOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const deletingId = ref<number | null>(null)
const deletingDesc = ref('')
const saving = ref(false)
const formRef = ref()

// Form fields
const formTipo = ref('Saida')
const formCategoria = ref('')
const formDescricao = ref('')
const formValor = ref<number | null>(null)
const formData = ref('')
const formGranjaId = ref<number | null>(null)

// Snackbar
const snackbar = ref({ show: false, text: '', color: 'success' })

// Summary computeds
const totalEntradas = computed(() =>
  items.value
    .filter((t) => t.tipo === 'Entrada')
    .reduce((sum, t) => sum + t.valor, 0)
)

const totalSaidas = computed(() =>
  items.value
    .filter((t) => t.tipo === 'Saida')
    .reduce((sum, t) => sum + t.valor, 0)
)

const saldo = computed(() => totalEntradas.value - totalSaidas.value)

const headers = [
  { title: 'Tipo', key: 'tipo' },
  { title: 'Categoria', key: 'categoria', sortable: true },
  { title: 'Descricao', key: 'descricao', sortable: true },
  { title: 'Valor', key: 'valor', sortable: true },
  { title: 'Data', key: 'data', sortable: true },
  { title: 'Granja', key: 'granjaNome' },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

const tipoOptions = ['Entrada', 'Saida']

const requiredRule = [(v: any) => !!v || 'Campo obrigatorio']
const valorRules = [
  (v: any) => v !== null && v !== '' && v !== undefined || 'Valor obrigatorio',
  (v: number) => v >= 0.01 || 'Valor deve ser no minimo R$ 0,01',
]

function showSnackbar(text: string, color: string = 'success') {
  snackbar.value = { show: true, text, color }
}

function formatCurrency(value: number): string {
  return `R$ ${value.toLocaleString('pt-BR', { minimumFractionDigits: 2 })}`
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('pt-BR')
}

function openCreate() {
  isEditMode.value = false
  editingId.value = null
  formTipo.value = 'Saida'
  formCategoria.value = ''
  formDescricao.value = ''
  formValor.value = null
  formData.value = new Date().toISOString().split('T')[0]
  formGranjaId.value = null
  dialogOpen.value = true
}

function openEdit(transacao: Transacao) {
  isEditMode.value = true
  editingId.value = transacao.id
  formTipo.value = transacao.tipo
  formCategoria.value = transacao.categoria
  formDescricao.value = transacao.descricao
  formValor.value = transacao.valor
  formData.value = transacao.data ? transacao.data.split('T')[0] : ''
  formGranjaId.value = transacao.granjaId
  dialogOpen.value = true
}

function openDelete(transacao: Transacao) {
  deletingId.value = transacao.id
  deletingDesc.value = transacao.descricao
  deleteDialogOpen.value = true
}

function closeDialog() {
  dialogOpen.value = false
  formRef.value?.resetValidation()
}

async function handleSubmit() {
  const { valid } = await formRef.value.validate()
  if (!valid) return

  saving.value = true
  try {
    const payload = {
      tipo: formTipo.value,
      categoria: formCategoria.value,
      descricao: formDescricao.value,
      valor: formValor.value,
      data: formData.value,
      granjaId: formGranjaId.value,
    }

    if (isEditMode.value && editingId.value) {
      await api.put(`/financas/${editingId.value}`, payload)
      showSnackbar('Transacao atualizada com sucesso!')
    } else {
      await api.post('/financas', payload)
      showSnackbar('Transacao criada com sucesso!')
    }
    dialogOpen.value = false
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao salvar transacao.',
      'error'
    )
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/financas/${deletingId.value}`)
    showSnackbar('Transacao excluida com sucesso!')
    deleteDialogOpen.value = false
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao excluir transacao.',
      'error'
    )
    deleteDialogOpen.value = false
  }
}

async function fetchData() {
  try {
    loading.value = true
    const [transacoesRes, granjasRes] = await Promise.all([
      api.get('/financas'),
      api.get('/granjas'),
    ])
    items.value = transacoesRes.data
    granjas.value = granjasRes.data
  } catch (err) {
    console.error('Erro ao buscar financas:', err)
    showSnackbar('Erro ao carregar dados financeiros.', 'error')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer title="Financeiro" subtitle="Gestao de transacoes financeiras">
    <template #action>
      <v-btn
        color="primary"
        prepend-icon="mdi-plus"
        @click="openCreate"
      >
        Nova Transacao
      </v-btn>
    </template>

    <LoadingSpinner v-if="loading" message="Carregando dados financeiros..." />

    <template v-else>
      <!-- Summary Cards -->
      <v-row class="mb-4">
        <v-col cols="12" sm="4">
          <v-card color="success" variant="tonal">
            <v-card-text class="text-center">
              <div class="text-body-2 mb-1">Total Entradas</div>
              <div class="text-h5 font-weight-bold">
                {{ formatCurrency(totalEntradas) }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" sm="4">
          <v-card color="error" variant="tonal">
            <v-card-text class="text-center">
              <div class="text-body-2 mb-1">Total Saidas</div>
              <div class="text-h5 font-weight-bold">
                {{ formatCurrency(totalSaidas) }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
        <v-col cols="12" sm="4">
          <v-card color="info" variant="tonal">
            <v-card-text class="text-center">
              <div class="text-body-2 mb-1">Saldo</div>
              <div class="text-h5 font-weight-bold">
                {{ formatCurrency(saldo) }}
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <v-card>
        <v-card-text class="pb-0">
          <v-text-field
            v-model="search"
            prepend-inner-icon="mdi-magnify"
            label="Buscar transacoes..."
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
          no-data-text="Nenhuma transacao encontrada"
          items-per-page="25"
          class="elevation-0"
        >
          <template #item.tipo="{ item }">
            <v-chip
              :color="item.tipo === 'Entrada' ? 'success' : 'error'"
              variant="outlined"
              size="small"
            >
              {{ item.tipo }}
            </v-chip>
          </template>

          <template #item.valor="{ item }">
            <span :class="item.tipo === 'Entrada' ? 'text-success' : 'text-error'">
              {{ formatCurrency(item.valor) }}
            </span>
          </template>

          <template #item.data="{ item }">
            {{ formatDate(item.data) }}
          </template>

          <template #item.granjaNome="{ item }">
            {{ item.granjaNome || '-' }}
          </template>

          <template #item.actions="{ item }">
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
    </template>

    <!-- Create/Edit Dialog -->
    <v-dialog v-model="dialogOpen" max-width="600" persistent>
      <v-card>
        <v-card-title class="text-h6">
          {{ isEditMode ? 'Editar Transacao' : 'Nova Transacao' }}
        </v-card-title>
        <v-card-text>
          <v-form ref="formRef" @submit.prevent="handleSubmit">
            <v-select
              v-model="formTipo"
              label="Tipo"
              :items="tipoOptions"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model="formCategoria"
              label="Categoria"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model="formDescricao"
              label="Descricao"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formValor"
              label="Valor (R$)"
              type="number"
              :rules="valorRules"
              class="mb-2"
            />
            <v-text-field
              v-model="formData"
              label="Data"
              type="date"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-select
              v-model="formGranjaId"
              label="Granja (opcional)"
              :items="granjas"
              item-title="nome"
              item-value="id"
              clearable
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
          Tem certeza que deseja excluir a transacao
          <strong>{{ deletingDesc }}</strong>?
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
