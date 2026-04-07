<script setup lang="ts">
// Stub - will be fully implemented in Task 2
defineProps<{
  title?: string
  subtitle?: string
}>()
</script>

<template>
  <div>
    <h1 v-if="title">{{ title }}</h1>
    <slot />
  </div>
</template>
