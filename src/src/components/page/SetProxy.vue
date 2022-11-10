<script setup lang="ts">
import { ref } from 'vue'
import { setProxyServer } from "../../common/api";

const started = ref(false)
const starting = ref(false)

function startProxy() {
    starting.value = true
    setProxyServer(1)
        .then(() => started.value = true)
        .catch(() => { })
        .finally(() => starting.value = false)
}
</script>

<template>
    <div class="flex flex-col py-4 px-8">
        <div class="item-line">
            <span class="text">当前状态:</span>
            <span class="space text">{{ started ? "已设置" : "未设置" }}</span>
        </div>

        <div class="item-line">
            <span class="text">代理地址:</span>
            <input class="input-line" readonly placeholder="http://127.0.0.1" />
        </div>

        <div class="item-line">
            <input type="checkbox" id="startWithApp">
            <label class="space text" for="startWithApp">随应用启动</label>
        </div>
        <div class="item-line">
            <input type="checkbox" id="startWithApp">
            <label class="space text" for="startWithApp">设置Git代理</label>
        </div>
        <div class="item-line">
            <input type="checkbox" id="startWithApp">
            <label class="space text" for="startWithApp">设置npm代理</label>
        </div>
        <div class="item-line mt-4">
            <button type="button" class="btn" :class="{ disable: starting }" @click="startProxy">
                {{ starting ? "启动中" : "启动" }}
            </button>
        </div>
    </div>
</template>

<style scoped lang="postcss">
.disable {
    @apply pointer-events-none
}
</style>