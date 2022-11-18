<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { ref } from 'vue'
import { imgInput, ImageInfo } from "../../common/api/apifile";
import { size2str } from "../../common/helper";

const infoRef = ref<ImageInfo>()
const input = ref()
const preview = ref()

function drop(e: DragEvent) {
    const files = e.dataTransfer?.files
    fileHandle(files)
    e.dataTransfer?.clearData()
}

// 点击
function openChose() {
    input.value.click()
}

function onChange() {
    if ("files" in input.value) {
        fileHandle(input.value.files)
    }
}

async function fileHandle(fs: FileList | undefined) {
    if (!fs || !fs.length) {
        return
    }

    infoRef.value = undefined
    const f = fs[0]

    imgInput(f).then(info => {
        infoRef.value = info
        preview.value.src = info.url
    }).catch(e => console.log(e))
}

const dimen = computed(() => {
    const v = infoRef.value
    if (!v) {
        return undefined
    }
    const d = v.dimen
    return d.w + "x" + d.h
})

const lenStr = computed(() => {
    const v = infoRef.value
    if (!v) {
        return undefined
    }
    return size2str(v.len)
})

function openImg() {
    // todo open by system
}

function convertIcon() {

}
</script>

<template>
    <!-- 图片 -->
    <div class="p-[var(--space-padding)]" id="pic">
        <div class="flex h-[10rem]">
            <!-- 文件上传区域 -->
            <div @dragenter.stop.prevent="" @dragover.stop.prevent="" @dragleave.stop.prevent=""
                @drop.stop.prevent="drop" @click="openChose"
                class="w-[var(--size)] h-[var(--size)] flex items-center justify-center border-div">
                <input ref="input" type="file" style="display: none;" @change="onChange">
                <i class="iconfont icon-add" style="font-size: 32px;color: #909399;"></i>
            </div>

            <!-- 预览界面 -->
            <div v-show="infoRef != undefined" class="w-max h-max flex flex-col ml-[var(--space-padding)]">
                <div
                    class="w-max h-max max-w-[var(--size)] max-h-[var(--size)] p-[var(--space-padding)]  flex items-center justify-center border-div">
                    <img ref="preview" @click="openImg">
                </div>
                <span class="text-[0.6rem] mt-2 block text-center text-gray-600">{{ dimen }}</span>
            </div>

            <!-- 图片信息 -->
            <div v-show="infoRef != undefined"
                class="flex flex-col max-w-[var(--size)] h-max mx-[var(--space-padding)]">
                <span class="text-[0.6rem] mt-2 inline-block break-words">{{ infoRef?.name }}</span>
                <span class="text-[0.5rem] mt-1 text-gray-600">{{ lenStr }}</span>
            </div>
        </div>
        <div class="flex flex-col" @click="convertIcon">
            <input type="button" class="btn" value="生成icon">
        </div>
    </div>
</template>

<style scoped lang="postcss">
#pic {
    --size: 8rem
}

.border-div {
    @apply rounded-lg border-2 border-[#cdd0d6] border-dotted
}
</style>
