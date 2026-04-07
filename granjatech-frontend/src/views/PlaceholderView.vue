<script setup lang="ts">
import { useRoute } from 'vue-router'
import PageContainer from '@/components/PageContainer.vue'

const route = useRoute()
</script>

<template>
  <PageContainer :title="route.name?.toString() || 'Pagina'">
    <p>Em construcao - sera implementado na Fase 5</p>
  </PageContainer>
</template>
