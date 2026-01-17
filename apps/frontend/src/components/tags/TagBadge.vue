<script setup lang="ts">
import type { Tag } from '@/types'

defineProps<{
  tag: Tag
  selected?: boolean
  clickable?: boolean
}>()
</script>

<template>
  <span :class="['tag-badge', { selected, clickable }]">
    {{ tag.name }}
  </span>
</template>

<style scoped>
.tag-badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  background: #374151;
  color: #9ca3af;
  border-radius: 9999px;
  font-size: 0.875rem;
}

.tag-badge.clickable {
  cursor: pointer;
}

.tag-badge.clickable:hover {
  background: #4b5563;
}

.tag-badge.selected {
  background: #3b82f6;
  color: white;
}
</style>
