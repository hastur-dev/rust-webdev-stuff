<script setup lang="ts">
import { useSearch } from '@/composables/useSearch'
import { useAuth } from '@/composables/useAuth'
import SearchBar from '@/components/search/SearchBar.vue'
import SearchResults from '@/components/search/SearchResults.vue'

const { results, query, loading, error, search, searchPublic } = useSearch()
const { isAuthenticated } = useAuth()

function handleSearch(q: string): void {
  if (isAuthenticated.value) {
    search(q)
  } else {
    searchPublic(q)
  }
}
</script>

<template>
  <div class="search-page">
    <h1>Search</h1>
    <SearchBar v-model="query" @search="handleSearch" />
    <div v-if="error" class="error">{{ error }}</div>
    <SearchResults :results="results" :query="query" :loading="loading" />
  </div>
</template>

<style scoped>
.search-page h1 {
  margin-bottom: 1.5rem;
}

.error {
  color: #ef4444;
  margin-top: 1rem;
}
</style>
