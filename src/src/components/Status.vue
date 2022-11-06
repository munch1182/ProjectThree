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
    <button @click="getOrOpen">{{ addr.get }}</button>
</template>

<style scoped>
button {
    font-size: 0.1rem;
}
</style>