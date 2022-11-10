import axios, { AxiosResponse } from "axios";
import { useServerStore } from "../stores/server";
import { invoke } from '@tauri-apps/api'

export const openWithSystemBrower = (addr: string) => get("/cmd/1/".concat(addr))
export const getServerAddr = () => invoke<string>('server_addr')
export const setProxyServer = (cmd: number) => get("/set/1/".concat(cmd))

function get<T = any, R = AxiosResponse<T>>(url: string, params?: any): Promise<R> {
    var _url = getFullUrl(url)
    if (_url == null) {
        return Promise.reject()
    }
    return axios.get(_url, { params: params })
}

function post<T = any, R = AxiosResponse<T>, D = any>(url: string, data?: D,): Promise<R> {
    var _url = getFullUrl(url)
    if (_url == null) {
        return Promise.reject()
    }
    return axios.post(_url, data)
}

function getFullUrl(url: string): string | null {
    const addr = useServerStore()
    if (addr.isNull) {
        return null
    }
    return "http://".concat(addr.get).concat(url)
}
