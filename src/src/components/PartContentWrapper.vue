<script setup lang="ts">
import { computed } from '@vue/reactivity';
import WeightWindowMenu from './WeightWindowMenu.vue';

const props = defineProps({ title: String })
const hadTitle = computed(() => props.title)
</script>

<template>
    <div class="flex flex-col w-full">
        <div class="flex w-full justify-end " data-tauri-drag-region>
            <div class="flex content-center w-full h-[var(--height-title-bar)] px-4" data-tauri-drag-region>
                <span v-if="hadTitle"
                    class="h-[var(--height-title-bar)] leading-[var(--height-title-bar)] text-center text-2xl">
                    {{ props.title }}
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