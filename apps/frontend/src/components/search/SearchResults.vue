<script setup lang="ts">
import type { Article } from '@/types'
import ArticleCard from '@/components/articles/ArticleCard.vue'

defineProps<{
  results: Article[]
  query: string
  loading?: boolean
}>()
</script>

<template>
  <div class="search-results">
    <div v-if="loading" class="loading">Searching...</div>
    <template v-else-if="query">
      <p class="results-count">
        Found {{ results.length }} result{{ results.length !== 1 ? 's' : '' }} for "{{ query }}"
      </p>
      <div v-if="results.length > 0" class="results-grid">
        <ArticleCard v-for="article in results" :key="article.id" :article="article" />
      </div>
      <div v-else class="no-results">
        No articles found matching your search.
      </div>
    </template>
    <div v-else class="hint">
      Enter a search term to find articles.
    </div>
  </div>
</template>

<style scoped>
.search-results {
  margin-top: 1.5rem;
}

.loading,
.no-results,
.hint {
  text-align: center;
  color: #6b7280;
  padding: 2rem;
}

.results-count {
  color: #9ca3af;
  margin-bottom: 1rem;
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}
</style>
