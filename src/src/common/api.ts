import { invoke } from "@tauri-apps/api";
import { u } from "@tauri-apps/api/globalShortcut-ff939597";
import axios, { AxiosResponse } from "axios";

export const testStartTime = async () => get<{ startTime: number; }>("/t/t").then(r => r.startTime)

class BaseResponse<D = any> {
    code!: number;
    data?: D

    isOk(): boolean {
        return this.code == 0
    }
}

async function get<D = any>(url: string, config?: any): Promise<D> {
    return getFullUrl(url) // 拼装url
        .then(u => axios.get(u, config)) // 执行请求
        .then(r => judgeStatusCode(r)) // 判断系统返回码
        .then(b => judgeBaseResponse<D>(b)) // 从BaseResponse中取出数据
}

async function post(url: string, p?: any, config?: any): Promise<any> {
    return getFullUrl(url).then(u => axios.post(u, p, config)).then(r => judgeStatusCode(r));
}

/**
 * 从body中取出数据
 */
function judgeStatusCode(r: AxiosResponse<any, any>): Promise<any> {
    console.log(r);

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
function judgeBaseResponse<D>(bb: any): Promise<D> {
    console.log("bb", bb);

    if (bb instanceof BaseResponse<D>) { // 不是BaseBean强转返回
        return Promise.resolve(bb as D)
    }
    // 否则判断BaseBean的code然后返回data
    return (bb.isOk() && bb.data) ? Promise.resolve(bb.data) : Promise.reject("error: ".concat(bb.code.toString()))
}

/**
 * @param url 基础url之后的部分, 需要以/开头 
 */
async function getFullUrl(url: string): Promise<string> {
    const startUrl = await invoke<string>("server_or_empty")
    if (!startUrl) {
        return Promise.reject("no server")
    } else {
        return Promise.resolve(startUrl.concat(url))
    }
}