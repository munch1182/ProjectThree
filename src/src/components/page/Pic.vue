<script setup lang="ts">
import { ref } from 'vue'
import { imgInput } from "../../common/api";

class ImgInfo {
    name!: string; // 文件名
    path!: string; // 文件路径
    type!: string; // 文件真实类型
    length!: number; // 文件大小


    lenStr(): string {
        return ((this.length / 1024).toFixed(2)).toString().concat("k");
    }
}

function infoFromFile(f: File): ImgInfo {
    const ii = new ImgInfo()
    ii.name = f.name
    ii.path = "none"
    ii.type = f.type
    ii.length = f.size
    return ii
}

const info = ref<ImgInfo>()
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

    const f = fs[0]
    info.value = infoFromFile(f)

    imgInput(f).then(r => { }).catch(e => console.log(e))

    const fr = new FileReader()
    fr.onload = () => preview.value.src = fr.result
    fr.readAsDataURL(f) // 将其转为base64

}

function openImg() {
    // todo open by system
}
</script>

<template>
    <div class="p-[var(--space-padding)]">
        <div class="flex h-[128px]">
            <div @dragenter.stop.prevent="" @dragover.stop.prevent="" @dragleave.stop.prevent=""
                @drop.stop.prevent="drop" @click="openChose"
                class="w-[128px] h-full flex items-center justify-center border-div">
                <input ref="input" type="file" style="display: none;" @change="onChange">
                <i class="iconfont icon-add" style="font-size: 32px;color: #909399;"></i>
            </div>
            <div v-show="info != undefined" class="flex flex-col max-w-[128px] h-full mx-[var(--space-padding)]">
                <span class="text-xs mt-2">{{ info?.name }}</span>
                <span class="text-xs mt-2">{{ info?.type }}</span>
                <span class="text-xs mt-1">{{ info?.lenStr() }}</span>
            </div>
            <div v-show="info != undefined"
                class="w-[128px] h-[128px] mx-[var(--space-padding)] flex items-center justify-center border-div">
                <img ref="preview" @click="openImg">
            </div>
        </div>
    </div>
</template>

<style scoped lang="postcss">
.border-div {
    @apply rounded-lg border-2 border-[#cdd0d6] border-dotted
}
</style>
