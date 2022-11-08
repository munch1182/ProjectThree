<script setup lang="ts">
import { ref } from 'vue';
import PartContentWrapperView from './PartContentWrapper.vue'
import PartNavView from './PartNav.vue'
import Home from './page/Home.vue'

const title = ref("首页")
</script>
<template>
    <div class="flex flex-row h-full">
        <div class="flex w-48 h-screen bg-[var(--nav-color)]">
            <PartNavView />
        </div>
        <div class="line-w"></div>
        <div class="flex flex-grow bg-[var(--content-color)]">
            <PartContentWrapperView :title="title">
                <Home />
            </PartContentWrapperView>
        </div>
    </div>
</template>