import { base, Error } from "../api";

export type Order = {
  id: number,
  timestamp: number,
  user_email: string,
  receipt?: string,
  payment_intent_id: string,
  served: boolean
  total_price: number
  detail: { name: string, quantity: number, subtotal: number }[]
}

export async function get_orders(email: string | null, date: [Date, Date] | null, receipt: string | null): Promise<Order[]> {
  let url = `${base}/admin/orders/search?email=${encodeURIComponent(email || "")}&date_begin=${date ? encodeURIComponent(date[0].getTime()) : ""}&date_end=${date ? encodeURIComponent(date[1].getTime()) : ""}&receipt=${encodeURIComponent(receipt || "")}`;
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

export async function get_order_by_id(id: number): Promise<Order | null> {
  let url = `${base}/admin/orders?id=${encodeURIComponent(id)}`;
  let error_title = "Erreur lors de la récupération de la commande"
  try {
    let res = await fetch(url, {
      credentials: "include",
    }).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res as Order
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}

export async function get_order_by_receipt(receipt: string): Promise<Order | null> {
  let url = `${base}/admin/orders/by_receipt?receipt=${encodeURIComponent(receipt)}`;
  let error_title = "Erreur lors de la récupération de la commande"
  try {
    let res = await fetch(url, {
      credentials: "include",
    }).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res as Order
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}

export async function set_served(order_id: number, new_served: boolean): Promise<boolean> {
  let url = `${base}/admin/orders/set_served?order_id=${encodeURIComponent(order_id)}&new_served=${encodeURIComponent(new_served)}`;
  let error_title = "Erreur lors de la maj de la commande"
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