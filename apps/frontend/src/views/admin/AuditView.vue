<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/composables/useApi'
import type { AuditEntry } from '@/types'

const entries = ref<AuditEntry[]>([])
const total = ref(0)
const loading = ref(true)
const offset = ref(0)
const limit = ref(50)

const filters = ref({
  action: '',
  resource_type: '',
})

onMounted(async () => {
  await fetchAuditLogs()
})

async function fetchAuditLogs(): Promise<void> {
  loading.value = true
  try {
    const params = new URLSearchParams()
    params.set('offset', offset.value.toString())
    params.set('limit', limit.value.toString())
    if (filters.value.action) params.set('action', filters.value.action)
    if (filters.value.resource_type) params.set('resource_type', filters.value.resource_type)

    const data = await api.get<{ entries: AuditEntry[]; total: number }>(
      `/api/admin/audit?${params}`
    )
    entries.value = data.entries
    total.value = data.total
  } catch {
    entries.value = []
  } finally {
    loading.value = false
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString()
}

async function applyFilters(): Promise<void> {
  offset.value = 0
  await fetchAuditLogs()
}

async function nextPage(): Promise<void> {
  offset.value += limit.value
  await fetchAuditLogs()
}

async function prevPage(): Promise<void> {
  offset.value = Math.max(0, offset.value - limit.value)
  await fetchAuditLogs()
}
</script>

<template>
  <div class="audit-page">
    <h1>Audit Log</h1>

    <div class="filters">
      <select v-model="filters.action" @change="applyFilters">
        <option value="">All Actions</option>
        <option value="create">Create</option>
        <option value="read">Read</option>
        <option value="update">Update</option>
        <option value="delete">Delete</option>
        <option value="login">Login</option>
        <option value="logout">Logout</option>
        <option value="login_failed">Login Failed</option>
      </select>
      <select v-model="filters.resource_type" @change="applyFilters">
        <option value="">All Resources</option>
        <option value="article">Article</option>
        <option value="user">User</option>
        <option value="tag">Tag</option>
        <option value="auth">Auth</option>
        <option value="system">System</option>
      </select>
    </div>

    <div v-if="loading" class="loading">Loading audit logs...</div>
    <div v-else>
      <table class="audit-table">
        <thead>
          <tr>
            <th>Timestamp</th>
            <th>Action</th>
            <th>Resource</th>
            <th>User ID</th>
            <th>IP Address</th>
            <th>Details</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="entry in entries" :key="entry.id">
            <td>{{ formatDate(entry.created_at) }}</td>
            <td><span :class="['action', entry.action]">{{ entry.action }}</span></td>
            <td>{{ entry.resource_type }}</td>
            <td>{{ entry.user_id || 'System' }}</td>
            <td>{{ entry.ip_address || '-' }}</td>
            <td class="details">{{ entry.details || '-' }}</td>
          </tr>
        </tbody>
      </table>

      <div class="pagination">
        <button @click="prevPage" :disabled="offset === 0">Previous</button>
        <span>Showing {{ offset + 1 }}-{{ Math.min(offset + limit, total) }} of {{ total }}</span>
        <button @click="nextPage" :disabled="offset + limit >= total">Next</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.audit-page h1 {
  margin-bottom: 1.5rem;
}

.filters {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.filters select {
  background: #374151;
  color: white;
  border: none;
  padding: 0.5rem;
  border-radius: 4px;
}

.loading {
  text-align: center;
  color: #6b7280;
}

.audit-table {
  width: 100%;
  border-collapse: collapse;
  background: #1f2937;
  border-radius: 8px;
  overflow: hidden;
  font-size: 0.875rem;
}

.audit-table th,
.audit-table td {
  padding: 0.5rem;
  text-align: left;
  border-bottom: 1px solid #374151;
}

.audit-table th {
  background: #111827;
  color: #9ca3af;
}

.action {
  padding: 0.125rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
}

.action.login { background: #10b981; color: white; }
.action.logout { background: #6b7280; color: white; }
.action.login_failed { background: #ef4444; color: white; }
.action.create { background: #3b82f6; color: white; }
.action.update { background: #f59e0b; color: white; }
.action.delete { background: #ef4444; color: white; }

.details {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #6b7280;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin-top: 1rem;
}

.pagination button {
  background: #374151;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.pagination button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
