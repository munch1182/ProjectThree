import { createRouter, createWebHistory } from "vue-router";
import Main from "../components/Main.vue"
import Home from "../components/page/Home.vue"
import NotFund from "../components/page/NotFund.vue"
import About from "../components/page/About.vue"
import Set from "../components/page/Set.vue"
import SetProxy from "../components/page/SetProxy.vue"

export const routes = [
    { path: '/', redirect: '/main/home' },
    {
        path: '/main', component: Main,
        children: [
            { path: 'home', component: Home, meta: { title: "首页" } },
            { path: '404', component: NotFund },
            {
                path: 'set', component: Set, meta: { title: "设置" },
                // children: [
                //     { path: 'proxy', component: SetProxy, meta: { title: "代理" } }
                // ]
            },
            { path: 'proxy', component: SetProxy, meta: { title: "代理" } },
            { path: 'about', component: About, meta: { title: "关于" } },
            { path: '*', redirect: '404' }
        ],
    },
    { path: '/:pathMatch(.*)*', redirect: '/main/404' }
]

export default createRouter({
    history: createWebHistory(),
    routes,
})