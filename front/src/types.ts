import { routerKey, useRouter, type Router } from "vue-router";

export type ProductId = number;

export type Product = {
    product_id: ProductId,
    name: string,
    quantity: number,
    price: number,
    available: boolean
};

export type User = {
    email: string,
    role: "admin" | "waiter",
    sessions: number
}

export class CartElement {
    product: Product
    quantity: number
    constructor(product: Product) {
        this.product = product;
        this.quantity = 0
    }
    add(quantity: number) {
        let new_quantity = this.quantity + quantity;
        if (new_quantity >= 0) {
            this.quantity = new_quantity
        }
    }
};

export class Cart {
    elements: CartElement[] = []
    constructor(products: Product[]) {
        this.elements = products.map(e => new CartElement(e))
    }
    elems_with_subtotal(): { cart_element: CartElement, subtotal: number }[] {
        return this.elements.filter(e=>e.quantity != 0).map((e) => { return { cart_element: e, subtotal: e.product.price * e.quantity } })
    }
    get_total(): string {
        return f_price(this.elements.reduce((acc, e) => acc + e.product.price * e.quantity, 0));
    }
    async validate(router: Router, email: string) {
        if (this.elements.length == 0) return
        try {
            let res = await fetch(`${import.meta.env.VITE_API_URL}/order/validate_cart`,
                {
                    method: "POST",
                    body: JSON.stringify(
                        {
                            email: email,
                            elements: this.elements.map((el) => {
                                return { product_id: el.product.product_id, quantity: el.quantity }
                            })
                        }
                    )
                }
            );
            if (res.ok) {
                let json = await res.json();
                console.log(json.order_id)
                router.push({ path: "/checkout", query: { order_id: json.order_id } })
            } else {
                console.log(`erreur de validation : ${await res.text()}`)
            }
        } catch (e: any) {
            console.error(e)
        }
    }
}

export function f_price(price:number): string{
    return (price/100).toFixed(2) + " €";
}