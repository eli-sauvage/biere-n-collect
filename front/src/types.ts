import { routerKey, useRouter, type Router } from "vue-router";

export type ProductId = number;

export type Product = {
    product_id: ProductId,
    name: string,
    stock: number,
    price: number
};

export type CartElement = {
    product: Product,
    quantity: number,
};

export class Cart {
    elements: CartElement[] = []
    elems_with_subtotal(): (CartElement & { subtotal: number })[] {
        return this.elements.map((e) => { return { ...e, subtotal: e.product.price * e.quantity } })
    }
    get_total(): string {
        return (this.elements.reduce((acc, e) => acc + e.product.price * e.quantity, 0)/100).toFixed(2) + " €";
    }
    addOneElem(item: Product) {
        console.log(this)
        let index = this.elements.findIndex((e) => e.product.product_id == item.product_id);
        if (index == -1) {
            this.elements.push({ product: item, quantity: 1 });
        } else {
            this.elements[index].quantity += 1;
        }
    }
    updateElemCount(product_id: ProductId, new_quantity: number) {
        let index = this.elements.findIndex((e) => e.product.product_id == product_id);
        if (index != -1) {
            if(new_quantity <=0){
                this.elements.splice(index, 1)
            }else{
                this.elements[index].quantity = new_quantity
            }
        }
    }
    async validate(router: Router, showErrorToast: (msg: string) => void) {
        if (this.elements.length == 0) return
        try {
            let res = await fetch(`${import.meta.env.VITE_API_URL}/validate_cart`,
                {
                    method: "POST",
                    body: JSON.stringify(
                        {
                            cart: this.elements.map((el) => {
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
            } else { console.log("not ok"); showErrorToast(await res.text()) }
        } catch (e: any) {
            console.error(e)
            showErrorToast(e.toString())
        }
    }
}