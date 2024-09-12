import { base, Error } from "../../api";


export type VariationEdition = {
    new_name?: string,
    new_price_ht?: number,
  new_tva?: number,
    new_volume?: number,
    new_available_to_order?: boolean
}

export async function edit_variation(variation_id: number, edition: VariationEdition): Promise<boolean> {
    let url = `${base}/admin/stock/variations/edit?variation_id=${encodeURIComponent(variation_id)}` +
        `&new_name=${encodeURIComponent(edition.new_name ? edition.new_name : "")}` +
        `&new_price_ht=${encodeURIComponent(edition.new_price_ht ? edition.new_price_ht : "")}` +
        `&new_tva=${encodeURIComponent(edition.new_tva ? edition.new_tva : "")}` +
        `&new_available_to_order=${encodeURIComponent(edition.new_available_to_order != null ? encodeURIComponent(edition.new_available_to_order) : "")}` +
        `&new_volume=${encodeURIComponent(edition.new_volume ? edition.new_volume : "")}`;
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
