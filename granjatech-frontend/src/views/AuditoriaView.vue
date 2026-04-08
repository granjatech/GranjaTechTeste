<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'

interface AuditLog {
  id: number
  acao: string
  detalhes: string
  usuarioId: number
  usuarioNome: string
  dataHora: string
}

const logs = ref<AuditLog[]>([])
const loading = ref(true)
const error = ref('')
const search = ref('')

const headers = [
  { title: 'Acao', key: 'acao', sortable: true },
  { title: 'Detalhes', key: 'detalhes', sortable: false },
  { title: 'Usuario', key: 'usuarioNome', sortable: true },
  { title: 'Data/Hora', key: 'dataHora', sortable: true },
]

function getActionColor(action: string): string {
  const lower = action?.toLowerCase() || ''
  if (lower.includes('criar')) return 'success'
  if (lower.includes('editar') || lower.includes('atualizar')) return 'warning'
  if (lower.includes('excluir') || lower.includes('deletar')) return 'error'
  if (lower.includes('login')) return 'info'
  return 'default'
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString('pt-BR')
}

async function fetchLogs() {
  try {
    loading.value = true
    error.value = ''
    const response = await api.get('/auditoria')
    logs.value = response.data
  } catch (err) {
    console.error('Erro ao buscar logs de auditoria:', err)
    error.value = 'Erro ao carregar logs de auditoria. Tente novamente.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchLogs()
})
</script>

<template>
  <PageContainer
    title="Auditoria"
    subtitle="Registro de acoes do sistema para compliance e seguranca"
  >
    <v-alert
      v-if="error"
      type="error"
      class="mb-4"
      closable
      @click:close="error = ''"
    >
      {{ error }}
    </v-alert>

    <v-card>
      <v-card-text class="pb-0">
        <v-text-field
          v-model="search"
          prepend-inner-icon="mdi-magnify"
          label="Buscar nos logs..."
          single-line
          hide-details
          density="compact"
          class="mb-4"
          clearable
        />
      </v-card-text>

      <v-data-table
        :headers="headers"
        :items="logs"
        :search="search"
        :loading="loading"
        loading-text="Carregando logs de auditoria..."
        no-data-text="Nenhum log de auditoria encontrado"
        items-per-page="25"
        class="elevation-0"
      >
        <template #item.acao="{ item }">
          <v-chip
            :color="getActionColor(item.acao)"
            variant="outlined"
            size="small"
          >
            {{ item.acao }}
          </v-chip>
        </template>

        <template #item.detalhes="{ item }">
          <span class="text-body-2" style="max-width: 300px; display: inline-block; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
            {{ item.detalhes || 'Sem detalhes' }}
          </span>
        </template>

        <template #item.dataHora="{ item }">
          {{ formatDate(item.dataHora) }}
        </template>
      </v-data-table>
    </v-card>
  </PageContainer>
</template>
