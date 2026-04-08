<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import api from '@/services/api'
import { useAuthStore } from '@/stores/auth'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

interface Granja {
  id: number
  codigo: string
  nome: string
  localizacao: string | null
  capacidadeTotal: number | null
  tipoProducao: string | null
  status: string
  usuarioId: number
  usuarioNome: string
  dataCriacao: string
}

interface GranjaForm {
  nome: string
  localizacao: string
  capacidadeTotal: number | null
  tipoProducao: string
}

const auth = useAuthStore()
const canCreate = computed(() => auth.user?.role !== 'Financeiro')

const items = ref<Granja[]>([])
const loading = ref(true)
const search = ref('')
const dialogOpen = ref(false)
const deleteDialogOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const deletingId = ref<number | null>(null)
const deletingName = ref('')
const saving = ref(false)
const formRef = ref()

// Form fields
const formNome = ref('')
const formLocalizacao = ref('')
const formCapacidadeTotal = ref<number | null>(null)
const formTipoProducao = ref('')

// Snackbar
const snackbar = ref({ show: false, text: '', color: 'success' })

const headers = [
  { title: 'Codigo', key: 'codigo', sortable: true },
  { title: 'Nome', key: 'nome', sortable: true },
  { title: 'Localizacao', key: 'localizacao', sortable: true },
  { title: 'Tipo Producao', key: 'tipoProducao' },
  { title: 'Status', key: 'status' },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

const tipoProducaoOptions = ['Corte', 'Postura', 'Matrizes']

const nomeRules = [(v: string) => !!v || 'Nome e obrigatorio']

function showSnackbar(text: string, color: string = 'success') {
  snackbar.value = { show: true, text, color }
}

function openCreate() {
  isEditMode.value = false
  editingId.value = null
  formNome.value = ''
  formLocalizacao.value = ''
  formCapacidadeTotal.value = null
  formTipoProducao.value = ''
  dialogOpen.value = true
}

function openEdit(granja: Granja) {
  isEditMode.value = true
  editingId.value = granja.id
  formNome.value = granja.nome
  formLocalizacao.value = granja.localizacao || ''
  formCapacidadeTotal.value = granja.capacidadeTotal
  formTipoProducao.value = granja.tipoProducao || ''
  dialogOpen.value = true
}

function openDelete(granja: Granja) {
  deletingId.value = granja.id
  deletingName.value = granja.nome
  deleteDialogOpen.value = true
}

function closeDialog() {
  dialogOpen.value = false
  formRef.value?.resetValidation()
}

async function handleSubmit() {
  if (!formRef.value) return
  const { valid } = await formRef.value.validate()
  if (!valid) return

  saving.value = true
  try {
    const payload: GranjaForm = {
      nome: formNome.value,
      localizacao: formLocalizacao.value,
      capacidadeTotal: formCapacidadeTotal.value,
      tipoProducao: formTipoProducao.value,
    }

    if (isEditMode.value && editingId.value) {
      await api.put(`/granjas/${editingId.value}`, payload)
      showSnackbar('Granja atualizada com sucesso!')
    } else {
      await api.post('/granjas', payload)
      showSnackbar('Granja criada com sucesso!')
    }
    dialogOpen.value = false
    await fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao salvar granja.',
      'error'
    )
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/granjas/${deletingId.value}`)
    showSnackbar('Granja excluida com sucesso!')
    deleteDialogOpen.value = false
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao excluir granja.',
      'error'
    )
    deleteDialogOpen.value = false
  }
}

async function fetchData() {
  try {
    loading.value = true
    const response = await api.get('/granjas')
    items.value = response.data
  } catch (err) {
    showSnackbar('Erro ao carregar lista de granjas.', 'error')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer title="Granjas" subtitle="Gerenciamento de granjas">
    <template #action>
      <v-btn
        v-if="canCreate"
        color="primary"
        prepend-icon="mdi-plus"
        @click="openCreate"
      >
        Nova Granja
      </v-btn>
    </template>

    <LoadingSpinner v-if="loading" message="Carregando granjas..." />

    <v-card v-else>
      <v-card-text class="pb-0">
        <v-text-field
          v-model="search"
          prepend-inner-icon="mdi-magnify"
          label="Buscar granjas..."
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
        no-data-text="Nenhuma granja encontrada"
        items-per-page="25"
        class="elevation-0"
      >
        <template #item.status="{ item }">
          <v-chip
            :color="item.status === 'Ativa' ? 'success' : 'default'"
            variant="outlined"
            size="small"
          >
            {{ item.status }}
          </v-chip>
        </template>

        <template #item.localizacao="{ item }">
          {{ item.localizacao || 'Nao informado' }}
        </template>

        <template #item.tipoProducao="{ item }">
          {{ item.tipoProducao || '-' }}
        </template>

        <template #item.actions="{ item }">
          <template v-if="canCreate">
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
        </template>
      </v-data-table>
    </v-card>

    <!-- Create/Edit Dialog -->
    <v-dialog v-model="dialogOpen" max-width="600" persistent>
      <v-card>
        <v-card-title class="text-h6">
          {{ isEditMode ? 'Editar Granja' : 'Nova Granja' }}
        </v-card-title>
        <v-card-text>
          <v-form ref="formRef" @submit.prevent="handleSubmit">
            <v-text-field
              v-model="formNome"
              label="Nome"
              :rules="nomeRules"
              class="mb-2"
            />
            <v-text-field
              v-model="formLocalizacao"
              label="Localizacao"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formCapacidadeTotal"
              label="Capacidade Total"
              type="number"
              class="mb-2"
            />
            <v-select
              v-model="formTipoProducao"
              label="Tipo de Producao"
              :items="tipoProducaoOptions"
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
          Tem certeza que deseja excluir a granja
          <strong>{{ deletingName }}</strong>?
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
