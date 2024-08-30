import type { Cart } from "../cart";
import { base, Error } from "./api"

export type Product = {
    name: string,
    quantity: number,
    product_id: number,
    price: number,
    available: boolean,
}

export async function get_stock(): Promise<Product[]> {
    let url = `${base}/order/get_available_stock`;
    let error_title = "Erreur lors de la recupération du stock";
    try {
        let res = await fetch(url).then(async e => await e.json())
        if (res.error) {
            new Error(error_title, res.error)
            return []
        } else {
            return res as Product[]
        }
    } catch (e: any) {
        new Error(error_title, e.toString());
        return []
    }
}

export async function validate_cart(cart: Cart, email: string): Promise<number | null> {
    let url = `${base}/order/validate_cart`
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
                        email: email,
                        elements: cart.elements.map((el) => {
                            return { product_id: el.product.product_id, quantity: el.quantity }
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
    let url = `${base}/order/get_payment_infos?order_id=${encodeURIComponent(order_id)}`
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


export type PaymentStatus = {
    status: "canceled" | "processing" | "requiresAction" | "requiresCapture" | "requiresConfirmation" | "requiresPaymentMethod" | "succeeded"
    receipt?: string,
    qr_code?: string
}

export async function get_payment_status(payment_intent_id: string, client_secret: string): Promise<PaymentStatus | null> {
    let url = `${base}/order/get_payment_status?payment_intent_id=${encodeURIComponent(payment_intent_id)}&client_secret=${encodeURIComponent(client_secret)}`
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

export  function get_qr_code_url(payment_intent_id: string, client_secret: string): string{
    return `${base}/order/get_qr_code?payment_intent_id=${encodeURIComponent(payment_intent_id)}&client_secret=${encodeURIComponent(client_secret)}`
}