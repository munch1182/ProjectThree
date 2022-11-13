import { readDir, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { dataDir } from '@tauri-apps/api/path';
import { ApiDoc } from "./apidoc";

const app = await dataDir();
const dir = app.concat("apidoc")
/**
 * 从该文件夹路径中读取api并解析 
 */
export async function getFromDir(): Promise<ApiDoc[] | void> {
    const list: ApiDoc[] = []
    try {
        console.log(1);

        console.log(2);

        const entries = await readDir(dir)
        console.log(3);

        for (const entry of entries) {
            console.log(entry.path);

            const text = await readTextFile(entry.path)
            list.push(JSON.parse(text))
        }
    } catch (error) {
    }
    return Promise.resolve(list)
}

/**
 * 将api保存到dir路径下, 此保存为覆盖保存, dir下的文件会先被清除 
 */
export async function saveToDir(apidoc: ApiDoc[] | void) {
    if (!apidoc) return
    for (const api of apidoc) {
        try {
            const path = dir.concat("\\").concat(api.projectPath + "_apidoc.json")
            const content = JSON.stringify(api)
            console.log(2, path, content)
            writeTextFile(path, content)
        } catch (error) {
        }
    }
}
