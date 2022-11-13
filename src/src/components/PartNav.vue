<script setup lang="ts">
import { computed } from 'vue';
import { RouteRecordNormalized, RouteRecordRaw, useRouter } from 'vue-router'
import WeightServerStatus from './WeightServerStatus.vue';

const router = useRouter()

class Nav {
    readonly p: string //path
    readonly t: string | undefined //title
    readonly s: boolean //是否正在显示
    children: Nav[] | null | undefined // children

    constructor(p: string, t: string, s: boolean, children?: Nav[] | null | undefined) {
        this.p = p
        this.t = t
        this.s = s
        this.children = children
    }
}

function getNavsFromRouter(r: RouteRecordNormalized): Nav {
    const nav = new Nav(r.path, r.meta.title as string, router.currentRoute.value.fullPath == r.path)
    if (r.children.length) {
        nav.children = r.children.map(cr => getNavsFromChildRouter(r, cr))
    }
    return nav
}

function getNavsFromChildRouter(r: RouteRecordNormalized, cr: RouteRecordRaw): Nav {
    let path = r.path.concat("/").concat(cr.path)
    let title = r.meta.title as string
    if (cr.meta) {
        title = cr.meta.title as string
    }
    return new Nav(path, title, router.currentRoute.value.fullPath == path)
}

// 从路由设置中构建nav
const navs = computed(() => router.getRoutes()
    .filter(r => (r.meta.title && r.path.split('/').length < 4))
    .map(r => getNavsFromRouter(r)))

// 更改页面
function update(n: Nav) {

    if (n.s) return
    router.push(n.p)
}

</script>

<template>
    <div class="flex flex-col justify-end w-full">
        <div class="flex flex-col justify-start h-full mt-[var(--height-title-bar)]">
            <ul>
                <li v-for="n in navs" :key="n.p" class="w-full list-none" @click="update(n)">
                    <a :class="{ select: n.s }">{{ n.t }}</a>
                    <ul v-if="n.children">
                        <li v-for="c in n.children" @click.stop="update(c)">
                            <a :class="{ select: c.s }">{{ c.t }}</a>
                        </li>
                    </ul>
                </li>
            </ul>
        </div>
        <WeightServerStatus />
    </div>
</template>

<style scoped lang="postcss">
a {
    @apply flex justify-center items-center h-14 text-base;
}

li ul li a {
    @apply h-8 text-xs;
}

a:hover {
    @apply bg-[var(--color-nav-hover-bg)]
}

.select {
    @apply bg-[var(--color-select-bg)] pointer-events-none
}
</style>