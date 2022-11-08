<script setup lang="ts">
import { useServerStore } from "../stores/server";
import { openWithSystemBrower, getServerAddr } from "../common/api"
const addr = useServerStore()

function getOrOpen() {
    if (addr.isNull) {
        get()
    } else {
        openWithSystemBrower(addr.get).then()
    }
}

function get() {
    getServerAddr().then(res => addr.set(res))
}

get()
</script>

<template>
    <div class="px-4 py-2">
        <button @click="getOrOpen" class="text-xs text-gray-400">{{ addr.get }}</button>
    </div>
</template>