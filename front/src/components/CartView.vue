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
            <InputNumber :model-value="cart.elements[index].quantity"
                @update:modelValue="(newcount: number) => $emit('updateCart', elem.product.product_id, newcount)"
                inputId="horizontal-buttons" showButtons buttonLayout="horizontal" :step="1" fluid class="input-buttons">
                <template #incrementbuttonicon>
                    <span class="pi pi-plus"></span>
                </template>
                <template #decrementbuttonicon>
                    <span v-if="elem.quantity > 1" class="pi pi-minus"></span>
                    <span v-else class="pi pi-trash"></span>
                </template>
            </InputNumber>
            <p class="subtotal">{{ (cart.elems_with_subtotal()[index].subtotal / 100).toFixed(2) + " €"}}</p>
            <!-- <Button class="remove" @click="$emit('removeFromCart', elem.product.product_id)">-</Button> -->
        </div>
        <div class="button">
            <p></p>
            <Button class="valider" @click="$emit('validate')" :badge="'Total: ' + cart.get_total()" badge-severity="contrast" label="Valider"></Button>
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

.subtotal {
    margin: auto 10px;
    text-align: center;
}

.input-buttons{
    width: 50%;
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