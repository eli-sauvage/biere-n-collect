import { type Router } from "vue-router";
import { validate_cart, type Product } from "./api/order";
import { f_price } from "./utils";

export type ProductId = number;



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
        return this.elements.filter(e => e.quantity != 0).map((e) => { return { cart_element: e, subtotal: e.product.price * e.quantity } })
    }
    get_total(): string {
        return f_price(this.elements.reduce((acc, e) => acc + e.product.price * e.quantity, 0));
    }
    async validate(router: Router, email: string) {
        if (this.elements.length == 0) return
        let order_id = await validate_cart(this, email);
        if (order_id != null)
            router.push({ path: "/checkout", query: { order_id: order_id } })
    }
}