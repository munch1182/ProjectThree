<script setup lang="ts">
import WindowMenu from './WindowMenu.vue';
</script>

<template>
    <div id="contentwrapper" class="w-full">
        <div class="flex flex-row-reverse w-full" data-tauri-drag-region>
            <WindowMenu />
        </div>
        <div>
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
#contentwrapper {
    background-color: #f5f5f5;
}
</style>