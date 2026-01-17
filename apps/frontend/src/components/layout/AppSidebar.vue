<script setup lang="ts">
import { useAuth } from '@/composables/useAuth'

const { isAuthenticated, isAdmin, canEdit } = useAuth()
</script>

<template>
  <aside class="sidebar">
    <nav class="sidebar-nav">
      <div class="nav-section">
        <h3>Navigation</h3>
        <router-link to="/">Home</router-link>
        <router-link to="/articles">All Articles</router-link>
        <router-link to="/search">Search</router-link>
      </div>

      <div v-if="isAuthenticated" class="nav-section">
        <h3>My Content</h3>
        <router-link to="/favorites">Favorites</router-link>
        <router-link v-if="canEdit" to="/articles/new">New Article</router-link>
      </div>

      <div v-if="isAdmin" class="nav-section">
        <h3>Administration</h3>
        <router-link to="/admin">Dashboard</router-link>
        <router-link to="/admin/users">Users</router-link>
        <router-link to="/admin/audit">Audit Log</router-link>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 220px;
  background: #111827;
  border-right: 1px solid #374151;
  padding: 1rem;
  flex-shrink: 0;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.nav-section h3 {
  color: #6b7280;
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 0.5rem;
}

.nav-section a {
  display: block;
  color: #9ca3af;
  text-decoration: none;
  padding: 0.5rem;
  border-radius: 4px;
}

.nav-section a:hover {
  background: #1f2937;
  color: white;
}

.nav-section a.router-link-active {
  background: #3b82f6;
  color: white;
}
</style>
