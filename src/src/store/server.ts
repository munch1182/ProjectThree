import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useServerStore = defineStore("server", () => {
    const _SERVER_DEF = "NULL"
    const _server = ref(_SERVER_DEF)

    /**
     * @returns 该参数是否未设置
     */
    function isNull(): boolean {
        return _server.value == _SERVER_DEF
    }

    /**
     * 清除该参数的设置
     */
    function clear() {
        _server.value = _SERVER_DEF
    }

    /**
     *  set
     */
    function set(str?: string) {
        if (str != undefined && str != null && str.length) {
            _server.value = str
        } else {
            clear()
        }
    }

    /**
     * get
     */
    function get(): string {
        return _server.value
    }

    return { isNull, clear, get, set }
})