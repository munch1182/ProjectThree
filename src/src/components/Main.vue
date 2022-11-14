<script setup lang="ts">
import PartContentWrapper from '../components/weight/PartContentWrapper.vue';
import PartNav from './weight/PartNav.vue';

</script>

<template>
    <div class="flex h-screen">
        <PartNav />
        <div class="line-h"></div>
        <PartContentWrapper>
            <RouterView />
        </PartContentWrapper>
    </div>
</template>

<style scoped>

</style>