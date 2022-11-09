import { createRouter, createWebHistory } from "vue-router";
import Main from "../components/Main.vue"
import Home from "../components/page/Home.vue"
import NotFund from "../components/page/NotFund.vue"
import About from "../components/page/About.vue"

const routes = [
    { path: '/', redirect: '/main/home' },
    {
        path: '/main', component: Main,
        children: [
            { path: 'home', component: Home },
            { path: '404', component: NotFund },
            { path: 'about', component: About },
            { path: '*', redirect: '404' }
        ],
    },
    { path: '/aaaa', component: Main },
    { path: '/:pathMatch(.*)*', redirect: '/main/404' }
]

export default createRouter({
    history: createWebHistory(),
    routes,
})