<script setup lang="ts">
import { ref } from 'vue';

const inputValue = ref("")
const emits = defineEmits(['input']) //父组件:@input="fun2receive"

function complate() {
    const str = inputValue.value
    inputValue.value = ""
    emits("input", str)
}
</script>

<template>
    <div class="flex flex-col h-full">
        <div class="flex-grow p-4">
            <slot></slot>
        </div>
        <div class="line-h"></div>
        <div class="p-4 h-32">
            <!-- // 双次回车(trim)/alt+回车输入完成 -->
            <textarea v-model.trim="inputValue" @keyup.alert.enter="complate"
                class="px-2 w-full h-full resize-none focus:outline-none bg-[var(--color-content-bg)]">
            </textarea>
        </div>
    </div>
</template>