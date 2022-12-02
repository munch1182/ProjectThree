<script setup lang="ts">
import loader from '@monaco-editor/loader';
import { onMounted, ref } from 'vue';
import api from './ApiFroMoanco';
import template from './ApiTemplate';

const editor = ref()
onMounted(async () => {
    const monaco = await loader.init();
    api.register(monaco);

    const e = monaco.editor.create(editor.value, {
        value: template,
        language: api.NAME,
        theme: api.THEME,
        readOnly: false,
        lineNumbers: 'on',
    })
    api.addCommand(monaco, e);
})

</script>

<template>
    <div class="w-full h-full flex">
        <div ref="editor" class="flex-1"></div>
    </div>
</template>

<style scoped>

</style>