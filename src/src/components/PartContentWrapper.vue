<script setup lang="ts">
import { computed } from '@vue/reactivity';
import WeightWindowMenu from './WeightWindowMenu.vue';
const title = defineProps({
    title: String
})
const hadTitle = computed(() => title.title)
</script>

<template>
    <div class="flex flex-col w-full">
        <div class="flex w-full justify-end " data-tauri-drag-region>
            <div class="flex content-center w-full h-[var(--title-bar-height)] px-4" data-tauri-drag-region>
                <span v-if="hadTitle"
                    class="h-[var(--title-bar-height)] leading-[var(--title-bar-height)] text-center text-2xl">
                    {{ title.title }}
                </span>
            </div>
            <WeightWindowMenu />
        </div>
        <div class="line-h"></div>
        <div class="h-full">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>

</style>