import type { ToastServiceMethods } from "primevue/toastservice"

export let base = import.meta.env.VITE_API_URL

let toast: null | ToastServiceMethods = null;
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


export async function get_stripe_pub_key(): Promise<string | null> {
    let url = `${base}/config`
    let error_title = "Erreur lors de la récupération de la clé d'API stripe"
    try {
        let res = await fetch(url).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return null
        } else {
            return res.publishable_key as string
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return null
    }
}