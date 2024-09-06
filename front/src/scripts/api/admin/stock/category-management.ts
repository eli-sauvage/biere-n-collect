import { base, Error } from "../../api";

export async function create_category(name: string): Promise<boolean> {
    let url = `${base}/admin/stock/categories?name=${encodeURIComponent(name)}`;
    let error_title = "Erreur lors de la maj d'une variation"
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


export async function delete_category(category_id: number): Promise<boolean> {
    let url = `${base}/admin/stock/categories?category_id=${encodeURIComponent(category_id)}`;
    let error_title = "Erreur lors de la suppression d'une variation"
    try {
        let res = await fetch(url, {
            method: "DELETE",
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


export async function edit_category_name(category_id: number, name: string): Promise<boolean> {
    let url = `${base}/admin/stock/categories/edit_name?category_id=${encodeURIComponent(category_id)}` +
        `&name=${encodeURIComponent(name)}`;
    let error_title = "Erreur lors de la maj d'une variation"
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