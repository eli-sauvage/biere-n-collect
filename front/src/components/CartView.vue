<script setup lang="ts">
import type { Cart, Product, ProductId } from '@/types';
let props = defineProps<{ cart: Cart }>();
defineEmits<{ removeFromCart: [ProductId], validate: [] }>()
</script>
<template>
    <div class="cart">
        <div v-for="elem in cart.elems_with_subtotal()" class="item">
            <p class="item-content">{{ elem.product.name + " x " + elem.quantity + " => " +
                (elem.subtotal / 100).toFixed(2)}}</p>
            <button class="remove" @click="$emit('removeFromCart', elem.product.product_id)">-</button>
        </div>
        <div class="button">
            <p></p>
            <button class="valider" @click="$emit('validate')">{{ "VALIDER (Total = " +
                (cart.get_total() / 100).toFixed(2) + ")"}}</button>
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
    height: 50px;
    min-width: 100px;
    background-color: lightgreen;
}
</style>