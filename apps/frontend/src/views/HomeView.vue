<script setup lang="ts">
import { onMounted } from 'vue'
import { useArticlesStore } from '@/stores/articles'
import { useAuth } from '@/composables/useAuth'
import ArticleList from '@/components/articles/ArticleList.vue'

const articlesStore = useArticlesStore()
const { isAuthenticated, canEdit } = useAuth()

onMounted(async () => {
  if (isAuthenticated.value) {
    await articlesStore.fetchArticles(0, 6)
  } else {
    await articlesStore.fetchPublishedArticles(0, 6)
  }
})
</script>

<template>
  <div class="home">
    <section class="hero">
      <h1>Knowledge Vault</h1>
      <p>Your secure knowledge management platform</p>
      <div class="hero-actions">
        <router-link to="/articles" class="btn btn-primary">Browse Articles</router-link>
        <router-link v-if="canEdit" to="/articles/new" class="btn btn-secondary">
          Create Article
        </router-link>
      </div>
    </section>

    <section class="recent-articles">
      <h2>Recent Articles</h2>
      <ArticleList :articles="articlesStore.articles" :loading="articlesStore.loading" />
      <div class="view-all">
        <router-link to="/articles">View all articles →</router-link>
      </div>
    </section>
  </div>
</template>

<style scoped>
.home {
  max-width: 1000px;
  margin: 0 auto;
}

.hero {
  text-align: center;
  padding: 3rem 1rem;
  margin-bottom: 2rem;
}

.hero h1 {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
}

.hero p {
  color: #9ca3af;
  font-size: 1.25rem;
  margin-bottom: 1.5rem;
}

.hero-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 4px;
  text-decoration: none;
  font-weight: 500;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-secondary {
  background: #374151;
  color: white;
}

.recent-articles h2 {
  margin-bottom: 1rem;
}

.view-all {
  text-align: center;
  margin-top: 1.5rem;
}

.view-all a {
  color: #3b82f6;
  text-decoration: none;
}

.view-all a:hover {
  text-decoration: underline;
}
</style>
