<script setup lang="ts">
import { ref } from 'vue';

const searchValue = ref("")

//watchEffect(() => searchTimer(() => console.log(searchValue.value)))
</script>

<template>
    <div>
        <input v-model="searchValue" class="focus:outline-none border-cyan-300 border-2 rounded-lg px-2 py-1 w-1/3">
    </div>
</template>

<style scoped>

</style>