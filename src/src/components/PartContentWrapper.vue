<script setup lang="ts">
import { ref } from '@vue/reactivity';
import WeightWindowMenu from './WeightWindowMenu.vue';
import { onBeforeRouteUpdate } from 'vue-router'

const title = ref('首页')
onBeforeRouteUpdate(async (to, from) => {
    if (to.meta.title) {
        title.value = to.meta.title as string
    }
})

</script>

<template>
    <div class="flex flex-col w-full">
        <div class="flex w-full justify-end " data-tauri-drag-region>
            <div class="flex content-center w-full h-[var(--height-title-bar)] px-4" data-tauri-drag-region>
                <h1 v-if="title.length > 0"
                    class="h-[var(--height-title-bar)] leading-[var(--height-title-bar)] text-center text-2xl">
                    {{ title }}
                </h1>
            </div>
            <WeightWindowMenu />
        </div>
        <div class="line-h"></div>
        <div class="h-full">
            <slot></slot>
        </div>
    </div>
</template>