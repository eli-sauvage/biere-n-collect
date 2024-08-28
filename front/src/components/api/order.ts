import { base, type Error } from "./api"

export type Stock = {
    name: String,
    quantity: number,
    product_id: number,
    price: number,
    available: boolean,
}

export async function get_stock(): Promise<Stock[] | Error> {
    let url = `${base}/order/get_available_stock`;
    try {
        let res = await fetch(url).then(async e => await e.json())
        if (res.error) {
            return { message: res.error } as Error
        } else {
            return res as Stock[]
        }
    } catch (e) {
        return { message: e } as Error
    }
}