<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const auth = useAuthStore()

const email = ref('')
const senha = ref('')
const error = ref('')
const loading = ref(false)

const isFormValid = computed(() => {
  return email.value.trim() !== '' && senha.value.trim() !== ''
})

async function handleSubmit() {
  loading.value = true
  error.value = ''

  try {
    await auth.login(email.value, senha.value)
    router.push('/')
  } catch {
    error.value = 'Email ou senha invalidos.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-wrapper">
    <v-card max-width="448" class="mx-auto" rounded="xl" elevation="24">
      <v-card-text class="pa-12">
        <!-- Brand -->
        <div class="d-flex flex-column align-center mb-8">
          <v-avatar size="80" color="primary" class="mb-4">
            <v-icon icon="mdi-sprout" size="40" />
          </v-avatar>
          <h1 class="text-h4 font-weight-bold text-primary mb-2">GranjaTech</h1>
          <p class="text-body-1 text-medium-emphasis text-center">
            Sistema de Gestao Agropecuaria
          </p>
        </div>

        <!-- Form -->
        <v-form @submit.prevent="handleSubmit">
          <v-text-field
            v-model="email"
            label="Email"
            type="email"
            variant="outlined"
            rounded="lg"
            :disabled="loading"
            autofocus
            class="mb-4"
          />

          <v-text-field
            v-model="senha"
            label="Senha"
            type="password"
            variant="outlined"
            rounded="lg"
            :disabled="loading"
            class="mb-6"
          />

          <v-alert v-if="error" type="error" class="mb-6">
            {{ error }}
          </v-alert>

          <v-btn
            type="submit"
            block
            size="large"
            rounded="lg"
            color="primary"
            :disabled="loading || !isFormValid"
            :loading="loading"
            style="background: linear-gradient(135deg, #2E7D32, #4CAF50)"
          >
            Entrar
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>
.login-wrapper {
  min-height: 100vh;
  background: linear-gradient(135deg, #2E7D32 0%, #66BB6A 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
}
</style>
