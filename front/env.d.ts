/// <reference types="vite/client" />
import { Router } from 'vue-router'
declare module '@vue/runtime-core' {
    interface ComponentCustomProperties {
        $router: Router
    }
}

declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    const component: DefineComponent<object, object, any>
    export default component
}
