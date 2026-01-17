<script setup lang="ts">
import type { Tag } from '@/types'
import TagBadge from './TagBadge.vue'

defineProps<{
  tags: Tag[]
  selectable?: boolean
  selected?: string[]
}>()

const emit = defineEmits<{
  select: [tagId: string]
}>()
</script>

<template>
  <div class="tag-list">
    <TagBadge
      v-for="tag in tags"
      :key="tag.id"
      :tag="tag"
      :selected="selected?.includes(tag.id)"
      :clickable="selectable"
      @click="selectable && emit('select', tag.id)"
    />
  </div>
</template>

<style scoped>
.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}
</style>
