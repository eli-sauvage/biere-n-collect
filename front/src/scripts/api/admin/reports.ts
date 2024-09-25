import { base, Error } from '../api'

export type Bar = {
    is_open: boolean
    open_since: Date
    closing_message: string
}

export type BarOpening = {
    begin: Date
    end: Date
}

export async function get_bar_openings(): Promise<BarOpening[]> {
    let url = `${base}/admin/reports/get_bar_openings`
    let error_title =
        'Erreur lors de la récupération des anciennes ouvertures du bar'
    try {
        let res = await fetch(url, {
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            res = (res as any[]).map((e) => {
                return { begin: new Date(e.begin), end: new Date(e.end) }
            })
            return res
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return []
    }
}

export type ReportItem = {
    item_name: string
    quantity: number
    tva: number
    subtotal_ht: number
    subtotal_ttc: number
}

export async function get_report(
    begin: Date,
    end: Date
): Promise<ReportItem[]> {
    let url =
        `${base}/admin/reports?begin=${encodeURIComponent(begin.getTime())}` +
        `&end=${encodeURIComponent(end.getTime())}`
    let error_title = "Erreur lors de l'envoi de la récupération du rapport"
    try {
        let res = await fetch(url, {
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            return res as ReportItem[]
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return []
    }
}

export async function close_bar(): Promise<boolean> {
    let url = `${base}/admin/bar/close`
    let error_title =
        "Erreur lors de l'envoi de la commande de fermeture du bar"
    try {
        let res = await fetch(url, {
            method: 'POST',
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return false
        } else {
            return true
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return false
    }
}

export async function set_closing_message(message: string): Promise<boolean> {
    let url = `${base}/admin/bar/set_closing_message?closing_message=${encodeURIComponent(message)}`
    let error_title = 'Erreur lors du changement de message de fermeture'
    try {
        let res = await fetch(url, {
            method: 'POST',
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return false
        } else {
            return true
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return false
    }
}

export async function list_reports(): Promise<string[]> {
    let url = `${base}/admin/bar/list_reports`
    let error_title = 'Erreur lors du chargement des rapports'
    try {
        let res = await fetch(url, {
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            return res as string[]
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return []
    }
}
