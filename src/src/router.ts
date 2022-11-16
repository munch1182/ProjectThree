import { createRouter, createWebHashHistory } from 'vue-router'
import Main from './components/Main.vue'
import Home from './components/page/Home.vue'
const Pic = () => import('./components/page/Pic.vue')
const About = () => import('./components/page/About.vue');

const routes = [
    { path: '/', redirect: '/main/home' },
    {
        path: '/main', component: Main,
        children: [
            { path: "home", component: Home, meta: { title: "首页" } },
            { path: "pic", component: Pic, meta: { title: "图片" } },
            { path: "about", component: About, meta: { title: "关于" } },
        ]
    }
]

export default createRouter({
    history: createWebHashHistory(),
    routes,
})