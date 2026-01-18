import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const sidebarOpen = ref(false)
  const loading = ref(false)
  const notification = ref<{ message: string; type: 'success' | 'error' | 'info' } | null>(null)

  function toggleSidebar(): void {
    sidebarOpen.value = !sidebarOpen.value
  }

  function showNotification(message: string, type: 'success' | 'error' | 'info' = 'info'): void {
    notification.value = { message, type }
    setTimeout(() => {
      notification.value = null
    }, 5000)
  }

  function clearNotification(): void {
    notification.value = null
  }

  return {
    sidebarOpen,
    loading,
    notification,
    toggleSidebar,
    showNotification,
    clearNotification,
  }
})
