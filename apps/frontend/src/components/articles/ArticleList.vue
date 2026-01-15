<script setup lang="ts">
import type { Article } from '@/types'
import ArticleCard from './ArticleCard.vue'

defineProps<{
  articles: Article[]
  loading?: boolean
}>()
</script>

<template>
  <div class="article-list">
    <div v-if="loading" class="loading">Loading articles...</div>
    <div v-else-if="articles.length === 0" class="empty">No articles found</div>
    <div v-else class="grid">
      <ArticleCard v-for="article in articles" :key="article.id" :article="article" />
    </div>
  </div>
</template>

<style scoped>
.article-list {
  width: 100%;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.loading,
.empty {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
}
</style>
