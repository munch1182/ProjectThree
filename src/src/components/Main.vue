<script setup lang="ts">
import PartContentWrapperView from './PartContentWrapper.vue'
import PartNavView from './PartNav.vue'
import { useNavStore } from '../stores/nav'

const nav = useNavStore()

</script>
<template>
    <div class="flex flex-row h-full">
        <div class="flex w-[var(--width-nav)] min-w-[var(--width-nav)] h-screen bg-[var(--color-nav-bg)]">
            <PartNavView />
        </div>
        <div class="line-w"></div>
        <div class="flex flex-grow bg-[var(--color-content-bg)]">
            <PartContentWrapperView :title="nav.title">
                <RouterView />
            </PartContentWrapperView>
        </div>
    </div>
</template>