import { invoke } from '@tauri-apps/api'

/**
 * 将js日志传递到rust中实时显示 
 */
export default function log(content: string) {
    console.log(content)
    invoke<string>("log_from_js", { content })
        .then()
        .catch(e => console.log(e))
}