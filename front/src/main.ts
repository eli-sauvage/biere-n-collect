import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { createRouter, createWebHistory } from 'vue-router'

import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import ToastService from 'primevue/toastservice'
import Toast from 'primevue/toast'
import ConfirmationService from 'primevue/confirmationservice'
import Button from 'primevue/button'
import ConfirmPopup from 'primevue/confirmpopup'
import Textarea from 'primevue/textarea'
import Panel from 'primevue/panel'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import DatePicker from 'primevue/datepicker'
import FloatLabel from 'primevue/floatlabel'
import Tag from 'primevue/tag'
import Accordion from 'primevue/accordion'
import AccordionContent from 'primevue/accordioncontent'
import AccordionHeader from 'primevue/accordionheader'
import AccordionPanel from 'primevue/accordionpanel'

import { definePreset } from '@primevue/themes'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import ToggleSwitch from 'primevue/toggleswitch'
import Divider from 'primevue/divider'
import Dialog from 'primevue/dialog'
import Select from 'primevue/select'
import ProgressSpinner from 'primevue/progressspinner'
import ColumnGroup from 'primevue/columngroup'
import Row from 'primevue/row'
import InputMask from 'primevue/inputmask'
import Tabs from 'primevue/tabs'
import Tab from 'primevue/tab'
import TabPanel from 'primevue/tabpanel'
import TabList from 'primevue/tablist'
import TabPanels from 'primevue/tabpanels'
import Drawer from 'primevue/drawer'

const routes = [
    { path: '/', component: () => import('./Home.vue') },
    { path: '/serveur', component: () => import('./Serveur.vue') },
    { path: '/admin', component: () => import('./Admin.vue') },
    { path: '/admin/report', component: () => import('./AdminReport.vue') },
    { path: '/login', component: () => import('./components/Login.vue') },
    {
        path: '/checkout',
        component: () => import('./components/pay/SrCheckoutForm.vue'),
    },
    {
        path: '/return',
        component: () => import('./components/pay/SrReturn.vue'),
    },
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
            950: '{yellow.950}',
        },
    },
})

const app = createApp(App)
app.use(router)
app.use(PrimeVue, {
    theme: {
        preset: MyPreset,
        options: {
            darkModeSelector: '.dark-mode',
        },
    },
})

app.component('Button', Button)
app.component('ConfirmPopup', ConfirmPopup)
app.component('Textarea', Textarea)
app.component('Panel', Panel)
app.component('DataTable', DataTable)
app.component('Column', Column)
app.component('DatePicker', DatePicker)
app.component('FloatLabel', FloatLabel)
app.component('Tag', Tag)
app.component('InputText', InputText)
app.component('InputNumber', InputNumber)
app.component('ToggleSwitch', ToggleSwitch)
app.component('Divider', Divider)
app.component('Dialog', Dialog)
app.component('Accordion', Accordion)
app.component('AccordionContent', AccordionContent)
app.component('AccordionHeader', AccordionHeader)
app.component('AccordionPanel', AccordionPanel)
app.component('Select', Select)
app.component('ProgressSpinner', ProgressSpinner)
app.component('DataTable', DataTable)
app.component('ColumnGroup', ColumnGroup)
app.component('Row', Row)
app.component('InputMask', InputMask)
app.component('Tabs', Tabs)
app.component('Tab', Tab)
app.component('TabPanel', TabPanel)
app.component('TabList', TabList)
app.component('TabPanels', TabPanels)
app.component('Drawer', Drawer)
app.component('Toast', Toast)

app.use(ConfirmationService)
app.use(ToastService)
app.mount('#app')
