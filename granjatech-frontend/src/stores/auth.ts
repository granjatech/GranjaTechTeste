import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { jwtDecode } from 'jwt-decode'
import api from '@/services/api'

interface JwtPayload {
  nameid: string
  email: string
  role: string
  exp: number
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const user = ref<JwtPayload | null>(null)

  // Hydrate from localStorage on init
  if (token.value) {
    try {
      const decoded = jwtDecode<JwtPayload>(token.value)
      // Check if token is expired (per Research pitfall 4)
      if (decoded.exp * 1000 < Date.now()) {
        token.value = null
        localStorage.removeItem('token')
      } else {
        user.value = decoded
      }
    } catch {
      token.value = null
      localStorage.removeItem('token')
    }
  }

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  async function login(email: string, senha: string) {
    const response = await api.post('/auth/login', { email, senha })
    const newToken: string = response.data.token
    token.value = newToken
    user.value = jwtDecode<JwtPayload>(newToken)
    localStorage.setItem('token', newToken)
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
  }

  return { token, user, isAuthenticated, login, logout }
})
