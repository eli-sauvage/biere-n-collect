import type { ToastServiceMethods } from "primevue/toastservice"

export let base = import.meta.env.VITE_API_URL

export let toast: null | ToastServiceMethods = null;
export function set_toast(t: ToastServiceMethods){
    toast = t
}


export class Error{
    titre: string;
    message: string;
    constructor(titre: string, message: string){
        this.message = message
        this.titre = titre
        if(toast != null){
            toast.add({ severity: 'error', summary:this.titre, detail:this.message, life: 3000 })
        }
    }
}


