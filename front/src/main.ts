import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { createRouter, createWebHistory } from 'vue-router'

import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import ToastService from "primevue/toastservice";
import ConfirmationService from 'primevue/confirmationservice';
import { definePreset } from '@primevue/themes';


const routes = [
    { path: '/', component: () => import('./Home.vue') },
    { path: '/serveur', component: () => import("./Serveur.vue") },
    { path: '/admin', component: () => import("./Admin.vue") },
    { path: '/admin/report', component: () => import("./AdminReport.vue") },
    { path: '/login', component: () => import("./components/Login.vue") },
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
            50: '{yellow.50}',
            100: '{yellow.100}',
            200: '{yellow.200}',
            300: '{yellow.300}',
            400: '{yellow.400}',
            500: '{yellow.500}',
            600: '{yellow.600}',
            700: '{yellow.700}',
            800: '{yellow.800}',
            900: '{yellow.900}',
            950: '{yellow.950}'
        }
    }
});


createApp(App)
    .use(router)
    .use(
        PrimeVue, {
            theme: {
                preset: MyPreset,
                options: {
                    darkModeSelector: '.dark-mode',
                }
            }
        }
    )
    .use(ConfirmationService)
    .use(ToastService)
    .mount('#app')
