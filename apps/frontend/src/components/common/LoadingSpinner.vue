<script setup lang="ts">
defineProps<{
  size?: 'sm' | 'md' | 'lg'
}>()
</script>

<template>
  <div :class="['spinner', size || 'md']">
    <div class="spinner-inner"></div>
  </div>
</template>

<style scoped>
.spinner {
  display: inline-block;
}

.spinner.sm { width: 16px; height: 16px; }
.spinner.md { width: 32px; height: 32px; }
.spinner.lg { width: 48px; height: 48px; }

.spinner-inner {
  width: 100%;
  height: 100%;
  border: 3px solid #374151;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
