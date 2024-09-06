import { base, Error } from "../../api";


export async function edit_variation(variation_id: number, new_name?: string, new_price?: number, new_volume?: number): Promise<boolean> {
    let url = `${base}/admin/stock/variations/edit?variation_id=${encodeURIComponent(variation_id)}` +
        `&new_name=${encodeURIComponent(new_name ? new_name : "")}`+
        `&new_price=${encodeURIComponent(new_price ? new_price : "")}`+
        `&new_volume=${encodeURIComponent(new_volume ? new_volume : "")}`;
    let error_title = "Erreur lors de la maj d'une variation"
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