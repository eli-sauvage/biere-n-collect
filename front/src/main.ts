import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { createRouter, createWebHistory } from 'vue-router'

import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import ToastService from "primevue/toastservice"
import { definePreset } from '@primevue/themes';
import Buy from './Buy.vue';

if (import.meta.env.VITE_SITE_URL == undefined) {
    throw Error(`no SITE URL specified in env`)
}
if (import.meta.env.VITE_API_URL == undefined) {
    throw Error(`no API URL specified in env`)
} else {
    let api_url = import.meta.env.VITE_API_URL as string;
    if (api_url.endsWith("/")) {
        throw Error(`API URL must not end with '/' : ${api_url}`)
    }
    if (!api_url.match(/^https?:\/\//)) {
        throw Error(`API URL must start with http:// or https:// : ${api_url}`)
    }
}


const routes = [
    { path: '/', component: () => import('./Buy.vue') },
    { path: '/', component: Buy},
    { path: '/admin', component: () => import("./Admin.vue") },
    { path: '/checkout', component: () => import("./components/pay/SrCheckoutForm.vue") },
    { path: '/return', component: () => import("./components/pay/SrReturn.vue") }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

const MyPreset = definePreset(Aura, {
    semantic: {
        primary: {
            50: '#e2b42c',
            100: '#e2b42c',
            200: '#e2b42c',
            300: '#e2b42c',
            400: '#e2b42c',
            500: '#e2b42c',
            600: '#e2b42c',
            700: '#e2b42c',
            800: '#e2b42c',
            900: '#e2b42c',
            950: '#e2b42c'
        }
    }
});


createApp(App)
    .use(router)
    .use(
        PrimeVue, {
            theme: {
                preset: MyPreset
            }
        }
    )
    .use(ToastService)
    .mount('#app')
