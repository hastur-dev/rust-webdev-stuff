<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useArticlesStore } from '@/stores/articles'
import { useAuth } from '@/composables/useAuth'
import { useUiStore } from '@/stores/ui'
import { api } from '@/composables/useApi'
import ArticleView from '@/components/articles/ArticleView.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import type { Tag } from '@/types'

const route = useRoute()
const router = useRouter()
const articlesStore = useArticlesStore()
const { canEditArticles, canDeleteArticles, user } = useAuth()
const uiStore = useUiStore()

const tags = ref<Tag[]>([])
const showDeleteDialog = ref(false)
const isFavorited = ref(false)

const articleId = route.params.id as string

onMounted(async () => {
  await articlesStore.fetchArticle(articleId)

  // Fetch favorite status
  try {
    const status = await api.get<{ is_favorited: boolean }>(`/api/favorites/${articleId}/status`)
    isFavorited.value = status.is_favorited
  } catch {
    // Not logged in or error
  }
})

async function toggleFavorite(): Promise<void> {
  try {
    if (isFavorited.value) {
      await api.delete(`/api/favorites/${articleId}`)
      isFavorited.value = false
      uiStore.showNotification('Removed from favorites', 'info')
    } else {
      await api.post(`/api/favorites/${articleId}`, {})
      isFavorited.value = true
      uiStore.showNotification('Added to favorites', 'success')
    }
  } catch (e) {
    uiStore.showNotification('Failed to update favorite', 'error')
  }
}

async function handleDelete(): Promise<void> {
  const success = await articlesStore.deleteArticle(articleId)
  if (success) {
    uiStore.showNotification('Article deleted', 'success')
    router.push('/articles')
  } else {
    uiStore.showNotification('Failed to delete article', 'error')
  }
  showDeleteDialog.value = false
}

function canEdit(): boolean {
  if (!articlesStore.currentArticle || !user.value) return false
  return canEditArticles() && (
    articlesStore.currentArticle.author_id === user.value.id || canDeleteArticles()
  )
}
</script>

<template>
  <div class="article-detail">
    <div v-if="articlesStore.loading" class="loading">Loading article...</div>
    <div v-else-if="articlesStore.error" class="error">{{ articlesStore.error }}</div>
    <template v-else-if="articlesStore.currentArticle">
      <div class="actions-bar">
        <button @click="router.back()" class="btn-back">← Back</button>
        <div class="actions">
          <button @click="toggleFavorite" class="btn-favorite">
            {{ isFavorited ? '★ Favorited' : '☆ Favorite' }}
          </button>
          <router-link
            v-if="canEdit()"
            :to="`/articles/${articleId}/edit`"
            class="btn-edit"
          >
            Edit
          </router-link>
          <button
            v-if="canDeleteArticles()"
            @click="showDeleteDialog = true"
            class="btn-delete"
          >
            Delete
          </button>
        </div>
      </div>

      <ArticleView :article="articlesStore.currentArticle" :tags="tags" />
    </template>

    <ConfirmDialog
      v-if="showDeleteDialog"
      title="Delete Article"
      message="Are you sure you want to delete this article? This action cannot be undone."
      confirm-text="Delete"
      @confirm="handleDelete"
      @cancel="showDeleteDialog = false"
    />
  </div>
</template>

<style scoped>
.loading,
.error {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
}

.error {
  color: #ef4444;
}

.actions-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.actions {
  display: flex;
  gap: 0.5rem;
}

.btn-back,
.btn-favorite,
.btn-edit,
.btn-delete {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  text-decoration: none;
  font-size: 0.875rem;
}

.btn-back {
  background: #374151;
  color: white;
}

.btn-favorite {
  background: #374151;
  color: #fbbf24;
}

.btn-edit {
  background: #3b82f6;
  color: white;
}

.btn-delete {
  background: #ef4444;
  color: white;
}
</style>
