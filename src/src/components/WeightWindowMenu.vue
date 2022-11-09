<script setup lang="ts">
import { ref } from "vue";
import { currWin, alawysTop } from "../common/win";

const alawys = ref(false)

function minimizeWindow() {
    const curr = currWin();
    if (!curr) return
    curr.minimize()
}

function closeWindow() {
    const curr = currWin();
    if (!curr) return
    curr.close()
}

function pinWindow() {
    alawys.value = !alawys.value
    alawysTop(alawys.value)
}
</script>

<template>
    <div class="flex h-[var(--height-window-menu)]">
        <div @click="pinWindow" class="btn_win" :class="{alway:alawys}">
            <i class="iconfont icon-pin"></i>
        </div>
        <div @click="minimizeWindow" class="btn_win">
            <i class="iconfont icon-minimizing"></i>
        </div>
        <div @click="closeWindow" class="btn_win">
            <i class="iconfont icon-close"></i>
        </div>
    </div>
</template>

<style scoped lang="postcss">
.btn_win {
    @apply w-max px-4 py-1
}

.btn_win:hover {
    @apply bg-gray-300
}

.alway {
    @apply bg-[var(--color-nav-hover-bg)]
}
</style>