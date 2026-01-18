import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, LoginRequest, UserRole } from '@/types'
import { api } from '@/composables/useApi'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => !!user.value)

  const isAdmin = computed(() => {
    return user.value?.role === 'super_admin' || user.value?.role === 'admin'
  })

  const canEdit = computed(() => {
    const editRoles: UserRole[] = ['super_admin', 'admin', 'editor']
    return user.value ? editRoles.includes(user.value.role) : false
  })

  async function login(credentials: LoginRequest): Promise<boolean> {
    loading.value = true
    error.value = null

    try {
      const response = await api.post<{ user: User }>('/api/auth/login', credentials)
      user.value = response.user
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function logout(): Promise<void> {
    try {
      await api.post('/api/auth/logout', {})
    } finally {
      user.value = null
    }
  }

  async function fetchUser(): Promise<void> {
    loading.value = true
    try {
      const userData = await api.get<User>('/api/auth/me')
      user.value = userData
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  return {
    user,
    loading,
    error,
    isAuthenticated,
    isAdmin,
    canEdit,
    login,
    logout,
    fetchUser,
  }
})
