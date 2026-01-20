<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuth } from '@/composables/useAuth'
import { useUiStore } from '@/stores/ui'

const router = useRouter()
const route = useRoute()
const { login } = useAuth()
const uiStore = useUiStore()

const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleSubmit(): Promise<void> {
  error.value = ''
  loading.value = true

  try {
    const success = await login({ username: username.value, password: password.value })
    if (success) {
      uiStore.showNotification('Login successful', 'success')
      const redirect = route.query.redirect as string
      router.push(redirect || '/')
    } else {
      error.value = 'Invalid username or password'
    }
  } catch (e) {
    error.value = 'Login failed. Please try again.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page">
    <form class="login-form" @submit.prevent="handleSubmit">
      <h1>Login</h1>

      <div v-if="error" class="error">{{ error }}</div>

      <div class="form-group">
        <label for="username">Username</label>
        <input
          id="username"
          v-model="username"
          type="text"
          required
          autocomplete="username"
        />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input
          id="password"
          v-model="password"
          type="password"
          required
          autocomplete="current-password"
        />
      </div>

      <button type="submit" class="submit-btn" :disabled="loading">
        {{ loading ? 'Logging in...' : 'Login' }}
      </button>

      <p class="demo-hint">
        Demo accounts: superadmin/SuperAdmin123!, admin1/Admin123!, editor1/Editor123!, viewer1/Viewer123!
      </p>
    </form>
  </div>
</template>

<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60vh;
}

.login-form {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 2rem;
  width: 100%;
  max-width: 400px;
}

.login-form h1 {
  margin: 0 0 1.5rem;
  text-align: center;
}

.error {
  background: #ef4444;
  color: white;
  padding: 0.75rem;
  border-radius: 4px;
  margin-bottom: 1rem;
  text-align: center;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.25rem;
  color: #9ca3af;
}

.form-group input {
  width: 100%;
  padding: 0.75rem;
  background: #111827;
  border: 1px solid #374151;
  border-radius: 4px;
  color: white;
  font-size: 1rem;
}

.form-group input:focus {
  outline: none;
  border-color: #3b82f6;
}

.submit-btn {
  width: 100%;
  padding: 0.75rem;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  cursor: pointer;
}

.submit-btn:hover:not(:disabled) {
  background: #2563eb;
}

.submit-btn:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.demo-hint {
  margin-top: 1rem;
  font-size: 0.75rem;
  color: #6b7280;
  text-align: center;
}
</style>
