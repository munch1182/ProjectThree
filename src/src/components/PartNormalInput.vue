<script setup lang="ts">
import { ref, watch } from 'vue';

const searchValue = ref("")
const inputValue = ref("")

watch(searchValue, (newValue) => delay(() => input(newValue)))

function input(value: string) {
    if (!value) return
    console.log(value)
    // goto search
}

function complate() {
    const str = searchValue.value
    inputValue.value = str
    searchValue.value = ""
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
    <div class="flex flex-col h-full">
        <div class="flex-grow p-4">{{ inputValue }}</div>
        <div class="line-h"></div>
        <div class="p-4 h-32">
            <!-- // 双次回车(trim)/alt+回车输入完成 -->
            <textarea v-model.trim="searchValue" @keyup.alert.enter="complate"
                class="px-2 w-full h-full resize-none focus:outline-none bg-[var(--color-content-bg)]">
            </textarea>
        </div>
    </div>
</template>