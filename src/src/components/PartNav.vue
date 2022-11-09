<script setup lang="ts">
import { useRouter } from 'vue-router'
import WeightServerStatus from './WeightServerStatus.vue';
import { Nav, useNavStore } from "../stores/nav";

const navs = useNavStore()
const router = useRouter()

function update(nav: Nav) {
    if (nav.showNow) return
    router.push(nav.fullRouter())
    navs.show(nav)
}
</script>

<template>
    <div class="flex flex-col justify-end w-full">
        <div class="flex flex-col justify-start h-full mt-[var(--height-title-bar)]">
            <ul>
                <li v-for="nav in navs.get" :key="nav.id" class="w-full list-none" @click="update(nav)">
                    <a :class="{ select: nav.showNow }">{{ nav.name }}</a>
                </li>
            </ul>
        </div>
        <WeightServerStatus />
    </div>
</template>

<style scoped lang="postcss">
a {
    @apply flex justify-center items-center h-16 text-base;
}

a:hover {
    @apply bg-[var(--color-nav-hover-bg)]
}

.select {
    @apply bg-[var(--color-select-bg)] pointer-events-none
}
</style>