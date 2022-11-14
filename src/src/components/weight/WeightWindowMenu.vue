<script setup lang="ts">
import { ref } from "vue";
import { minimizeWindow, closeWindow, alawysTop } from "../../common/win";

const alawys = ref(false)

function pinWindow() {
    alawys.value = !alawys.value
    alawysTop(alawys.value)
}
</script>

<template>
    <div class="h-[var(--height-window-menu)] flex flex-row">
        <div @click="pinWindow" class="window-icon" :class="{alawys:alawys}">
            <i class="iconfont icon-pin"></i>
        </div>
        <div @click="minimizeWindow" class="window-icon">
            <i class="iconfont icon-minimizing"></i>
        </div>
        <div @click="closeWindow" class="window-icon">
            <i class="iconfont icon-close"></i>
        </div>
    </div>
</template>

<style scoped lang="postcss">
.window-icon {
    @apply w-max px-4 py-1
}

.window-icon:hover {
    @apply bg-gray-300
}

.alawys {
    @apply bg-[var(--color-nav-hover-bg)]
}
</style>