<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/composables/useApi'
import { useUiStore } from '@/stores/ui'
import type { User, UserRole } from '@/types'

const uiStore = useUiStore()

const users = ref<User[]>([])
const loading = ref(true)
const editingUser = ref<User | null>(null)

onMounted(async () => {
  await fetchUsers()
})

async function fetchUsers(): Promise<void> {
  loading.value = true
  try {
    const data = await api.get<{ users: User[] }>('/api/admin/users')
    users.value = data.users
  } catch {
    users.value = []
  } finally {
    loading.value = false
  }
}

async function updateRole(userId: string, role: UserRole): Promise<void> {
  try {
    await api.put(`/api/admin/users/${userId}`, { role })
    await fetchUsers()
    uiStore.showNotification('User role updated', 'success')
  } catch {
    uiStore.showNotification('Failed to update role', 'error')
  }
}

async function toggleActive(userId: string, isActive: boolean): Promise<void> {
  try {
    await api.put(`/api/admin/users/${userId}`, { is_active: isActive })
    await fetchUsers()
    uiStore.showNotification(`User ${isActive ? 'activated' : 'deactivated'}`, 'success')
  } catch {
    uiStore.showNotification('Failed to update status', 'error')
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString()
}
</script>

<template>
  <div class="users-page">
    <h1>User Management</h1>

    <div v-if="loading" class="loading">Loading users...</div>
    <table v-else class="users-table">
      <thead>
        <tr>
          <th>Username</th>
          <th>Email</th>
          <th>Role</th>
          <th>Status</th>
          <th>Created</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users" :key="user.id">
          <td>{{ user.username }}</td>
          <td>{{ user.email }}</td>
          <td>
            <select
              :value="user.role"
              @change="updateRole(user.id, ($event.target as HTMLSelectElement).value as UserRole)"
              class="role-select"
            >
              <option value="super_admin">Super Admin</option>
              <option value="admin">Admin</option>
              <option value="editor">Editor</option>
              <option value="viewer">Viewer</option>
            </select>
          </td>
          <td>
            <span :class="['status', user.is_active ? 'active' : 'inactive']">
              {{ user.is_active ? 'Active' : 'Inactive' }}
            </span>
          </td>
          <td>{{ formatDate(user.created_at) }}</td>
          <td>
            <button
              @click="toggleActive(user.id, !user.is_active)"
              :class="['btn-toggle', user.is_active ? 'deactivate' : 'activate']"
            >
              {{ user.is_active ? 'Deactivate' : 'Activate' }}
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.users-page h1 {
  margin-bottom: 1.5rem;
}

.loading {
  text-align: center;
  color: #6b7280;
}

.users-table {
  width: 100%;
  border-collapse: collapse;
  background: #1f2937;
  border-radius: 8px;
  overflow: hidden;
}

.users-table th,
.users-table td {
  padding: 0.75rem;
  text-align: left;
  border-bottom: 1px solid #374151;
}

.users-table th {
  background: #111827;
  color: #9ca3af;
  font-weight: 500;
}

.role-select {
  background: #374151;
  color: white;
  border: none;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}

.status {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
}

.status.active {
  background: #10b981;
  color: white;
}

.status.inactive {
  background: #6b7280;
  color: white;
}

.btn-toggle {
  padding: 0.25rem 0.75rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.75rem;
}

.btn-toggle.deactivate {
  background: #ef4444;
  color: white;
}

.btn-toggle.activate {
  background: #10b981;
  color: white;
}
</style>
