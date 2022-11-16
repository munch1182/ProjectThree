<script setup lang="ts">
import PartContentWrapper from '../components/weight/PartContentWrapper.vue';
import PartNav from './weight/PartNav.vue';
</script>

<template>
    <div class="flex h-screen">
        <!-- 页面模板, 包括nav+content的标题部分, 内容部分使用router, 需是main的子路由 -->
        <PartNav />
        <div class="line-h"></div>
        <PartContentWrapper>
            <RouterView />
        </PartContentWrapper>
    </div>
</template>

<style scoped>

</style>