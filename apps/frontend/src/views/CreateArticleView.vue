<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useArticlesStore } from '@/stores/articles'
import { useUiStore } from '@/stores/ui'
import ArticleEditor from '@/components/articles/ArticleEditor.vue'

const router = useRouter()
const articlesStore = useArticlesStore()
const uiStore = useUiStore()

const title = ref('')
const content = ref('')
const loading = ref(false)

async function handleSubmit(): Promise<void> {
  if (!title.value.trim()) {
    uiStore.showNotification('Title is required', 'error')
    return
  }

  loading.value = true

  const article = await articlesStore.createArticle({
    title: title.value,
    content: content.value,
    tags: [],
  })

  loading.value = false

  if (article) {
    uiStore.showNotification('Article created successfully', 'success')
    router.push(`/articles/${article.id}`)
  } else {
    uiStore.showNotification(articlesStore.error || 'Failed to create article', 'error')
  }
}
</script>

<template>
  <div class="create-article">
    <h1>Create Article</h1>

    <form @submit.prevent="handleSubmit" class="article-form">
      <div class="form-group">
        <label for="title">Title</label>
        <input
          id="title"
          v-model="title"
          type="text"
          required
          placeholder="Enter article title"
        />
      </div>

      <div class="form-group">
        <label>Content</label>
        <ArticleEditor v-model="content" />
      </div>

      <div class="form-actions">
        <button type="button" @click="router.back()" class="btn-cancel">Cancel</button>
        <button type="submit" :disabled="loading" class="btn-submit">
          {{ loading ? 'Creating...' : 'Create Article' }}
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.create-article {
  max-width: 900px;
  margin: 0 auto;
}

.create-article h1 {
  margin-bottom: 1.5rem;
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
  font-size: 1rem;
}

.btn-cancel {
  background: #374151;
  color: white;
}

.btn-submit {
  background: #3b82f6;
  color: white;
}

.btn-submit:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}
</style>
