<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js'
import { Bar } from 'vue-chartjs'
import api from '@/services/api'
import PageContainer from '@/components/PageContainer.vue'
import { useFormatters } from '@/composables/useFormatters'

const { formatCurrency } = useFormatters()
import LoadingSpinner from '@/components/LoadingSpinner.vue'

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

interface Kpis {
  totalGranjas: number
  lotesAtivos: number
  totalAves: number
  receitaTotal: number
}

interface MonthlyEntry {
  mes: string
  entradas: number
  saidas: number
}

const kpis = ref<Kpis | null>(null)
const monthlyData = ref<MonthlyEntry[]>([])
const loading = ref(true)
const error = ref('')

const kpiCards = computed(() => {
  if (!kpis.value) return []
  return [
    {
      title: 'Total Granjas',
      value: kpis.value.totalGranjas,
      color: '#2E7D32',
      icon: 'mdi-home-group',
    },
    {
      title: 'Lotes Ativos',
      value: kpis.value.lotesAtivos,
      color: '#1565C0',
      icon: 'mdi-layers',
    },
    {
      title: 'Total de Aves',
      value: kpis.value.totalAves,
      color: '#E65100',
      icon: 'mdi-bird',
    },
    {
      title: 'Receita Total',
      value: formatCurrency(kpis.value.receitaTotal),
      color: '#6A1B9A',
      icon: 'mdi-cash-multiple',
    },
  ]
})

const chartData = computed(() => ({
  labels: monthlyData.value.map((d) => d.mes),
  datasets: [
    {
      label: 'Entradas',
      data: monthlyData.value.map((d) => d.entradas),
      backgroundColor: '#4caf50',
      borderRadius: 4,
    },
    {
      label: 'Saidas',
      data: monthlyData.value.map((d) => d.saidas),
      backgroundColor: '#f44336',
      borderRadius: 4,
    },
  ],
}))

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom' as const,
    },
  },
}

async function fetchData() {
  try {
    loading.value = true
    error.value = ''
    const [kpisRes, monthlyRes] = await Promise.all([
      api.get('/dashboard/kpis'),
      api.get('/dashboard/resumo-mensal'),
    ])
    kpis.value = kpisRes.data
    monthlyData.value = monthlyRes.data
  } catch (err) {
    error.value = 'Erro ao carregar dados do dashboard.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})
</script>

<template>
  <PageContainer
    title="Dashboard"
    subtitle="Visao geral do seu sistema de gestao agropecuaria"
    :show-breadcrumbs="false"
  >
    <LoadingSpinner v-if="loading" message="Carregando dashboard..." />

    <template v-else>
      <v-alert v-if="error" type="error" class="mb-4" closable @click:close="error = ''">
        {{ error }}
      </v-alert>

      <v-row v-if="kpis" class="mb-4">
        <v-col
          v-for="(card, index) in kpiCards"
          :key="index"
          cols="12"
          sm="6"
          lg="3"
        >
          <v-card
            :style="{
              background: `linear-gradient(135deg, ${card.color}, ${card.color}dd)`,
              color: 'white',
            }"
            class="kpi-card"
            height="160"
          >
            <v-card-text class="d-flex flex-column justify-space-between h-100 pa-4">
              <div class="d-flex align-center justify-space-between mb-2">
                <span class="text-subtitle-1 font-weight-medium">{{ card.title }}</span>
                <v-icon :icon="card.icon" size="32" style="opacity: 0.8" />
              </div>
              <span class="text-h4 font-weight-bold">{{ card.value }}</span>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>

      <v-card>
        <v-card-title class="text-h6 font-weight-bold">
          Resumo Financeiro Mensal
        </v-card-title>
        <v-card-text>
          <div style="height: 300px">
            <Bar :data="chartData" :options="chartOptions" />
          </div>
        </v-card-text>
      </v-card>
    </template>
  </PageContainer>
</template>

<style scoped>
.kpi-card {
  cursor: pointer;
  transition: all 0.3s ease-in-out;
}
.kpi-card:hover {
  transform: translateY(-4px);
  box-shadow: 0px 12px 32px rgba(0, 0, 0, 0.2);
}
.h-100 {
  height: 100%;
}
</style>
