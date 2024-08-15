<script setup lang="ts">
import type { Cart, Product, ProductId } from '@/types';
import Button from "primevue/button"
import InputNumber from 'primevue/inputnumber';
let props = defineProps<{ cart: Cart }>();
defineEmits<{ updateCart: [ProductId, number], validate: [] }>()
</script>
<template>
    <div class="cart">
        <div v-for="(elem, index) in cart.elems_with_subtotal()" class="item">
            <p class="product-name">{{ elem.product.name }}</p>
            <InputNumber :model-value="cart.elements[index].quantity" inputId="horizontal-buttons" showButtons
                buttonLayout="horizontal" :step="1" fluid class="input-buttons" focused="false">
                <template #incrementbutton>
                    <Button icon="pi pi-plus" severity="secondary" class="increment"
                        @click="(e) => { $emit('updateCart', elem.product.product_id, cart.elements[index].quantity + 1); e.preventDefault() }"></Button>
                </template>
                <template #decrementbutton="test">
                    <Button :icon="elem.quantity > 1 ? 'pi pi-minus' : 'pi pi-trash'" severity="secondary"
                        class="decrement"
                        @click="(e) => { $emit('updateCart', elem.product.product_id, cart.elements[index].quantity - 1); e.preventDefault() }"></Button>
                </template>
            </InputNumber>
            <p class="subtotal">{{ (cart.elems_with_subtotal()[index].subtotal / 100).toFixed(2) + " €" }}</p>
            <!-- <Button class="remove" @click="$emit('removeFromCart', elem.product.product_id)">-</Button> -->
        </div>
        <div class="button">
            <Button class="valider" @click="$emit('validate')" :badge="'Total: ' + cart.get_total()"
                badge-severity="contrast" label="Valider"></Button>
        </div>
    </div>
</template>
<style scoped>
.cart {
    display: flex;
    flex-direction: column;
    padding: 20px;
}

.item {
    display: flex;
    margin: 10px 0;
    justify-content: space-evenly;
}

.product-name {
    text-transform: capitalize;
}

.subtotal {
    margin: auto 10px;
    text-align: center;
}

.input-buttons {
    width: 50%;
}

.increment {
    order: 3;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
}

.decrement {
    order: 1;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.item-content {
    margin: 0;
}

.remove {
    height: fit-content;
    margin-left: 10px;
}

.button {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    align-items: end;
}

.valider {
    min-width: 100px;
    position: relative;
    bottom: 25%;
    margin-top: 20px;
    font-size: larger;
}
</style>