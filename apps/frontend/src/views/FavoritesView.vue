<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/composables/useApi'
import type { Favorite } from '@/types'

const favorites = ref<Favorite[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    favorites.value = await api.get<Favorite[]>('/api/favorites')
  } catch {
    favorites.value = []
  } finally {
    loading.value = false
  }
})

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString()
}
</script>

<template>
  <div class="favorites-page">
    <h1>My Favorites</h1>

    <div v-if="loading" class="loading">Loading favorites...</div>
    <div v-else-if="favorites.length === 0" class="empty">
      You haven't favorited any articles yet.
    </div>
    <ul v-else class="favorites-list">
      <li v-for="fav in favorites" :key="fav.article_id" class="favorite-item">
        <router-link :to="`/articles/${fav.article_id}`" class="favorite-link">
          <span class="title">{{ fav.article_title }}</span>
          <span class="meta">Favorited on {{ formatDate(fav.favorited_at) }}</span>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.favorites-page h1 {
  margin-bottom: 1.5rem;
}

.loading,
.empty {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
}

.favorites-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.favorite-item {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.favorite-link {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  text-decoration: none;
  color: inherit;
}

.favorite-link:hover {
  background: #374151;
}

.title {
  font-weight: 500;
}

.meta {
  font-size: 0.875rem;
  color: #6b7280;
}
</style>
