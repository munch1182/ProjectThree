import { onBeforeUnmount, onMounted } from "vue";

/**
 * 禁止文件拖动到窗口, 控件中仍可以设置拖拽事件实现
 */
export default function disDrag() {
    onMounted(() => {
        window.addEventListener('dragenter', disdrag)
        window.addEventListener('dragleave', disdrag)
        window.addEventListener('dragover', disdrag)
        window.addEventListener('drop', disdrag)
    })
    onBeforeUnmount(() => {
        window.removeEventListener('dragenter', disdrag)
        window.removeEventListener('dragleave', disdrag)
        window.removeEventListener('dragover', disdrag)
        window.removeEventListener('drop', disdrag)
    })

    function disdrag(e: DragEvent) {
        e.preventDefault()
        const t = e.dataTransfer
        if (t) {
            t.effectAllowed = "none"
            t.dropEffect = "none"
        }
    }
}