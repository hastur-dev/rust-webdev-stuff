<script setup lang="ts">
import type { Article } from '@/types'

defineProps<{
  article: Article
}>()

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString()
}

function stripHtml(html: string): string {
  const tmp = document.createElement('div')
  tmp.innerHTML = html
  return tmp.textContent || tmp.innerText || ''
}

function truncate(text: string, length: number): string {
  const stripped = stripHtml(text)
  if (stripped.length <= length) return stripped
  return stripped.slice(0, length) + '...'
}
</script>

<template>
  <router-link :to="`/articles/${article.id}`" class="article-card">
    <div class="card-header">
      <h3 class="title">{{ article.title }}</h3>
      <span v-if="!article.is_published" class="draft-badge">Draft</span>
    </div>
    <p class="excerpt">{{ truncate(article.content, 150) }}</p>
    <div class="card-footer">
      <span class="date">{{ formatDate(article.created_at) }}</span>
    </div>
  </router-link>
</template>

<style scoped>
.article-card {
  display: block;
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 1rem;
  text-decoration: none;
  color: inherit;
  transition: border-color 0.2s;
}

.article-card:hover {
  border-color: #3b82f6;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.title {
  font-size: 1.125rem;
  font-weight: 600;
  color: white;
  margin: 0;
}

.draft-badge {
  font-size: 0.75rem;
  padding: 0.125rem 0.5rem;
  background: #f59e0b;
  color: black;
  border-radius: 4px;
  flex-shrink: 0;
}

.excerpt {
  color: #9ca3af;
  font-size: 0.875rem;
  line-height: 1.5;
  margin: 0;
}

.card-footer {
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: #6b7280;
}
</style>
