<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

withDefaults(defineProps<{
  title?: string
  subtitle?: string
  showBreadcrumbs?: boolean
}>(), {
  showBreadcrumbs: true,
})

const route = useRoute()

const routeLabels: Record<string, string> = {
  '/': 'Dashboard',
  '/granjas': 'Granjas',
  '/lotes': 'Lotes',
  '/estoque': 'Estoque',
  '/sensores': 'Sensores',
  '/financeiro': 'Financeiro',
  '/relatorios': 'Relatorios',
  '/usuarios': 'Usuarios',
  '/auditoria': 'Auditoria',
  '/perfil': 'Perfil',
  '/avicultura': 'Avicultura Pro',
  '/consumo': 'Consumo',
  '/pesagem': 'Pesagens',
  '/sanitario': 'Sanitario',
}

const breadcrumbItems = computed(() => {
  const segments = route.path.split('/').filter(Boolean)
  const items: Array<{ title: string; to?: string }> = [
    { title: 'Dashboard', to: '/' },
  ]

  let currentPath = ''
  segments.forEach((segment) => {
    currentPath += `/${segment}`
    items.push({
      title: routeLabels[currentPath] || segment,
      to: currentPath,
    })
  })

  return items
})
</script>

<template>
  <v-container fluid class="pa-6" style="max-width: 1280px">
    <v-breadcrumbs
      v-if="showBreadcrumbs && route.path !== '/'"
      :items="breadcrumbItems"
      class="mb-4 pa-0"
    >
      <template #divider>
        <v-icon icon="mdi-chevron-right" size="small" />
      </template>
    </v-breadcrumbs>

    <div
      v-if="title || $slots.action"
      class="d-flex flex-column flex-sm-row align-start align-sm-center justify-space-between ga-4 mb-6"
    >
      <div>
        <h1 v-if="title" class="text-h4 font-weight-bold">
          {{ title }}
        </h1>
        <p v-if="subtitle" class="text-body-1 text-medium-emphasis mt-1">
          {{ subtitle }}
        </p>
      </div>
      <div v-if="$slots.action" class="flex-shrink-0">
        <slot name="action" />
      </div>
    </div>

    <v-fade-transition>
      <slot />
    </v-fade-transition>
  </v-container>
</template>
