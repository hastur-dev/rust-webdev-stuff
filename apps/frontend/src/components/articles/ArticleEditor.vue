<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import suneditor from 'suneditor'
import 'suneditor/dist/css/suneditor.min.css'
import plugins from 'suneditor/src/plugins'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const editorRef = ref<HTMLTextAreaElement | null>(null)
let editor: ReturnType<typeof suneditor.create> | null = null

onMounted(() => {
  if (!editorRef.value) return

  editor = suneditor.create(editorRef.value, {
    plugins: plugins,
    buttonList: [
      ['undo', 'redo'],
      ['font', 'fontSize', 'formatBlock'],
      ['bold', 'underline', 'italic', 'strike'],
      ['fontColor', 'hiliteColor'],
      ['removeFormat'],
      ['outdent', 'indent'],
      ['align', 'list', 'lineHeight'],
      ['table', 'link', 'image'],
      ['fullScreen', 'showBlocks', 'codeView'],
    ],
    height: '400px',
    placeholder: props.placeholder || 'Start writing your article...',
    charCounter: true,
    charCounterLabel: 'Characters:',
  })

  // Set initial content
  if (props.modelValue) {
    editor.setContents(props.modelValue)
  }

  // Listen for changes
  editor.onChange = (contents: string) => {
    emit('update:modelValue', contents)
  }
})

// Watch for external changes to modelValue
watch(
  () => props.modelValue,
  (newValue) => {
    if (editor && editor.getContents() !== newValue) {
      editor.setContents(newValue)
    }
  }
)

onBeforeUnmount(() => {
  if (editor) {
    editor.destroy()
    editor = null
  }
})

// Expose method to get content
function getContent(): string {
  return editor?.getContents() || ''
}

defineExpose({ getContent })
</script>

<template>
  <div class="article-editor">
    <textarea ref="editorRef"></textarea>
  </div>
</template>

<style scoped>
.article-editor {
  width: 100%;
}

.article-editor :deep(.sun-editor) {
  border: 1px solid #374151;
  border-radius: 4px;
}

.article-editor :deep(.sun-editor .se-toolbar) {
  background: #1f2937;
  border-bottom: 1px solid #374151;
}

.article-editor :deep(.sun-editor .se-btn) {
  color: #9ca3af;
}

.article-editor :deep(.sun-editor .se-btn:hover) {
  background: #374151;
  color: white;
}

.article-editor :deep(.sun-editor .se-wrapper) {
  background: #111827;
}

.article-editor :deep(.sun-editor .se-wrapper-inner) {
  color: white;
}

.article-editor :deep(.sun-editor-editable) {
  background: #111827;
  color: white;
}
</style>
