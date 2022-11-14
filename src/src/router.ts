import { createRouter, createWebHashHistory } from 'vue-router'
import Main from './components/Main.vue'
import Home from './components/Home.vue'

const routes = [
    { path: '/', redirect: '/main/home' },
    {
        path: '/main', component: Main,
        children: [
            { path: "home", component: Home, meta: { title: "首页" } },
        ]
    }
]

export default createRouter({
    history: createWebHashHistory(),
    routes,
})