<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import api from '@/services/api'
import { useAuthStore } from '@/stores/auth'
import PageContainer from '@/components/PageContainer.vue'
import { PERFIL_OPTIONS } from '@/constants/perfis'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

interface Usuario {
  id: number
  nome: string
  email: string
  perfilId: number
  perfilNome: string
}

const auth = useAuthStore()
const isAdmin = computed(() => auth.user?.role === 'Administrador')

const users = ref<Usuario[]>([])
const loading = ref(true)
const search = ref('')

// Dialog state
const dialogOpen = ref(false)
const deleteDialogOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const deletingId = ref<number | null>(null)
const deletingName = ref('')
const formRef = ref()
const saving = ref(false)

// Form fields
const formNome = ref('')
const formEmail = ref('')
const formSenha = ref('')
const formPerfilId = ref<number | null>(null)

// Snackbar
const snackbar = ref({ show: false, text: '', color: 'success' })

const headers = [
  { title: 'Nome', key: 'nome', sortable: true },
  { title: 'Email', key: 'email', sortable: true },
  { title: 'Perfil', key: 'perfilNome', sortable: true },
  { title: 'Acoes', key: 'actions', sortable: false, align: 'end' as const },
]

const perfilOptions = PERFIL_OPTIONS

const requiredRule = [(v: string) => !!v || 'Campo obrigatorio']
const emailRules = [
  (v: string) => !!v || 'Email obrigatorio',
  (v: string) => /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/.test(v) || 'Email invalido',
]
const perfilRule = [(v: number | null) => !!v || 'Perfil obrigatorio']
const senhaCreateRule = [
  (v: string) => !!v || 'Senha obrigatoria',
  (v: string) => v.length >= 6 || 'Senha deve ter no minimo 6 caracteres',
]

function getRoleColor(role: string): string {
  switch (role) {
    case 'Administrador': return 'error'
    case 'Financeiro': return 'warning'
    case 'Produtor': return 'success'
    default: return 'default'
  }
}

function showSnackbar(text: string, color: string = 'success') {
  snackbar.value = { show: true, text, color }
}

function openCreate() {
  isEditMode.value = false
  editingId.value = null
  formNome.value = ''
  formEmail.value = ''
  formSenha.value = ''
  formPerfilId.value = null
  dialogOpen.value = true
}

function openEdit(user: Usuario) {
  isEditMode.value = true
  editingId.value = user.id
  formNome.value = user.nome
  formEmail.value = user.email
  formSenha.value = ''
  formPerfilId.value = user.perfilId
  dialogOpen.value = true
}

function openDelete(user: Usuario) {
  deletingId.value = user.id
  deletingName.value = user.nome
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
    if (isEditMode.value && editingId.value) {
      await api.put(`/auth/usuarios/${editingId.value}`, {
        nome: formNome.value,
        email: formEmail.value,
        perfilId: formPerfilId.value,
      })
      showSnackbar('Usuario atualizado com sucesso!')
    } else {
      await api.post('/auth/register', {
        nome: formNome.value,
        email: formEmail.value,
        senha: formSenha.value,
        perfilId: formPerfilId.value,
      })
      showSnackbar('Usuario criado com sucesso!')
    }
    dialogOpen.value = false
    fetchUsers()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao salvar usuario.',
      'error'
    )
  } finally {
    saving.value = false
  }
}

async function handleDelete() {
  if (!deletingId.value) return
  try {
    await api.delete(`/auth/usuarios/${deletingId.value}`)
    showSnackbar('Usuario excluido com sucesso!')
    deleteDialogOpen.value = false
    fetchUsers()
  } catch (err: any) {
    showSnackbar(
      err.response?.data?.message || 'Erro ao excluir usuario.',
      'error'
    )
    deleteDialogOpen.value = false
  }
}

async function fetchUsers() {
  try {
    loading.value = true
    const response = await api.get('/auth/usuarios')
    users.value = response.data
  } catch (err) {
    showSnackbar('Erro ao carregar lista de usuarios.', 'error')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  if (isAdmin.value) {
    fetchUsers()
  } else {
    loading.value = false
  }
})
</script>

<template>
  <PageContainer title="Usuarios" subtitle="Gerenciamento de usuarios do sistema">
    <template #action>
      <v-btn
        v-if="isAdmin"
        color="primary"
        prepend-icon="mdi-plus"
        @click="openCreate"
      >
        Novo Usuario
      </v-btn>
    </template>

    <!-- Non-admin warning -->
    <v-alert v-if="!isAdmin" type="warning" class="mb-4">
      Apenas administradores podem gerenciar usuarios.
    </v-alert>

    <template v-else>
      <LoadingSpinner v-if="loading" message="Carregando usuarios..." />

      <v-card v-else>
        <v-card-text class="pb-0">
          <v-text-field
            v-model="search"
            prepend-inner-icon="mdi-magnify"
            label="Buscar usuarios..."
            single-line
            hide-details
            density="compact"
            class="mb-4"
            clearable
          />
        </v-card-text>

        <v-data-table
          :headers="headers"
          :items="users"
          :search="search"
          no-data-text="Nenhum usuario encontrado"
          items-per-page="25"
          class="elevation-0"
        >
          <template #item.perfilNome="{ item }">
            <v-chip
              :color="getRoleColor(item.perfilNome)"
              variant="outlined"
              size="small"
            >
              {{ item.perfilNome }}
            </v-chip>
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
    <v-dialog v-model="dialogOpen" max-width="500" persistent>
      <v-card>
        <v-card-title class="text-h6">
          {{ isEditMode ? 'Editar Usuario' : 'Novo Usuario' }}
        </v-card-title>
        <v-card-text>
          <v-form ref="formRef" @submit.prevent="handleSubmit">
            <v-text-field
              v-model="formNome"
              label="Nome"
              :rules="requiredRule"
              class="mb-2"
            />
            <v-text-field
              v-model="formEmail"
              label="Email"
              type="email"
              :rules="emailRules"
              class="mb-2"
            />
            <v-text-field
              v-if="!isEditMode"
              v-model="formSenha"
              label="Senha"
              type="password"
              :rules="senhaCreateRule"
              class="mb-2"
            />
            <v-select
              v-model="formPerfilId"
              label="Perfil"
              :items="perfilOptions"
              :rules="perfilRule"
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
            Salvar
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="deleteDialogOpen" max-width="400">
      <v-card>
        <v-card-title class="text-h6">Confirmar Exclusao</v-card-title>
        <v-card-text>
          Tem certeza que deseja excluir o usuario
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
