import type { Cart } from "../cart";
import { base, Error } from "./api"


export async function get_stripe_pub_key(): Promise<string | null> {
  let url = `${base}/get_stripe_pub_key`
  let error_title = "Erreur lors de la récupération de la clé d'API stripe"
  try {
    let res = await fetch(url).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res.publishable_key as string
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}

export async function validate_cart(cart: Cart): Promise<number | null> {
  let url = `${base}/validate_cart`
  let error_title = "Erreur lors de la valiation du panier"
  try {
    let res = await fetch(url,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(
          {
            elements: cart.elements.map((el) => {
              return { variation_id: el.variation.id, quantity: el.quantity }
            })
          }
        )
      }
    ).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res.order_id as number
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}


export type PaymentInfos = {
  client_secret: string,
  total_price: number
}
export async function get_payment_infos(order_id: number): Promise<PaymentInfos | null> {
  let url = `${base}/get_payment_infos?order_id=${encodeURIComponent(order_id)}`
  let error_title = "Erreur lors de la récupération des informations de paiement"
  try {
    let res = await fetch(url).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res as PaymentInfos
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}

export async function set_email(client_secret: string, email: string): Promise<boolean> {
  let url = `${base}/set_email?client_secret=${encodeURIComponent(client_secret)}&email=${encodeURIComponent(email)}`
  let error_title = "Erreur lors de l'envoi de l'adresse mail"
  try {
    let res = await fetch(url, { method: "PATCH" }).then(e => e.json());
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

export type PaymentStatusDetails = {
  variation_name: string, product_name: string, quantity: number, subtotal_ht: number, subtotal_ttc: number
}

export type PaymentStatus = {
  status: "canceled" | "processing" | "requiresAction" | "requiresCapture" | "requiresConfirmation" | "requiresPaymentMethod" | "succeeded"
  receipt?: string,
  email?: string,
  detail: PaymentStatusDetails[],
  total_price: number
}

export async function get_payment_status(client_secret: string): Promise<PaymentStatus | null> {
  let url = `${base}/get_payment_status?client_secret=${encodeURIComponent(client_secret)}`
  let error_title = "Erreur lors de la récupération du paiment, verifiez vos mails"
  try {
    let res = await fetch(url).then(e => e.json());
    if (res.error) {
      new Error(error_title, res.error)
      return null
    } else {
      return res as PaymentStatus
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return null
  }
}

export function get_qr_code_url(client_secret: string): string {
  return `${base}/get_qr_code?client_secret=${encodeURIComponent(client_secret)}`
}
