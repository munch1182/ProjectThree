import { post, dataOrReject, getBaseUrl, fullUrl } from "./api";

const URL_File = "/f"

const URL_IMG_INPUT = URL_File + "/i/u"
const URL_IMG_OPERA = URL_File + "/i/1"

export type ImageDimen = { w: number, h: number }
export type ImageInfo = { name: string, url: string, len: number, dimen: ImageDimen, operate?: ImageOperateType, value?: number, target?: string }

/**
 * 将图片上传并返回图片信息 
 */
export const imgInput = async (f: File) => {
    const p = new FormData();
    p.append("f_i_1", f, f.name) // f_i_1
    return post<ImageInfo[]>(URL_IMG_INPUT, p)
        .then(l => dataOrReject(l))
        .then(l => l[0])
        .then(async i => {
            i.url = await fullUrl(i.url)
            return i
        })
}

type ImageOperateReq = { url: string, operate?: ImageOperateType, dimen?: ImageDimen[] }
type ImageOperateType = {
    "ico"?: number, // size
    "flip"?: number, // 0为水平, 1为垂直
    "crop"?: { x: number, y: number, w: number, h: number },
    "resize"?: { w: number, h: number }, // 要修改成的宽高
    "blur"?: number,
    "rotate"?: number
}
/**
 * 上传image并转为icon, 返回的地址为icon的地址
 * @param f 如果传入file, 则先上传再转换, 如果传入string, 即url, 则会直接转换(可以传入全路径)
 * 
 * return 返回的参数里总有原图片的url
 */
export const img2icon = async (f: File | string, size: number = 128) => {
    return imageOperate(f, { "ico": size })
}

async function imageOperate(f: File | string, operate?: ImageOperateType, dimen?: ImageDimen[]): Promise<ImageInfo> {
    const url = await imgfileReqUrl(f);
    const req: ImageOperateReq = { url, operate, dimen }
    return post(URL_IMG_OPERA, [req])
        .then<ImageInfo[]>(l => dataOrReject(l))
        .then(l => l[0])
        .then(async i => {
            i.url = await fullUrl(i.url)
            return i
        })
}

/**
 * 传入图片的文件类型, 会上传该文件并返回url(不含baseurl)
 * 传入图片的url, 如果是全路径, 则去除baseurl并返回url, 否则直接返回 
 */
async function imgfileReqUrl(f: File | string): Promise<string> {
    let url = '';
    if (f instanceof File) {
        url = (await imgInput(f)).url;
    } else if (typeof f === 'string') {
        if (f.startsWith("http")) { // 如果是全url
            const startUrl = await getBaseUrl()
            if (startUrl) {
                url = f.replace(startUrl, "")
            }
        } else {
            url = f
        }
    }
    if (!url) {
        return Promise.reject("error premeter")
    }
    return Promise.resolve(url)
}
