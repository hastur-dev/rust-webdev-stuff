<script setup lang="ts">
import type { Article, Tag } from '@/types'

defineProps<{
  article: Article
  tags?: Tag[]
}>()

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString()
}
</script>

<template>
  <article class="article-view">
    <header class="article-header">
      <h1>{{ article.title }}</h1>
      <div class="meta">
        <span>Created: {{ formatDate(article.created_at) }}</span>
        <span v-if="article.updated_at !== article.created_at">
          Updated: {{ formatDate(article.updated_at) }}
        </span>
        <span v-if="!article.is_published" class="draft-badge">Draft</span>
      </div>
      <div v-if="tags && tags.length > 0" class="tags">
        <span v-for="tag in tags" :key="tag.id" class="tag">{{ tag.name }}</span>
      </div>
    </header>
    <div class="content" v-html="article.content"></div>
  </article>
</template>

<style scoped>
.article-view {
  max-width: 800px;
  margin: 0 auto;
}

.article-header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid #374151;
}

.article-header h1 {
  font-size: 2rem;
  font-weight: bold;
  margin: 0 0 0.5rem;
}

.meta {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: #6b7280;
  flex-wrap: wrap;
}

.draft-badge {
  background: #f59e0b;
  color: black;
  padding: 0.125rem 0.5rem;
  border-radius: 4px;
}

.tags {
  display: flex;
  gap: 0.5rem;
  margin-top: 0.75rem;
  flex-wrap: wrap;
}

.tag {
  background: #374151;
  color: #9ca3af;
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
}

.content {
  line-height: 1.75;
}

.content :deep(h1),
.content :deep(h2),
.content :deep(h3) {
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
}

.content :deep(p) {
  margin-bottom: 1rem;
}

.content :deep(pre) {
  background: #1f2937;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.content :deep(code) {
  background: #374151;
  padding: 0.125rem 0.25rem;
  border-radius: 2px;
}

.content :deep(blockquote) {
  border-left: 4px solid #3b82f6;
  padding-left: 1rem;
  margin: 1rem 0;
  color: #9ca3af;
}
</style>
