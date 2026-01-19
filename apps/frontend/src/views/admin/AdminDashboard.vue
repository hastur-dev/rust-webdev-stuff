<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/composables/useApi'

const stats = ref({
  total_entries: 0,
  login_count: 0,
  failed_login_count: 0,
})

const userCount = ref(0)
const loading = ref(true)

onMounted(async () => {
  try {
    const [auditStats, users] = await Promise.all([
      api.get<typeof stats.value>('/api/admin/audit/stats'),
      api.get<{ total: number }>('/api/admin/users?limit=1'),
    ])
    stats.value = auditStats
    userCount.value = users.total
  } catch {
    // Error handling
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="admin-dashboard">
    <h1>Admin Dashboard</h1>

    <div v-if="loading" class="loading">Loading...</div>
    <div v-else class="stats-grid">
      <div class="stat-card">
        <h3>Total Users</h3>
        <p class="stat-value">{{ userCount }}</p>
      </div>
      <div class="stat-card">
        <h3>Audit Entries</h3>
        <p class="stat-value">{{ stats.total_entries }}</p>
      </div>
      <div class="stat-card">
        <h3>Successful Logins</h3>
        <p class="stat-value">{{ stats.login_count }}</p>
      </div>
      <div class="stat-card warning">
        <h3>Failed Logins</h3>
        <p class="stat-value">{{ stats.failed_login_count }}</p>
      </div>
    </div>

    <div class="quick-links">
      <h2>Quick Links</h2>
      <nav>
        <router-link to="/admin/users">Manage Users</router-link>
        <router-link to="/admin/audit">View Audit Log</router-link>
      </nav>
    </div>
  </div>
</template>

<style scoped>
.admin-dashboard h1 {
  margin-bottom: 1.5rem;
}

.loading {
  text-align: center;
  color: #6b7280;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 1.5rem;
}

.stat-card h3 {
  font-size: 0.875rem;
  color: #9ca3af;
  margin: 0 0 0.5rem;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  margin: 0;
}

.stat-card.warning .stat-value {
  color: #f59e0b;
}

.quick-links {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 1.5rem;
}

.quick-links h2 {
  margin: 0 0 1rem;
  font-size: 1.25rem;
}

.quick-links nav {
  display: flex;
  gap: 1rem;
}

.quick-links a {
  color: #3b82f6;
  text-decoration: none;
}

.quick-links a:hover {
  text-decoration: underline;
}
</style>
