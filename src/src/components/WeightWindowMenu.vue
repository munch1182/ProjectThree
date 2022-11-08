<script setup lang="ts">
import { currWin, newWinow } from "../common/win";

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
    newWinow("/aaaa")
}
</script>

<template>
    <div class="flex h-[var(--window-menu-height)]">
        <div @click="pinWindow" class="btn_win ">
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
</style>