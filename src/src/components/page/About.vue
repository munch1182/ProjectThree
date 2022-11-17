<script setup lang="ts">
import { ref } from "vue";
import { testStartTime } from "../../common/api";

const show = ref()
testStartTime().then(r => show.value = r.toString())

</script>

<template>
    <div class="p-[var(--space-padding)]">
        {{ show }}
    </div>
</template>

<style scoped>

</style>