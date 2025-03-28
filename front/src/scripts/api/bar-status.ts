import { base, Error } from './api'

export type BarStatus = {
    is_open: boolean
    closed_message?: string
}

export async function get_bar_status(): Promise<BarStatus | null> {
    let url = `${base}/get_bar_status`
    let error_title = "Erreur lors de la recupération de l'ouverture du bar"
    try {
        let res = await fetch(url).then(async (e) => await e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return null
        } else {
            return res as BarStatus
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return null
    }
}

export type BarStatusAsServeur = BarStatus & { open_since: Date }

export async function get_bar_status_as_serveur(): Promise<BarStatusAsServeur | null> {
    let url = `${base}/get_bar_status`
    let error_title = "Erreur lors de la recupération de l'ouverture du bar"
    try {
        let res = await fetch(url, {
            credentials: 'include',
        }).then(async (e) => await e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return null
        } else {
            return res as BarStatusAsServeur
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return null
    }
}
