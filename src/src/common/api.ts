import axios, { AxiosResponse } from "axios";
import { useServerStore } from "../stores/server";
import { invoke } from '@tauri-apps/api'

export const openWithSystemBrower = (addr: string) => get("/cmd/1/".concat(addr))
export const getServerAddr = () => invoke<string>('server_addr')
export const setProxyServer = (cmd: number) => get("/set/1/".concat(cmd.toString()))
export const bookmark = (url: string, title?: string) => get("/set/2/", { url, title })

function get<T = any, R = AxiosResponse<T>>(url: string, params?: any): Promise<R> {
    return getFullUrl(url).then((url) => axios.get(url, { params: params }))
}

function post<T = any, R = AxiosResponse<T>, D = any>(url: string, data?: D,): Promise<R> {
    return getFullUrl(url).then((url) => axios.post(url, data))
}

function getFullUrl(url: string): Promise<string> {
    //return Promise.resolve("http://localhost:62242".concat(url))
    const addr = useServerStore()
    if (addr.isNull) {
        return Promise.reject("no url")
    }
    return Promise.resolve("http://".concat(addr.get).concat(url))
}
