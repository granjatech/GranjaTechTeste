<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

interface ProfileDetails {
  id: number
  nome: string
  email: string
  perfil: string
  perfilNome?: string
  granjas?: string[]
  associados?: string[]
}

const loading = ref(true)
const profileDetails = ref<ProfileDetails | null>(null)
const nome = ref('')
const email = ref('')
const senhaAtual = ref('')
const novaSenha = ref('')
const confirmaNovaSenha = ref('')
const showSenhaAtual = ref(false)
const showNovaSenha = ref(false)
const showConfirmaSenha = ref(false)
const profileFormRef = ref()
const passwordFormRef = ref()

const profileMessage = ref<{ type: 'success' | 'error'; text: string } | null>(null)
const passwordMessage = ref<{ type: 'success' | 'error'; text: string } | null>(null)

const requiredRule = [(v: string) => !!v || 'Campo obrigatorio']
const emailRules = [
  (v: string) => !!v || 'Email obrigatorio',
  (v: string) => /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/.test(v) || 'Email invalido',
]

async function fetchProfile() {
  try {
    loading.value = true
    const response = await api.get('/profile')
    profileDetails.value = response.data
    nome.value = response.data.nome
    email.value = response.data.email
  } catch (err) {
    console.error('Erro ao buscar perfil:', err)
    profileMessage.value = { type: 'error', text: 'Erro ao carregar dados do perfil.' }
  } finally {
    loading.value = false
  }
}

async function handleProfileSubmit() {
  const { valid } = await profileFormRef.value.validate()
  if (!valid) return

  profileMessage.value = null
  try {
    await api.put('/profile', { nome: nome.value, email: email.value })
    profileMessage.value = { type: 'success', text: 'Perfil atualizado com sucesso!' }
    fetchProfile()
  } catch (err: any) {
    profileMessage.value = {
      type: 'error',
      text: err.response?.data?.message || 'Erro ao atualizar perfil.',
    }
  }
}

async function handlePasswordSubmit() {
  const { valid } = await passwordFormRef.value.validate()
  if (!valid) return

  passwordMessage.value = null

  if (novaSenha.value !== confirmaNovaSenha.value) {
    passwordMessage.value = { type: 'error', text: 'As novas senhas nao coincidem.' }
    return
  }

  try {
    const response = await api.post('/profile/change-password', {
      senhaAtual: senhaAtual.value,
      novaSenha: novaSenha.value,
    })
    passwordMessage.value = {
      type: 'success',
      text: response.data.message || 'Senha alterada com sucesso!',
    }
    senhaAtual.value = ''
    novaSenha.value = ''
    confirmaNovaSenha.value = ''
    passwordFormRef.value.resetValidation()
  } catch (err: any) {
    passwordMessage.value = {
      type: 'error',
      text: err.response?.data?.message || 'Erro ao alterar a senha.',
    }
  }
}

onMounted(() => {
  fetchProfile()
})
</script>

<template>
  <PageContainer title="Meu Perfil" subtitle="Gerencie seus dados e senha">
    <LoadingSpinner v-if="loading" message="Carregando perfil..." />

    <template v-else>
      <!-- Informacoes do cargo -->
      <v-card v-if="profileDetails" class="mb-4">
        <v-card-title class="text-h6">Informacoes do Cargo</v-card-title>
        <v-card-text>
          <div class="d-flex align-center ga-2 mb-2">
            <span class="font-weight-bold">Cargo:</span>
            <v-chip color="primary" size="small">
              {{ profileDetails.perfil || profileDetails.perfilNome }}
            </v-chip>
          </div>
          <div
            v-if="profileDetails.granjas && profileDetails.granjas.length > 0"
            class="mt-2"
          >
            <span class="font-weight-bold">Granjas associadas:</span>
            <div class="d-flex flex-wrap ga-2 mt-1">
              <v-chip
                v-for="(granja, index) in profileDetails.granjas"
                :key="index"
                size="small"
              >
                {{ granja }}
              </v-chip>
            </div>
          </div>
          <div
            v-if="profileDetails.associados && profileDetails.associados.length > 0"
            class="mt-2"
          >
            <span class="font-weight-bold">
              {{ (profileDetails.perfil || profileDetails.perfilNome) === 'Produtor' ? 'Responsavel por:' : 'Financeiro de:' }}
            </span>
            <div class="d-flex flex-wrap ga-2 mt-1">
              <v-chip
                v-for="(assoc, index) in profileDetails.associados"
                :key="index"
                size="small"
              >
                {{ assoc }}
              </v-chip>
            </div>
          </div>
        </v-card-text>
      </v-card>

      <v-row>
        <!-- Editar Informacoes -->
        <v-col cols="12" md="6">
          <v-card>
            <v-card-title class="text-h6">Editar Informacoes</v-card-title>
            <v-card-text>
              <v-form ref="profileFormRef" @submit.prevent="handleProfileSubmit">
                <v-text-field
                  v-model="nome"
                  label="Nome Completo"
                  :rules="requiredRule"
                  class="mb-2"
                />
                <v-text-field
                  v-model="email"
                  label="Email"
                  type="email"
                  :rules="emailRules"
                  class="mb-2"
                />
                <v-alert
                  v-if="profileMessage"
                  :type="profileMessage.type"
                  class="mb-2"
                  closable
                  @click:close="profileMessage = null"
                >
                  {{ profileMessage.text }}
                </v-alert>
                <v-btn type="submit" color="primary" variant="flat">
                  Salvar Alteracoes
                </v-btn>
              </v-form>
            </v-card-text>
          </v-card>
        </v-col>

        <!-- Alterar Senha -->
        <v-col cols="12" md="6">
          <v-card>
            <v-card-title class="text-h6">Alterar Senha</v-card-title>
            <v-card-text>
              <v-form ref="passwordFormRef" @submit.prevent="handlePasswordSubmit">
                <v-text-field
                  v-model="senhaAtual"
                  label="Senha Atual"
                  :type="showSenhaAtual ? 'text' : 'password'"
                  :rules="requiredRule"
                  :append-inner-icon="showSenhaAtual ? 'mdi-eye-off' : 'mdi-eye'"
                  class="mb-2"
                  @click:append-inner="showSenhaAtual = !showSenhaAtual"
                />
                <v-text-field
                  v-model="novaSenha"
                  label="Nova Senha"
                  :type="showNovaSenha ? 'text' : 'password'"
                  :rules="requiredRule"
                  :append-inner-icon="showNovaSenha ? 'mdi-eye-off' : 'mdi-eye'"
                  class="mb-2"
                  @click:append-inner="showNovaSenha = !showNovaSenha"
                />
                <v-text-field
                  v-model="confirmaNovaSenha"
                  label="Confirmar Nova Senha"
                  :type="showConfirmaSenha ? 'text' : 'password'"
                  :rules="requiredRule"
                  :append-inner-icon="showConfirmaSenha ? 'mdi-eye-off' : 'mdi-eye'"
                  class="mb-2"
                  @click:append-inner="showConfirmaSenha = !showConfirmaSenha"
                />
                <v-alert
                  v-if="passwordMessage"
                  :type="passwordMessage.type"
                  class="mb-2"
                  closable
                  @click:close="passwordMessage = null"
                >
                  {{ passwordMessage.text }}
                </v-alert>
                <v-btn type="submit" color="primary" variant="flat">
                  Alterar Senha
                </v-btn>
              </v-form>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </template>
  </PageContainer>
</template>
