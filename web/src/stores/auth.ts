import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(null)
  const user = ref<{ email: string; role: string } | null>(null)

  const isAuthenticated = computed(() => !!token.value)

  function setAuth(newToken: string, newUser: { email: string; role: string }) {
    token.value = newToken
    user.value = newUser
  }

  function logout() {
    token.value = null
    user.value = null
  }

  return { token, user, isAuthenticated, setAuth, logout }
})
