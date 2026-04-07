import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/LoginView.vue'),
    meta: { requiresAuth: false },
  },
  {
    path: '/',
    name: 'Dashboard',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/granjas',
    name: 'Granjas',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/lotes',
    name: 'Lotes',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/estoque',
    name: 'Estoque',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/avicultura',
    name: 'Avicultura Pro',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/consumo',
    name: 'Consumo',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/pesagem',
    name: 'Pesagens',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/sanitario',
    name: 'Sanitario',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/sensores',
    name: 'Sensores',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/financeiro',
    name: 'Financeiro',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/relatorios',
    name: 'Relatorios',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/usuarios',
    name: 'Usuarios',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/auditoria',
    name: 'Auditoria',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/perfil',
    name: 'Perfil',
    component: () => import('@/views/PlaceholderView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/',
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to) => {
  // useAuthStore is called inside beforeEach (not at module top-level)
  // because Pinia must be installed before store can be used
  const auth = useAuthStore()

  if (to.meta.requiresAuth !== false && !auth.isAuthenticated) {
    return { name: 'Login' }
  }

  if (to.name === 'Login' && auth.isAuthenticated) {
    return { path: '/' }
  }
})

export default router
