<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  search: [query: string]
}>()

const inputRef = ref<HTMLInputElement | null>(null)

function handleInput(event: Event): void {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', value)
}

function handleSubmit(): void {
  emit('search', props.modelValue)
}

function focus(): void {
  inputRef.value?.focus()
}

defineExpose({ focus })
</script>

<template>
  <form class="search-bar" @submit.prevent="handleSubmit">
    <input
      ref="inputRef"
      type="search"
      :value="modelValue"
      :placeholder="placeholder || 'Search articles...'"
      @input="handleInput"
      class="search-input"
    />
    <button type="submit" class="search-btn">Search</button>
  </form>
</template>

<style scoped>
.search-bar {
  display: flex;
  gap: 0.5rem;
  width: 100%;
  max-width: 600px;
}

.search-input {
  flex: 1;
  padding: 0.75rem 1rem;
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 4px;
  color: white;
  font-size: 1rem;
}

.search-input:focus {
  outline: none;
  border-color: #3b82f6;
}

.search-btn {
  padding: 0.75rem 1.5rem;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
}

.search-btn:hover {
  background: #2563eb;
}
</style>
