import { ref } from 'vue'
import type { Article, SearchResult } from '@/types'
import { api } from './useApi'

export function useSearch() {
  const results = ref<Article[]>([])
  const query = ref('')
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function search(searchQuery: string): Promise<void> {
    if (!searchQuery.trim()) {
      results.value = []
      return
    }

    query.value = searchQuery
    loading.value = true
    error.value = null

    try {
      const data = await api.get<SearchResult>(
        `/api/search?q=${encodeURIComponent(searchQuery)}`
      )
      results.value = data.results
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Search failed'
      results.value = []
    } finally {
      loading.value = false
    }
  }

  async function searchPublic(searchQuery: string): Promise<void> {
    if (!searchQuery.trim()) {
      results.value = []
      return
    }

    query.value = searchQuery
    loading.value = true
    error.value = null

    try {
      const data = await api.get<SearchResult>(
        `/api/search/public?q=${encodeURIComponent(searchQuery)}`
      )
      results.value = data.results
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Search failed'
      results.value = []
    } finally {
      loading.value = false
    }
  }

  function clearResults(): void {
    results.value = []
    query.value = ''
    error.value = null
  }

  return {
    results,
    query,
    loading,
    error,
    search,
    searchPublic,
    clearResults,
  }
}
