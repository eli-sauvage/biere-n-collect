import { base, Error } from "../api";
import type { Product } from "../order";

export async function insert_stock(new_prod: Product): Promise<boolean> {
    let url = `${base}/admin/stock` +
        `?name=${encodeURIComponent(new_prod.name)}` +
        `&price=${encodeURIComponent(new_prod.price)}` +
        `&quantity=${encodeURIComponent(new_prod.quantity)}` +
        `&available=${encodeURIComponent(new_prod.available)}`;
    let error_title = "Erreur lors de la création d'un nouveau produit"
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

export async function update_stock(new_stock: Product): Promise<boolean> {
    let url = `${base}/admin/stock`
    console.log(JSON.stringify(new_stock))
    let error_title = "Erreur lors de la maj du stock"
    try {
        let res = await fetch(url, {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(new_stock),
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

export async function delete_stock(product_id: number): Promise<boolean> {
    let url = `${base}/admin/stock?product_id=${encodeURIComponent(product_id)}`;
    let error_title = "Erreur lors de la suppréssion d'un produit"
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

export async function move_stock(product_id: number, direction: "up" | "down"): Promise<boolean> {
    let url = `${base}/admin/stock/move?product_id=${encodeURIComponent(product_id)}&direction=${encodeURIComponent(direction)}`;
    let error_title = "Erreur lors du déplacement d'un produit dans la liste"
    try {
        let res = await fetch(url, {
            method: "PATCH",
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