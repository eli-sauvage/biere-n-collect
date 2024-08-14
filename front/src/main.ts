import './assets/main.css'

import { compile, createApp } from 'vue'
import App from './App.vue'

import { createMemoryHistory, createRouter, createWebHistory } from 'vue-router'

import Admin from './Admin.vue'
import Buy from './Buy.vue'
import SrCheckoutForm from './components/pay/SrCheckoutForm.vue'
import SrReturn from './components/pay/SrReturn.vue'

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
