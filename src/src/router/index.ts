import { createRouter, createWebHistory } from "vue-router";
import Main from "../components/Main.vue"
import Home from "../components/page/Home.vue"
const NotFund = () => import("../components/page/NotFund.vue")
const About = () => import("../components/page/About.vue")
const Set = () => import("../components/page/Set.vue")
const SetProxy = () => import("../components/page/SetProxy.vue")
const Bookmark = () => import("../components/page/Bookmark.vue")
const ApiDoc = () => import("../components/page/apidoc/ApiDoc.vue")

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
            { path: 'bookmark', component: Bookmark, meta: { title: "书签" } },
            { path: 'apidoc', component: ApiDoc, meta: { title: "接口" } },
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