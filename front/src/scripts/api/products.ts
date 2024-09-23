import { base, Error } from "./api"

export type Product = {
  id: number,
  name: string,
  description: string,
  stock_quantity: number,
  variations: Variation[]
}
export type Variation = {
    id: number,
    name: string,
    product_id: number,
    price_ht: number,
    price_ttc: number,
    tva: number,
    volume: number
    available_to_order: boolean
}
export type BarStatus = {
  is_open: boolean,
  closed_message?: string
}

export async function get_bar_status(): Promise<BarStatus | null> {
  let url = `${base}/get_bar_status`;
  let error_title = "Erreur lors de la recupération de l'ouverture du bar";
  try {
    let res = await fetch(url).then(async e => await e.json())
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res as BarStatus
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}


export async function get_available_products(): Promise<Product[]> {
  let url = `${base}/get_available_products`;
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

