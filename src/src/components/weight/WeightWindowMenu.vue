<script setup lang="ts">
import { ref } from "vue";
import { minimizeWindow, closeWindow, alawysTop } from "../../common/win";

const alawys = ref(false)

function pinOr() {
    pin(!alawys.value)
}

function pin(b: boolean) {
    alawys.value = b
    alawysTop(b)
}

function minimize() {
    pin(false)
    minimizeWindow()
}

function close() {
    pin(false)
    closeWindow()
}

</script>

<template>
    <!-- 窗口控制功能 -->
    <div class="h-[var(--height-window-menu)] flex flex-row">
        <div @click="pinOr" class="window-icon" :class="{alawys:alawys}">
            <i class="iconfont icon-pin"></i>
        </div>
        <div @click="minimize" class="window-icon">
            <i class="iconfont icon-minimizing"></i>
        </div>
        <div @click="close" class="window-icon">
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