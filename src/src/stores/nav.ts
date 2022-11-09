import { defineStore } from 'pinia'
import { computed, Ref, ref } from 'vue'

const NAME = "nav"

export class Last {
    readonly id: number
    time: number
    desc: string

    constructor(id: number, time: number, desc: string) {
        this.id = id
        this.time = time
        this.desc = desc
    }
}

export class Nav {
    readonly id: number //唯一id
    readonly name: string //显示为标题
    readonly router: string //子路由
    last: Last | null = null //需要显示的省略内容
    showNow: boolean

    constructor(id: number, name: string, router: string) {
        this.id = id
        this.name = name
        this.router = router
        this.showNow = false
    }

    fullRouter(): string {
        return "/main".concat(this.router)
    }
}

const initNavs: Nav[] = [
    new Nav(1, "首页", "/home"),
    new Nav(2, "关于", "/about"),
    new Nav(3, "test", "/test"),
]

export const useNavStore = defineStore(NAME, () => {
    const _navs: Ref<Nav[]> = ref([])

    initNavs.forEach((v) => _navs.value.push(v))

    const first = _navs.value[0]
    first.showNow = true
    const title = ref(first.name)

    /**
     * 
     * @param nav 动态添加一个nav
     */
    function addNav(nav: Nav) {
        _navs.value.push(nav)
    }

    /**
     * 动态移除一个nav
     */
    function removeNav(nav: Nav) {
        const index = _navs.value.indexOf(nav)
        if (index <= -1) return
        _navs.value.splice(index, 1)
    }

    /**
     * 
     * @param nav 将当前显示的nav变更为这个nav 
     */
    function show(nav: Nav) {
        const index = find(nav)
        if (index <= -1) return

        const curr = _navs.value.find((nav) => nav.showNow)
        if (curr) {
            curr.showNow = false
        }

        nav.showNow = true
        title.value = nav.name
    }

    function find(nav: Nav): number {
        return _navs.value.findIndex((n) => nav.id == n.id)
    }

    const get = computed(() => _navs.value)

    /**
     * @param fullPath 通过路径值变更显示的nav
     */
    function updateNav(fullPath: string) {
        const nav = _navs.value.find((n) => n.fullRouter() == fullPath)
        if (!nav) return
        show(nav)
    }

    const main = computed(() => _navs.value[0])

    return { get, addNav, removeNav, show, updateNav, title, main }
})