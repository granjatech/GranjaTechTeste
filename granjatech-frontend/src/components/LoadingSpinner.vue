<script setup lang="ts">
withDefaults(defineProps<{
  message?: string
  size?: number
  fullScreen?: boolean
}>(), {
  message: 'Carregando...',
  size: 40,
  fullScreen: false,
})
</script>

<template>
  <div
    :class="['loading-spinner', { 'loading-spinner--fullscreen': fullScreen }]"
    :style="{ minHeight: fullScreen ? '100vh' : '200px' }"
  >
    <v-progress-circular
      :size="size"
      :width="4"
      color="primary"
      indeterminate
    />
    <span v-if="message" class="text-body-2 text-medium-emphasis">
      {{ message }}
    </span>
  </div>
</template>

<style scoped>
.loading-spinner {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
}

.loading-spinner--fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
}

.v-theme--light .loading-spinner--fullscreen {
  background-color: rgba(255, 255, 255, 0.9);
}

.v-theme--dark .loading-spinner--fullscreen {
  background-color: rgba(18, 18, 18, 0.9);
}
</style>
