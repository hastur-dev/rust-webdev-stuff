import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Article, CreateArticle, UpdateArticle, Tag } from '@/types'
import { api } from '@/composables/useApi'

export const useArticlesStore = defineStore('articles', () => {
  const articles = ref<Article[]>([])
  const currentArticle = ref<Article | null>(null)
  const tags = ref<Tag[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchArticles(offset = 0, limit = 20): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const data = await api.get<Article[]>(`/api/articles?offset=${offset}&limit=${limit}`)
      articles.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch articles'
    } finally {
      loading.value = false
    }
  }

  async function fetchPublishedArticles(offset = 0, limit = 20): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const data = await api.get<Article[]>(`/api/articles/published?offset=${offset}&limit=${limit}`)
      articles.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch articles'
    } finally {
      loading.value = false
    }
  }

  async function fetchArticle(id: string): Promise<Article | null> {
    loading.value = true
    error.value = null
    try {
      const data = await api.get<{ article: Article }>(`/api/articles/${id}`)
      currentArticle.value = data.article
      return data.article
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch article'
      return null
    } finally {
      loading.value = false
    }
  }

  async function createArticle(article: CreateArticle): Promise<Article | null> {
    loading.value = true
    error.value = null
    try {
      const created = await api.post<Article>('/api/articles', article)
      articles.value.unshift(created)
      return created
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create article'
      return null
    } finally {
      loading.value = false
    }
  }

  async function updateArticle(id: string, update: UpdateArticle): Promise<Article | null> {
    loading.value = true
    error.value = null
    try {
      const updated = await api.put<Article>(`/api/articles/${id}`, update)
      const index = articles.value.findIndex((a) => a.id === id)
      if (index !== -1) {
        articles.value[index] = updated
      }
      currentArticle.value = updated
      return updated
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update article'
      return null
    } finally {
      loading.value = false
    }
  }

  async function deleteArticle(id: string): Promise<boolean> {
    loading.value = true
    error.value = null
    try {
      await api.delete(`/api/articles/${id}`)
      articles.value = articles.value.filter((a) => a.id !== id)
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete article'
      return false
    } finally {
      loading.value = false
    }
  }

  async function fetchTags(): Promise<void> {
    try {
      tags.value = await api.get<Tag[]>('/api/tags')
    } catch {
      tags.value = []
    }
  }

  return {
    articles,
    currentArticle,
    tags,
    loading,
    error,
    fetchArticles,
    fetchPublishedArticles,
    fetchArticle,
    createArticle,
    updateArticle,
    deleteArticle,
    fetchTags,
  }
})
