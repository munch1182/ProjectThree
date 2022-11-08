import { createRouter, createWebHistory } from "vue-router";
import Main from "../components/Main.vue"

const routes = [
    { path: '/', component: Main },
    { path: '/aaaa', component: Main }
]

export default createRouter({
    history: createWebHistory(),
    routes,
})