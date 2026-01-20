<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useArticlesStore } from '@/stores/articles'
import { useUiStore } from '@/stores/ui'
import ArticleEditor from '@/components/articles/ArticleEditor.vue'

const route = useRoute()
const router = useRouter()
const articlesStore = useArticlesStore()
const uiStore = useUiStore()

const title = ref('')
const content = ref('')
const isPublished = ref(false)
const loading = ref(false)

const articleId = route.params.id as string

onMounted(async () => {
  const article = await articlesStore.fetchArticle(articleId)
  if (article) {
    title.value = article.title
    content.value = article.content
    isPublished.value = article.is_published
  }
})

async function handleSubmit(): Promise<void> {
  if (!title.value.trim()) {
    uiStore.showNotification('Title is required', 'error')
    return
  }

  loading.value = true

  const article = await articlesStore.updateArticle(articleId, {
    title: title.value,
    content: content.value,
    is_published: isPublished.value,
  })

  loading.value = false

  if (article) {
    uiStore.showNotification('Article updated successfully', 'success')
    router.push(`/articles/${article.id}`)
  } else {
    uiStore.showNotification(articlesStore.error || 'Failed to update article', 'error')
  }
}
</script>

<template>
  <div class="edit-article">
    <h1>Edit Article</h1>

    <div v-if="articlesStore.loading" class="loading">Loading article...</div>
    <form v-else @submit.prevent="handleSubmit" class="article-form">
      <div class="form-group">
        <label for="title">Title</label>
        <input id="title" v-model="title" type="text" required />
      </div>

      <div class="form-group">
        <label>Content</label>
        <ArticleEditor v-model="content" />
      </div>

      <div class="form-group checkbox">
        <label>
          <input type="checkbox" v-model="isPublished" />
          Published
        </label>
      </div>

      <div class="form-actions">
        <button type="button" @click="router.back()" class="btn-cancel">Cancel</button>
        <button type="submit" :disabled="loading" class="btn-submit">
          {{ loading ? 'Saving...' : 'Save Changes' }}
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.edit-article {
  max-width: 900px;
  margin: 0 auto;
}

.edit-article h1 {
  margin-bottom: 1.5rem;
}

.loading {
  text-align: center;
  color: #6b7280;
}

.article-form {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: #9ca3af;
}

.form-group input[type="text"] {
  width: 100%;
  padding: 0.75rem;
  background: #111827;
  border: 1px solid #374151;
  border-radius: 4px;
  color: white;
  font-size: 1rem;
}

.form-group.checkbox label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.btn-cancel,
.btn-submit {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-cancel {
  background: #374151;
  color: white;
}

.btn-submit {
  background: #3b82f6;
  color: white;
}
</style>
