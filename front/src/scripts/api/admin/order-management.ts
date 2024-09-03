import { base, Error } from "../api";

export type Order = {
  id: number,
  timestamp: number,
  user_email: string,
  receipt?: string,
  payment_intent_id: string,
  served: boolean
  total_price: number
}

export async function get_all(): Promise<Order[]> {
  let url = `${base}/admin/orders/get_all`;
  let error_title = "Erreur lors de la récupération des commandes"
  try {
    let res = await fetch(url, {
      credentials: "include",
    }).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return []
    } else {
      return res as Order[]
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return []
  }
}
