<script setup lang="ts">

import { computed } from 'vue';
import { RouteRecordNormalized, useRouter } from 'vue-router';

class Nav {
    path!: string; // 自己的路由
    title!: string; // 名字
    isNow?: boolean = false; // 是否是当前路径
    children?: Nav[] | void | null; // 子部分
}

const router = useRouter()

// 非最终路径不能有title // todo childen
const navs = computed(() => router.getRoutes().filter(r => r.meta.title).map(r => navFromRouter(r)))

function navFromRouter(r: RouteRecordNormalized): Nav {
    return { path: r.path, title: r.meta.title as string, isNow: router.currentRoute.value.fullPath == r.path, children: null }
}

function update(n: Nav) {
    router.push(n.path)
    navs.value.forEach(r => { r.isNow = r.path == n.path })
}
</script>

<template>
    <div class="w-[var(--width-nav)] h-full bg-[var(--color-nav-bg)]">
        <ul class="mt-[var(--height-title-bar)]">
            <li v-for="n in navs" :key="n.path" @click="update(n)" :class="{ isNow: n.isNow }" class="item">
                {{ n.title }}
            </li>
        </ul>
    </div>
</template>

<style scoped lang="postcss">
.isNow {
    @apply bg-[var(--color-nav-select-bg)] pointer-events-none
}

li:hover {
    @apply bg-[var(--color-nav-hover-bg)]
}

.item {
    @apply py-3 text-center
}
</style>