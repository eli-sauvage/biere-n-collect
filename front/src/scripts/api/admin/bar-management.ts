import { base, Error, toast } from "../api";

export type Bar = {
    is_open: boolean,
    open_since: Date,
    closing_message: string
}


export async function get_bar(): Promise<Bar | null> {
    let url = `${base}/admin/bar`;
    let error_title = "Erreur lors de la récupération de l'état du bar"
    try {
        let res = await fetch(url, {
            credentials: "include",
        }).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return null
        } else {
            res.open_since = new Date(res.open_since)
            return res as Bar
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return null
    }
}

export async function open_bar(): Promise<boolean> {
    let url = `${base}/admin/bar/open`;
    let error_title = "Erreur lors de l'envoi de la commande d'ouverture du bar"
    try {
        let res = await fetch(url, {
            method: "POST",
            credentials: "include",
        }).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return false
        } else {
            return true
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return false
    }
}

export async function close_bar(): Promise<boolean> {
    let url = `${base}/admin/bar/close`;
    let error_title = "Erreur lors de l'envoi de la commande de fermeture du bar"
    try {
        let res = await fetch(url, {
            method: "POST",
            credentials: "include",
        }).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return false
        } else {
            return true
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return false
    }
}

export async function set_closing_message(message: string): Promise<boolean> {
    let url = `${base}/admin/bar/set_closing_message?closing_message=${encodeURIComponent(message)}`;
    let error_title = "Erreur lors du changement de message de fermeture"
    try {
        let res = await fetch(url, {
            method: "POST",
            credentials: "include",
        }).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return false
        } else {
            return true
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return false
    }
}


export async function list_reports(): Promise<string[]>{
    let url = `${base}/admin/bar/list_reports`;
    let error_title = "Erreur lors du chargement des rapports"
    try {
        let res = await fetch(url, {
            credentials: "include",
        }).then(e => e.json());
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            return res as string[]
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return []
    }

}