import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useServerStore = defineStore("server", () => {
    const _SERVER_DEF = "NULL"
    const _server = ref(_SERVER_DEF)

    /**
     * @returns 该参数是否未设置
     */
    const isNull = computed(() => _server.value == _SERVER_DEF)

    /**
     * set
     */
    const get = computed(() => _server.value.replace("localhost:", ""))

    /**
     *  set, 如果传入null或者不传参数, 将清除设置的值
     */
    function set(str?: string) {
        if (str != undefined && str != null && str.length) {
            _server.value = str
        } else {
            _server.value = _SERVER_DEF
        }
    }

    return { isNull, set, get }
})