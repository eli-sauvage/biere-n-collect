import { type Router } from "vue-router";
import { type Product, type Variation } from "./api/products";
import { validate_cart } from "./api/order";
import { f_price } from "./utils";

export type ProductId = number;



export class CartElement {
    product: Product
    variation: Variation
    quantity: number
    constructor(product: Product, variation: Variation) {
        this.product = product;
        this.variation = variation;
        this.quantity = 0
    }
    add(quantity: number) {
        let new_quantity = this.quantity + quantity;
        if (new_quantity >= 0) {
            this.quantity = new_quantity
        }
    }
};


export type CartElemWithSubtotal = {
  cart_element: CartElement,
  subtotal: number
};
export class Cart {
    elements: CartElement[] = []
    constructor(products: Product[]) {
        this.elements = products.map(prod=>prod.variations.map(variation=>new CartElement(prod, variation))).flat()
    }
    elems_with_subtotal(): CartElemWithSubtotal[] {
        return this.elements.filter(e => e.quantity != 0).map((e) => { return { cart_element: e, subtotal: e.variation.price_ttc * e.quantity } })
    }
    get_total(): string {
        return f_price(this.elements.reduce((acc, e) => acc + e.variation.price_ttc * e.quantity, 0));
    }
    async validate(router: Router) {
        if (this.elements.length == 0) return
        let order_id = await validate_cart(this);
        if (order_id != null)
            router.push({ path: "/checkout", query: { order_id: order_id } })
    }
}
