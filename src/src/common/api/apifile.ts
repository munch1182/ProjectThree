import { get, post, dataOrReject, getBaseUrl } from "../api";

export type ImageInfo = { name: string, url: string, len: number, dimen: { w: number, h: number }, operate?: number, target?: string }
/**
 * 将图片上传并返回图片信息 
 */
export const imgInput = async (f: File) => {
    const p = new FormData();
    p.append("f_i_1", f, f.name) // f_i_1
    return post("/f/i", p)
        .then(l => dataOrReject(l))
        .then(l => l[0])
        .then(async i => {
            const startUrl = await getBaseUrl()
            if (startUrl) {
                i.url = startUrl.concat(i.url)
            }
            return i
        })
}

/**
 * 上传image并转为icon, 返回的地址为icon的地址
 * @param f 如果传入file, 则先上传再转换, 如果传入string,即url, 则会直接转换(可以传入全路径)
 * 
 * return 返回的参数里总有原图片的url
 */
export const img2icon = async (f: File | string) => {
    let url = '';
    if (f instanceof File) {
        url = (await imgInput(f)).url;
    } else if (typeof f === 'string') {
        const startUrl = await getBaseUrl()
        if (startUrl) {
            url = f.replace(startUrl, "")
        } else {
            url = f
        }
    }
    if (url) {
        return Promise.reject("error premeter")
    }
    return get("/f/i/1", { url })
        .then<ImageInfo[]>(l => dataOrReject(l))
        .then(l => l[0])
        .then(async i => {
            const startUrl = await getBaseUrl()
            if (startUrl) {
                i.url = startUrl.concat(i.url)
            }
            return i
        })
}

