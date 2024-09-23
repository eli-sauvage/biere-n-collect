<script setup lang="ts">
import type { Product, Variation } from '@/scripts/api/products';
import type { Cart, CartElement } from '@/scripts/cart';
import { f_price } from '@/scripts/utils';
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';
import Tag from 'primevue/tag';
import Divider from 'primevue/divider'
import { ref, type Ref } from 'vue';
import { watch } from 'vue';
let props = defineProps<{ product: Product, cart: Cart }>();

let variations_with_cart_elem: Ref<[Variation, CartElement][]> =
  ref(props.product.variations.map(variation => [variation, props.cart.getElement(variation.id)]))

function allow_add_product(quantity: number): boolean {
    let new_quantity = props.cart.elements
        .filter(e => e.product.id == props.product.id)
        .map(e => e.quantity)
        .reduce((prev, curr) => prev + curr, 0)
        + quantity;
    return props.product.stock_quantity - new_quantity >= 0;
}


function addOne(e: Event, cartElem: CartElement) {
    e.preventDefault();
    cartElem.add(1)
}
function removeOne(e: Event, cartElem: CartElement) {
    e.preventDefault()
    cartElem.add(-1)
}

</script>

<template>
    <div class="prod">
        <div class="top">
            <h2 class="titre">{{ product.name }}</h2>
            <p class="description">{{ product.description }}</p>
            <Tag class="stock" v-if="product.stock_quantity == 0" value="plus de stock!" severity="danger"></Tag>
        </div>
        <div v-for="([variation, cartElem], index) in variations_with_cart_elem">
            <Divider v-if="index != 0" class="variation-divider" />
            <div class="variation">
                <p>{{ variation.name }}</p>
                <div class="add-and-price">
                    <InputNumber v-if="cartElem.quantity > 0" :model-value="cartElem.quantity"
                        inputId="horizontal-buttons" showButtons buttonLayout="horizontal" :step="1" fluid
                        class="input-buttons" focused="false"
                        @update:model-value="e=>cartElem.setQuantity(e)"  >
                        <template #incrementbutton>
                            <Button :disabled="!allow_add_product(variation.volume)" icon="pi pi-plus"
                                severity="primary" class="increment" @click="(e) => addOne(e, cartElem)"
                                :badge="f_price(variation.price_ttc)"></Button>
                        </template>
                        <template #decrementbutton>
                            <Button :icon="cartElem.quantity > 1 ? 'pi pi-minus' : 'pi pi-trash'" severity="secondary"
                                class="decrement" @click="(e) => removeOne(e, cartElem)"></Button>
                        </template>
                    </InputNumber>
                    <Button v-if="cartElem.quantity == 0" icon="pi pi-plus" severity="primary" class="add-to-cart"
                        @click="(e) => addOne(e, cartElem)" :disabled="product.stock_quantity == 0"
                        :badge="f_price(variation.price_ttc)"></Button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.prod {
    margin: 20px auto;
    display: flex;
    flex-direction: column;
    padding: 20px 40px;
    border-radius: 10px;
    background-color: #1b6589;
    color: white;
    width: 90vw;
    max-width: 450px;
    min-width: 30vw;
    justify-content: center;
    /* font-size: large */
}

.top {
    display: flex;
    flex-direction: column;
    text-align: left;
}
.titre{
    text-transform: capitalize
}
.description{
    opacity: .7;
}



.input-buttons {
    max-width: 200px;
}

.variation {
    display: flex;
    justify-content: space-between;

}

.variation-divider {
    color: white;
    opacity: .2;
    width: 80%;
    margin: 0 auto;
}

.add-and-price {
    display: flex;
    align-items: center;
}

.increment {
    order: 3;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
    flex-direction: row-reverse;
    gap: 2px;
}

.decrement {
    order: 1;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.ajout {
    margin-top: 10px;
}

.add-to-cart {
    flex-direction: row-reverse;
    gap: 2px;
}
</style>
