<script setup lang="ts">
import loader from '@monaco-editor/loader'
import { onMounted, ref } from 'vue';

const editor = ref()
onMounted(async () => {
    const monaco = await loader.init();
    monaco.editor.create(editor.value, {
        value: '',
        language: 'typescript',
        readOnly: false,
        lineNumbers: 'off',
    })
})

</script>

<template>
    <div class="w-full h-full flex">
        <div ref="editor" class="flex-1"></div>
    </div>
</template>

<style scoped>

</style>