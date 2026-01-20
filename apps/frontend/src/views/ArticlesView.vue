<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useArticlesStore } from '@/stores/articles'
import { useAuth } from '@/composables/useAuth'
import ArticleList from '@/components/articles/ArticleList.vue'

const articlesStore = useArticlesStore()
const { isAuthenticated, canEdit } = useAuth()

const offset = ref(0)
const limit = ref(20)

onMounted(async () => {
  if (isAuthenticated.value) {
    await articlesStore.fetchArticles(offset.value, limit.value)
  } else {
    await articlesStore.fetchPublishedArticles(offset.value, limit.value)
  }
})

async function loadMore(): Promise<void> {
  offset.value += limit.value
  if (isAuthenticated.value) {
    await articlesStore.fetchArticles(offset.value, limit.value)
  } else {
    await articlesStore.fetchPublishedArticles(offset.value, limit.value)
  }
}
</script>

<template>
  <div class="articles-page">
    <header class="page-header">
      <h1>Articles</h1>
      <router-link v-if="canEdit" to="/articles/new" class="btn-create">
        + New Article
      </router-link>
    </header>

    <ArticleList :articles="articlesStore.articles" :loading="articlesStore.loading" />

    <div v-if="articlesStore.articles.length >= limit" class="load-more">
      <button @click="loadMore" :disabled="articlesStore.loading">
        Load More
      </button>
    </div>
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.page-header h1 {
  margin: 0;
}

.btn-create {
  background: #3b82f6;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  text-decoration: none;
}

.btn-create:hover {
  background: #2563eb;
}

.load-more {
  text-align: center;
  margin-top: 2rem;
}

.load-more button {
  padding: 0.75rem 2rem;
  background: #374151;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.load-more button:hover:not(:disabled) {
  background: #4b5563;
}

.load-more button:disabled {
  opacity: 0.5;
}
</style>
