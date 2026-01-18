import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import type { UserRole } from '@/types'

export function useAuth() {
  const authStore = useAuthStore()

  const user = computed(() => authStore.user)
  const isAuthenticated = computed(() => authStore.isAuthenticated)
  const isAdmin = computed(() => authStore.isAdmin)
  const canEdit = computed(() => authStore.canEdit)

  function hasRole(roles: UserRole[]): boolean {
    if (!authStore.user) return false
    return roles.includes(authStore.user.role)
  }

  function canManageUsers(): boolean {
    return hasRole(['super_admin', 'admin'])
  }

  function canEditArticles(): boolean {
    return hasRole(['super_admin', 'admin', 'editor'])
  }

  function canDeleteArticles(): boolean {
    return hasRole(['super_admin', 'admin'])
  }

  function canViewAudit(): boolean {
    return hasRole(['super_admin', 'admin'])
  }

  return {
    user,
    isAuthenticated,
    isAdmin,
    canEdit,
    hasRole,
    canManageUsers,
    canEditArticles,
    canDeleteArticles,
    canViewAudit,
    login: authStore.login,
    logout: authStore.logout,
  }
}
