<script setup lang="ts">
// Root application component
</script>

<template>
  <v-app>
    <v-main>
      <v-container>
        <p>GranjaTech Vue 3 + Vuetify</p>
      </v-container>
    </v-main>
  </v-app>
</template>

<style>
/* Global font family */
body {
  font-family: 'Inter', 'Roboto', 'Helvetica', 'Arial', sans-serif;
}

/* Smooth theme transitions */
* {
  transition: color 0.2s ease-in-out, background-color 0.2s ease-in-out, border-color 0.2s ease-in-out;
}

/* Custom scrollbar */
*::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.v-theme--light *::-webkit-scrollbar-track,
.v-theme--light::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.v-theme--light *::-webkit-scrollbar-thumb,
.v-theme--light::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.v-theme--light *::-webkit-scrollbar-thumb:hover,
.v-theme--light::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

.v-theme--dark *::-webkit-scrollbar-track,
.v-theme--dark::-webkit-scrollbar-track {
  background: #2A2A2A;
  border-radius: 4px;
}

.v-theme--dark *::-webkit-scrollbar-thumb,
.v-theme--dark::-webkit-scrollbar-thumb {
  background: #555555;
  border-radius: 4px;
}

.v-theme--dark *::-webkit-scrollbar-thumb:hover,
.v-theme--dark::-webkit-scrollbar-thumb:hover {
  background: #666666;
}

/* Focus visible */
.v-theme--light *:focus-visible {
  outline: 2px solid #2E7D32;
  outline-offset: 2px;
}

.v-theme--dark *:focus-visible {
  outline: 2px solid #90CAF9;
  outline-offset: 2px;
}

/* Selection */
.v-theme--light ::selection {
  background-color: #66BB6A;
  color: #ffffff;
}

.v-theme--dark ::selection {
  background-color: #388E3C;
  color: #ffffff;
}
</style>
