<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuth } from '@/composables/useAuth'
import { useUiStore } from '@/stores/ui'

const router = useRouter()
const { user, isAuthenticated, isAdmin, logout } = useAuth()
const uiStore = useUiStore()

async function handleLogout(): Promise<void> {
  await logout()
  router.push('/login')
}
</script>

<template>
  <header class="header">
    <div class="header-left">
      <button class="menu-btn" @click="uiStore.toggleSidebar">☰</button>
      <router-link to="/" class="logo">Knowledge Vault</router-link>
    </div>

    <nav class="nav">
      <router-link to="/articles">Articles</router-link>
      <router-link to="/search">Search</router-link>
      <template v-if="isAuthenticated">
        <router-link to="/favorites">Favorites</router-link>
        <router-link v-if="isAdmin" to="/admin">Admin</router-link>
      </template>
    </nav>

    <div class="header-right">
      <template v-if="isAuthenticated && user">
        <span class="username">{{ user.username }}</span>
        <span class="role-badge">{{ user.role }}</span>
        <button @click="handleLogout" class="logout-btn">Logout</button>
      </template>
      <template v-else>
        <router-link to="/login" class="login-btn">Login</router-link>
      </template>
    </div>
  </header>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: #1f2937;
  color: white;
  border-bottom: 1px solid #374151;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.menu-btn {
  background: none;
  border: none;
  color: white;
  font-size: 1.25rem;
  cursor: pointer;
  padding: 0.25rem;
}

.logo {
  font-size: 1.25rem;
  font-weight: bold;
  color: white;
  text-decoration: none;
}

.nav {
  display: flex;
  gap: 1rem;
}

.nav a {
  color: #9ca3af;
  text-decoration: none;
  padding: 0.5rem;
}

.nav a:hover,
.nav a.router-link-active {
  color: white;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.username {
  font-weight: 500;
}

.role-badge {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  background: #374151;
  border-radius: 4px;
}

.logout-btn,
.login-btn {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  text-decoration: none;
}

.logout-btn:hover,
.login-btn:hover {
  background: #2563eb;
}
</style>
