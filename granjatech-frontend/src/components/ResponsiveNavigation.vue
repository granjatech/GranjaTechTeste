<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDisplay } from 'vuetify'
import { useAuthStore } from '@/stores/auth'
import { useAccessibilityStore } from '@/stores/accessibility'

const route = useRoute()
const router = useRouter()
const { mdAndUp } = useDisplay()
const auth = useAuthStore()
const accessibility = useAccessibilityStore()

const mobileOpen = ref(false)
const menuOpen = ref(false)

const navigationItems = [
  { path: '/', label: 'Dashboard', icon: 'mdi-view-dashboard', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/granjas', label: 'Granjas', icon: 'mdi-barn', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/lotes', label: 'Lotes', icon: 'mdi-bird', roles: ['Administrador', 'Produtor', 'Financeiro'] },
  { path: '/estoque', label: 'Estoque', icon: 'mdi-package-variant', roles: ['Administrador', 'Produtor'] },
  { path: '/avicultura', label: 'Avicultura Pro', icon: 'mdi-chart-line', roles: ['Administrador', 'Produtor'] },
  { path: '/consumo', label: 'Consumo', icon: 'mdi-food-drumstick', roles: ['Administrador', 'Produtor'] },
  { path: '/pesagem', label: 'Pesagens', icon: 'mdi-scale', roles: ['Administrador', 'Produtor'] },
  { path: '/sanitario', label: 'Sanitario', icon: 'mdi-medical-bag', roles: ['Administrador', 'Produtor'] },
  { path: '/sensores', label: 'Sensores', icon: 'mdi-access-point', roles: ['Administrador', 'Produtor'] },
  { path: '/financeiro', label: 'Financeiro', icon: 'mdi-currency-usd', roles: ['Administrador', 'Financeiro'] },
  { path: '/relatorios', label: 'Relatorios', icon: 'mdi-file-chart', roles: ['Administrador', 'Financeiro', 'Produtor'] },
  { path: '/usuarios', label: 'Usuarios', icon: 'mdi-account-group', roles: ['Administrador'] },
  { path: '/auditoria', label: 'Auditoria', icon: 'mdi-shield-check', roles: ['Administrador'] },
]

const filteredNavItems = computed(() => {
  if (!auth.user?.role) return []
  return navigationItems.filter((item) => item.roles.includes(auth.user!.role))
})

const currentPageTitle = computed(() => {
  const found = navigationItems.find((item) => item.path === route.path)
  return found?.label || 'GranjaTech'
})

const userInitial = computed(() => {
  return auth.user?.email?.charAt(0).toUpperCase() || '?'
})

function handleLogout() {
  auth.logout()
  router.push('/login')
}

// Close mobile drawer on navigation
watch(() => route.path, () => {
  mobileOpen.value = false
})
</script>

<template>
  <!-- App Bar -->
  <v-app-bar flat border="b">
    <v-app-bar-nav-icon
      class="d-md-none"
      @click="mobileOpen = !mobileOpen"
    />

    <v-app-bar-title>{{ currentPageTitle }}</v-app-bar-title>

    <template #append>
      <div class="d-flex align-center ga-1 ga-sm-2 mr-sm-2">
        <!-- Dark mode toggle -->
        <v-tooltip :text="accessibility.mode === 'dark' ? 'Ativar tema claro' : 'Ativar tema escuro'">
          <template #activator="{ props }">
            <v-btn icon v-bind="props" @click="accessibility.toggleColorMode">
              <v-icon>{{ accessibility.mode === 'dark' ? 'mdi-weather-sunny' : 'mdi-weather-night' }}</v-icon>
            </v-btn>
          </template>
        </v-tooltip>

        <!-- Font decrease -->
        <v-tooltip text="Diminuir tamanho da fonte">
          <template #activator="{ props }">
            <v-btn
              icon
              v-bind="props"
              :disabled="!accessibility.canDecreaseFont"
              @click="accessibility.decreaseFontScale"
            >
              <v-icon>mdi-format-font-size-decrease</v-icon>
            </v-btn>
          </template>
        </v-tooltip>

        <!-- Font increase -->
        <v-tooltip text="Aumentar tamanho da fonte">
          <template #activator="{ props }">
            <v-btn
              icon
              v-bind="props"
              :disabled="!accessibility.canIncreaseFont"
              @click="accessibility.increaseFontScale"
            >
              <v-icon>mdi-format-font-size-increase</v-icon>
            </v-btn>
          </template>
        </v-tooltip>

        <!-- Reset -->
        <v-tooltip text="Restaurar preferencias padrao">
          <template #activator="{ props }">
            <v-btn icon v-bind="props" @click="accessibility.resetSettings">
              <v-icon>mdi-restart</v-icon>
            </v-btn>
          </template>
        </v-tooltip>

        <!-- User menu -->
        <v-menu v-model="menuOpen">
          <template #activator="{ props }">
            <v-btn v-bind="props" variant="text" class="text-none">
              <v-avatar size="32" color="primary" class="mr-2">
                {{ userInitial }}
              </v-avatar>
              <span class="d-none d-sm-inline">{{ auth.user?.email }}</span>
            </v-btn>
          </template>

          <v-list>
            <v-list-item
              prepend-icon="mdi-account"
              title="Perfil"
              :to="'/perfil'"
              @click="menuOpen = false"
            />
            <v-divider />
            <v-list-item
              prepend-icon="mdi-logout"
              title="Sair"
              @click="handleLogout"
            />
          </v-list>
        </v-menu>
      </div>
    </template>
  </v-app-bar>

  <!-- Navigation Drawer -->
  <v-navigation-drawer
    v-model="mobileOpen"
    :permanent="mdAndUp"
    :temporary="!mdAndUp"
    width="280"
  >
    <!-- Drawer Header -->
    <div class="d-flex align-center pa-6">
      <v-icon icon="mdi-agriculture" color="primary" size="32" />
      <span class="ml-3 text-body-1 font-weight-bold text-primary">GranjaTech</span>
      <v-spacer />
      <v-btn
        v-if="!mdAndUp"
        icon="mdi-close"
        variant="text"
        size="small"
        @click="mobileOpen = false"
      />
    </div>

    <v-divider />

    <!-- User Info -->
    <div class="pa-4 d-flex align-center ga-3">
      <v-avatar size="40" color="primary">
        {{ userInitial }}
      </v-avatar>
      <div>
        <div class="text-body-2 font-weight-bold">{{ auth.user?.email }}</div>
        <v-chip size="small" variant="outlined" color="primary">
          {{ auth.user?.role }}
        </v-chip>
      </div>
    </div>

    <v-divider />

    <!-- Navigation Items -->
    <v-list nav>
      <v-list-item
        v-for="item in filteredNavItems"
        :key="item.path"
        :to="item.path"
        rounded="lg"
        :style="{ minHeight: '48px' }"
        :active="route.path === item.path"
        :class="route.path === item.path ? 'nav-item-active' : ''"
        color="primary"
      >
        <template #prepend>
          <v-icon :icon="item.icon" />
        </template>
        {{ item.label }}
      </v-list-item>
    </v-list>

    <v-divider />

    <!-- Logout -->
    <v-list nav>
      <v-list-item
        rounded="lg"
        color="error"
        :style="{ minHeight: '48px' }"
        @click="handleLogout"
      >
        <template #prepend>
          <v-icon icon="mdi-logout" />
        </template>
        Sair
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>
.nav-item-active {
  font-weight: 700;
}
</style>
