<script setup lang="ts">
import { ref } from 'vue';
import { onBeforeRouteUpdate } from 'vue-router'
import WeightWindowMenu from './WeightWindowMenu.vue';

const title = ref('首页')
onBeforeRouteUpdate(async (to, _) => {
    if (to.meta.title) {
        title.value = to.meta.title as string
    } else {
        title.value = to.fullPath
    }
})
</script>

<template>
    <div class="flex flex-col flex-grow">
        <div class="flex h-[var(--height-title-bar)]" data-tauri-drag-region>
            <span class="h-max mx-[var(--space-padding)] my-auto font-bold text-[1.5rem]">{{ title }}</span>
            <div class="flex flex-grow" data-tauri-drag-region></div>
            <WeightWindowMenu />
        </div>
        <div class="line-w"></div>
        <slot></slot>
    </div>
</template>

<style scoped>

</style>