<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'
import { useFormatters } from '@/composables/useFormatters'

const { formatCurrency } = useFormatters()

interface Produto {
  id: number
  nome: string
  categoria: string
  unidade: string
  quantidadeAtual: number
  quantidadeMinima: number
  precoUnitario: number
  fornecedor: string | null
  observacoes: string | null
}

const items = ref<Produto[]>([])
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
const formCategoria = ref('')
const formUnidade = ref('')
const formQuantidadeAtual = ref<number | null>(null)
const formQuantidadeMinima = ref<number | null>(null)
const formPrecoUnitario = ref<number | null>(null)
const formFornecedor = ref('')
const formObservacoes = ref('')

// Snackbar
const snackbar = ref({ show: false, text: '', color: 'success' })

const headers = [
  { title: 'Nome', key: 'nome', sortable: true },
  { title: 'Categoria', key: 'categoria', sortable: true },
  { title: 'Unidade', key: 'unidade' },
  { title: 'Qtd Atual', key: 'quantidadeAtual' },
  { title: 'Qtd Minima', key: 'quantidadeMinima' },
  { title: 'Preco Unitario', key: 'precoUnitario' },
  { title: 'Fornecedor', key: 'fornecedor' },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

const categoriaOptions = ['Racao', 'Medicamento', 'Vacina', 'Equipamento', 'Outros']
const unidadeOptions = ['kg', 'L', 'un', 'dose', 'cx']

const requiredRule = [(v: any) => !!v || v === 0 || 'Campo obrigatorio']
const minZeroRule = [
  (v: any) => v !== null && v !== '' && v !== undefined || 'Campo obrigatorio',
  (v: number) => v >= 0 || 'Valor deve ser no minimo 0',
]

function showSnackbar(text: string, color: string = 'success') {
  snackbar.value = { show: true, text, color }
}

function isLowStock(item: Produto): boolean {
  return item.quantidadeAtual <= item.quantidadeMinima
}

function openCreate() {
  isEditMode.value = false
  editingId.value = null
  formNome.value = ''
  formCategoria.value = ''
  formUnidade.value = ''
  formQuantidadeAtual.value = null
  formQuantidadeMinima.value = null
  formPrecoUnitario.value = null
  formFornecedor.value = ''
  formObservacoes.value = ''
  dialogOpen.value = true
}

function openEdit(produto: Produto) {
  isEditMode.value = true
  editingId.value = produto.id
  formNome.value = produto.nome
  formCategoria.value = produto.categoria
  formUnidade.value = produto.unidade
  formQuantidadeAtual.value = produto.quantidadeAtual
  formQuantidadeMinima.value = produto.quantidadeMinima
  formPrecoUnitario.value = produto.precoUnitario
  formFornecedor.value = produto.fornecedor || ''
  formObservacoes.value = produto.observacoes || ''
  dialogOpen.value = true
}

function openDelete(produto: Produto) {
  deletingId.value = produto.id
  deletingName.value = produto.nome
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
    const payload = {
      nome: formNome.value,
      categoria: formCategoria.value,
      unidade: formUnidade.value,
      quantidadeAtual: formQuantidadeAtual.value,
      quantidadeMinima: formQuantidadeMinima.value,
      precoUnitario: formPrecoUnitario.value,
      fornecedor: formFornecedor.value || null,
      observacoes: formObservacoes.value || null,
    }

    if (isEditMode.value && editingId.value) {
      await api.put(`/estoque/${editingId.value}`, payload)
      showSnackbar('Produto atualizado com sucesso!')
    } else {
      await api.post('/estoque', payload)
      showSnackbar('Produto criado com sucesso!')
    }
    dialogOpen.value = false
    await fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao salvar produto.',
      'error'
    )
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/estoque/${deletingId.value}`)
    showSnackbar('Produto excluido com sucesso!')
    deleteDialogOpen.value = false
    fetchData()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao excluir produto.',
      'error'
    )
    deleteDialogOpen.value = false
  }
}

async function fetchData() {
  try {
    loading.value = true
    const response = await api.get('/estoque')
    items.value = response.data
  } catch (err) {
    showSnackbar('Erro ao carregar lista de estoque.', 'error')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer title="Estoque" subtitle="Controle de produtos e insumos">
    <template #action>
      <v-btn
        color="primary"
        prepend-icon="mdi-plus"
        @click="openCreate"
      >
        Novo Produto
      </v-btn>
    </template>

    <LoadingSpinner v-if="loading" message="Carregando estoque..." />

    <v-card v-else>
      <v-card-text class="pb-0">
        <v-text-field
          v-model="search"
          prepend-inner-icon="mdi-magnify"
          label="Buscar produtos..."
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
        no-data-text="Nenhum produto em estoque"
        items-per-page="25"
        class="elevation-0"
      >
        <template #item.quantidadeAtual="{ item }">
          <span :class="isLowStock(item) ? 'text-error font-weight-bold' : ''">
            {{ item.quantidadeAtual }}
          </span>
        </template>

        <template #item.precoUnitario="{ item }">
          {{ formatCurrency(item.precoUnitario) }}
        </template>

        <template #item.fornecedor="{ item }">
          {{ item.fornecedor || '-' }}
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

    <!-- Create/Edit Dialog -->
    <v-dialog v-model="dialogOpen" max-width="600" persistent>
      <v-card>
        <v-card-title class="text-h6">
          {{ isEditMode ? 'Editar Produto' : 'Novo Produto' }}
        </v-card-title>
        <v-card-text>
          <v-form ref="formRef" @submit.prevent="handleSubmit">
            <v-text-field
              v-model="formNome"
              label="Nome"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-select
              v-model="formCategoria"
              label="Categoria"
              :items="categoriaOptions"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-select
              v-model="formUnidade"
              label="Unidade"
              :items="unidadeOptions"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formQuantidadeAtual"
              label="Quantidade Atual"
              type="number"
              :rules="minZeroRule"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formQuantidadeMinima"
              label="Quantidade Minima"
              type="number"
              :rules="minZeroRule"
              class="mb-2"
            />
            <v-text-field
              v-model.number="formPrecoUnitario"
              label="Preco Unitario"
              type="number"
              :rules="minZeroRule"
              class="mb-2"
            />
            <v-text-field
              v-model="formFornecedor"
              label="Fornecedor"
              class="mb-2"
            />
            <v-textarea
              v-model="formObservacoes"
              label="Observacoes"
              rows="3"
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
          Tem certeza que deseja excluir o produto
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
