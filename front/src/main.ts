import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { createRouter, createWebHistory } from 'vue-router'

import Admin from './Admin.vue'
import Buy from './Buy.vue'
import SrCheckoutForm from './components/pay/SrCheckoutForm.vue'
import SrReturn from './components/pay/SrReturn.vue'

if(import.meta.env.VITE_SITE_URL == undefined){
     throw Error(`no SITE URL specified in env`)
}
if(import.meta.env.VITE_API_URL == undefined){
     throw Error(`no API URL specified in env`)
}else{
    let api_url = import.meta.env.VITE_API_URL as string;
    if(api_url.endsWith("/")){
        throw Error(`API URL must not end with '/' : ${api_url}`)
    }
    if(!api_url.match(/^https?:\/\//)){
        throw Error(`API URL must start with http:// or https:// : ${api_url}`)
    }
}


const routes = [
    { path: '/', component: Buy },
    { path: '/admin', component: Admin },
    { path: '/checkout', component: SrCheckoutForm },
    { path: '/return', component: SrReturn }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

createApp(App).use(router).mount('#app')
