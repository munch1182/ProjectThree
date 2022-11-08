<script setup lang="ts">
import { ref, watch } from 'vue';

const searchValue = ref("")

watch(searchValue, (newValue) => delay(() => input(newValue)))

function input(value: string) {
    if (!value) return
    console.log(value)
    // goto search
}

/**
 * 延时处理
 */
let timer: ReturnType<typeof setTimeout>
function delay(fn: () => void) {
    timer ? clearTimeout(timer) : null
    timer = setTimeout(fn, 500);
}
</script>

<template>
    <div>
        <input v-model.trim="searchValue"
            class="focus:outline-none border-cyan-300 border-2 rounded-lg px-2 py-1 w-1/3">
        <br>
        {{ searchValue }}
    </div>
</template>

<style scoped>

</style>