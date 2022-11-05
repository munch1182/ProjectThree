<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { useServerStore } from "../store/server";

const addr = useServerStore()

function getOrOpen() {
    if (addr.isNull()) {
        console.log("get")
    } else {
        console.log("open")
    }
}

function open(url: string) {
}

function get() {
    invoke<string>('server_addr').then(res => addr.set(res))
}

get()
</script>

<template>
    <a class="md:flex max-w-auto hover:bg-gray-300" @click="getOrOpen">
        <button>{{ addr.get() }}</button>
    </a>
</template>

<style scoped>
a {
    font-size: 0.2rem;
}
</style>