import { invoke } from "@tauri-apps/api";
import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from "axios";

function getNet(): AxiosInstance {
    return axios
}

// 当第一次import此文件时, 就行调用此方法
getNet().interceptors.response.use(async res => {
    const body = await judgeStatusCode(res) // 判断状态码是否返回200, 否则走reject
    return await judgeBaseResponse(body) // 如果不是BaseResponse, 返回原数据, 否则判断是否回复code为0, 为0则返回data, 否则走reject(错误码)
})


/**
 * 测试api, 返回服务器开始时间
 */
export const testStartTime = async () => get<{ startTime: number }>("/t/t").then(r => r?.startTime).then(r => dataOrDefault(r, 0))

// class BaseResponse<D = any> {
//     code!: number;
//     data?: D
// }

/**
 * 如果data没有值, 则返回def
 */
export async function dataOrDefault<D>(data: D | undefined | null, def: D): Promise<D> {
    return Promise.resolve(data ? data : def)
}

/**
 * 如果data没有值, 则返回reject 
 */
export async function dataOrReject<D>(data: D | undefined | null): Promise<D> {
    return data ? Promise.resolve(data) : Promise.reject("no data")
}

/**
 * 执行get请求, D在成功时为BaseResponsed的data类型, 使用getNet失败或者不是BaseResponsed都会走reject
 */
export async function get<D = any>(url: string, config?: AxiosRequestConfig<any>): Promise<D | undefined | null> {
    return fullUrl(url) // 拼装url
        .then(u => getNet().get(u, config)) // 执行请求
}

export async function post<D = any>(url: string, p?: any, config?: AxiosRequestConfig<any>): Promise<D | undefined | null> {
    return fullUrl(url) // 拼装url
        .then(u => getNet().post(u, p, config)) // 执行请求
}

/**
 * 从body中取出数据
 */
function judgeStatusCode(r: AxiosResponse<any, any>): Promise<any> {
    if (r.status != 200) {
        return Promise.reject("error: status ".concat(r.status.toString()))
    }
    const data = r.data;
    if (!data) {
        return Promise.reject("error: no data")
    }
    return Promise.resolve(r.data);

}

/**
 * 从BaseResponse中取出数据
 */
function judgeBaseResponse(bb: any): Promise<any | undefined | null> {
    // 只判断BaseBean的code然后返回data
    if ("code" in bb) {
        return bb.code == 0 ? Promise.resolve(bb.data) : Promise.reject(bb.code) // 此时bb.data可能为空
    }
    // 不是BaseBean则直接错误: 应该使用其它配置进行请求
    return Promise.resolve(-1)
}

let startUrl: string | undefined = undefined

/**
 * 组装成完全的url
 * 
 * @param url 基础url之后的部分, 需要以/开头 
 */
export async function fullUrl(url: string): Promise<string> {
    startUrl = await getBaseUrl()
    if (!startUrl) {
        return Promise.reject("no server")
    } else {
        return Promise.resolve(startUrl.concat(url))
    }
}

/**
 * @returns 获取baseurl
 */
export async function getBaseUrl(): Promise<string> {
    if (!startUrl) {
        startUrl = await invoke<string>("server_or_empty")
    }
    return Promise.resolve(startUrl)
}

