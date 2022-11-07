import axios, { AxiosResponse } from "axios";
import { useServerStore } from "../stores/server";

//axios.interceptors.response.use()

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


export { get, post }