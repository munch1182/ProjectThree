<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { useServerStore } from "../stores/server";
import { openWithSystemBrower } from "../common/api"
const addr = useServerStore()

function getOrOpen() {
    if (addr.isNull) {
        get()
    } else {
        openWithSystemBrower(addr.get).then()
    }

}

function get() {
    invoke<string>('server_addr').then(res => addr.set(res))
}

get()
</script>

<template>
    <div class="flex flex-row-reverse px-4 py-2">
        <button @click="getOrOpen" class="text-xs text-gray-400">{{ addr.get }}</button>
    </div>
</template>

<style scoped>

</style>