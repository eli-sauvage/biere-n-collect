import { base, Error } from '../api'

export type User = {
    email: string
    role: 'admin' | 'waiter'
    active_sessions: number
}

export async function get_all_users(): Promise<User[]> {
    let url = `${base}/admin/users/get_all`
    let error_title =
        'Erreur lors de la récupération de la liste des utilisateurs'
    try {
        let res = await fetch(url, {
            credentials: 'include',
        }).then((e) => e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            return res as User[]
        }
    } catch (e: any) {
        new Error(error_title, e.toString())
        return []
    }
}
export async function add_user(
    email: string,
    role: 'admin' | 'waiter'
): Promise<boolean> {
    let url = `${base}/admin/users?email=${encodeURIComponent(email)}&role=${encodeURIComponent(role)}`
    let error_title = "Erreur lors de l'ajout d'un utilisateur"
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

export async function delete_user(email: string): Promise<boolean> {
    let url = `${base}/admin/users?email=${encodeURIComponent(email)}`
    let error_title = "Erreur lors de la suppression d'un utilisateur"
    try {
        let res = await fetch(url, {
            method: 'DELETE',
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

export async function update_role(
    email: string,
    new_role: 'admin' | 'waiter'
): Promise<boolean> {
    let url = `${base}/admin/users/update_role?email=${encodeURIComponent(email)}&new_role=${encodeURIComponent(new_role)}`
    let error_title = "Erreur lors de la maj d'un utilisateur"
    try {
        let res = await fetch(url, {
            method: 'PATCH',
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

export async function disconnect_user(email: string): Promise<boolean> {
    let url = `${base}/admin/users/disconnect?email=${encodeURIComponent(email)}`
    let error_title = "Erreur lors de la déconnexion d'un utilisateur"
    try {
        let res = await fetch(url, {
            method: 'PATCH',
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
