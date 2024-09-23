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
    setQuantity(new_quantity: number){
      if(new_quantity >= 0){
        this.quantity = new_quantity
      }
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
    try{
      let rawOldCart = window.localStorage.getItem("cart");
      if(!rawOldCart)return
      let save = JSON.parse(rawOldCart) as {savedAt: number, cart: Cart};
      if(!save.savedAt || Date.now() - save.savedAt > 1 * 60 * 60 * 1000) //1h
        return
      let oldCart = save.cart;
      if(oldCart.elements && Array.isArray(oldCart.elements)){
        for(let oldElement of oldCart.elements){
          if(!oldElement.variation || !oldElement.variation.id || !oldElement.quantity)
            continue
          let elementIndex = this.elements.findIndex(e=>e.variation.id == oldElement.variation.id)
          if(elementIndex != -1){
            this.elements[elementIndex].quantity = oldElement.quantity;
          }
        };
      }
    }
    catch{};
  }
  updateCache(){
    window.localStorage.setItem("cart", JSON.stringify({cart: this, savedAt: Date.now()}))
  }
    getElement(variation_id: number){
      return this.elements.find(e => e.variation.id == variation_id) as CartElement
    }
    elems_with_subtotal(): CartElemWithSubtotal[] {
        return this.elements
          .filter(e => e.quantity != 0)
          .map((e) => { return { cart_element: e, subtotal: e.variation.price_ttc * e.quantity } })
    }
    get_total(): string {
        return f_price(this.elements.reduce((acc, e) => acc + e.variation.price_ttc * e.quantity, 0));
    }
  async validate(router: Router): Promise<boolean>{
    if (this.elements.find(e=>e.quantity>0) == undefined) return false

    let order_id = await validate_cart(this);
    if (order_id != null){
      router.push({ path: "/checkout", query: { order_id: order_id } })
      return true
    }else{
      return false
    }
  }
}
