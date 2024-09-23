import { base, Error } from "../../api";
import type { Product } from "../../products";

export async function get_all_products(): Promise<Product[]> {
  let url = `${base}/admin/stock/products/get_all`;
  let error_title = "Erreur lors de la recupération du stock";
  try {
    let res = await fetch(url).then(async e => await e.json())
    if (res.error) {
      new Error(error_title, res.error)
      return []
    } else {
      let prods = res as Product[];
      for(let i=0; i<prods.length; i++){
        for(let j=0; j<prods[i].variations.length; j++){
          prods[i].variations[j].price_ttc =  prods[i].variations[j].price_ht * (1 + prods[i].variations[j].tva)
        }
      }
      return prods
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return []
  }
}
export async function insert_product(new_prod: Product): Promise<boolean> {
    let url = `${base}/admin/stock/products` +
        `?name=${encodeURIComponent(new_prod.name)}` +
        `&description=${encodeURIComponent(new_prod.description)}` +
        `&stock_quantity=${encodeURIComponent(new_prod.stock_quantity)}`;
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

export type ProductEdition = {
    new_name?: string,
    new_description?: string,
    new_stock_quantity?: number,
}
export async function edit_product(
    product_id: number,
    edition: ProductEdition
): Promise<boolean> {
    let url = `${base}/admin/stock/products?product_id=${encodeURIComponent(product_id)}` +
        `&new_name=${encodeURIComponent(edition.new_name ? edition.new_name : "")}` +
        `&new_description=${encodeURIComponent(edition.new_description ? edition.new_description : "")}` +
        `&new_stock_quantity=${encodeURIComponent(edition.new_stock_quantity ? edition.new_stock_quantity : "")}`;
    let error_title = "Erreur lors de la maj du stock"
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


export async function delete_product(product_id: number): Promise<boolean> {
    let url = `${base}/admin/stock/products?product_id=${encodeURIComponent(product_id)}`;
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

export async function move_product(product_id: number, direction: "up" | "down"): Promise<boolean> {
    let url = `${base}/admin/stock/products/move?product_id=${encodeURIComponent(product_id)}&direction=${encodeURIComponent(direction)}`;
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


export async function add_variation(product_id: number, name: string, price_ht: number, tva: number, volume: number, available_to_order: boolean): Promise<boolean> {
    let url = `${base}/admin/stock/products/add_variation?product_id=${encodeURIComponent(product_id)}` +
        `&name=${encodeURIComponent(name)}` +
        `&price_ht=${encodeURIComponent(price_ht)}` +
        `&tva=${encodeURIComponent(tva)}` +
        `&available_to_order=${encodeURIComponent(available_to_order)}` +
        `&volume=${encodeURIComponent(volume)}`;
    let error_title = "Erreur lors de l'ajout d'une variation"
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

export async function remove_variation(product_id: number, variation_id: number): Promise<boolean> {
    let url = `${base}/admin/stock/products/remove_variation?product_id=${encodeURIComponent(product_id)}` +
        `&variation_id=${encodeURIComponent(variation_id)}`;
    let error_title = "Erreur lors du retrait d'une variation"
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
