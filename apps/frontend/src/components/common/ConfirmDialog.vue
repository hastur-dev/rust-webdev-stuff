<script setup lang="ts">
defineProps<{
  title: string
  message: string
  confirmText?: string
  cancelText?: string
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('cancel')">
    <div class="dialog">
      <h2 class="dialog-title">{{ title }}</h2>
      <p class="dialog-message">{{ message }}</p>
      <div class="dialog-actions">
        <button class="btn btn-secondary" @click="emit('cancel')">
          {{ cancelText || 'Cancel' }}
        </button>
        <button class="btn btn-danger" @click="emit('confirm')">
          {{ confirmText || 'Confirm' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 8px;
  padding: 1.5rem;
  max-width: 400px;
  width: 90%;
}

.dialog-title {
  margin: 0 0 0.5rem;
  font-size: 1.25rem;
}

.dialog-message {
  color: #9ca3af;
  margin: 0 0 1.5rem;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

.btn {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 0.875rem;
}

.btn-secondary {
  background: #374151;
  color: white;
}

.btn-danger {
  background: #ef4444;
  color: white;
}

.btn:hover {
  opacity: 0.9;
}
</style>
