import { invoke } from "@tauri-apps/api";
import axios, { AxiosResponse } from "axios";


export const testStartTime = async () => get<{ startTime: number; }>("/t/t").then(r => r?.startTime).then(r => dataOrDefault(r, 0))

export type ImageInfo = { name: string, url: string, len: number, dimen: { w: number, h: number } }
export const imgInput = async (f: File) => {
    const p = new FormData();
    p.append("f_i_1", f, f.name) // f_i_1
    return post<ImageInfo[]>("/f/i", p)
        .then(l => dataOrReject(l))
        .then(l => l[0])
        .then(i => {
            i.url = startUrl!!.concat(i.url)
            return i
        })
}

// class BaseResponse<D = any> {
//     code!: number;
//     data?: D
// }

/**
 * 如果data没有值, 则返回def
 */
async function dataOrDefault<D>(data: D | undefined, def: D): Promise<D> {
    return Promise.resolve(data ? data : def)
}

/**
 * 如果data没有值, 则返回reject 
 */
async function dataOrReject<D>(data: D | undefined): Promise<D> {
    return data ? Promise.resolve(data) : Promise.reject("no data")
}

async function get<D = any>(url: string, config?: any): Promise<D | undefined> {
    return getFullUrl(url) // 拼装url
        .then(u => axios.get(u, config)) // 执行请求
        .then(r => judgeStatusCode(r)) // 判断系统返回码
        .then(b => judgeBaseResponse<D>(b)) // 从BaseResponse中取出数据
}

async function post<D = any>(url: string, p?: any, config?: any): Promise<D | undefined> {
    return getFullUrl(url) // 拼装url
        .then(u => axios.post(u, p, config)) // 执行请求
        .then(r => judgeStatusCode(r)) // 判断系统返回码
        .then(b => judgeBaseResponse<D>(b)) // 从BaseResponse中取出数据
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
function judgeBaseResponse<D>(bb: any): Promise<D | undefined> {
    // 只判断BaseBean的code然后返回data
    if ("code" in bb) {
        return bb.code == 0 ? Promise.resolve(bb.data) : Promise.reject("error: ".concat(bb.code.toString()))
    }
    // 不是BaseBean强转返回
    return Promise.resolve(bb as D)
}

let startUrl: string | undefined = undefined

/**
 * @param url 基础url之后的部分, 需要以/开头 
 */
async function getFullUrl(url: string): Promise<string> {
    if (startUrl) {
        return Promise.resolve(startUrl.concat(url))
    }
    startUrl = await invoke<string>("server_or_empty")
    if (!startUrl) {
        return Promise.reject("no server")
    } else {
        return Promise.resolve(startUrl.concat(url))
    }
}