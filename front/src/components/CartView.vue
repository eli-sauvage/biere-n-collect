<script setup lang="ts">
import { f_price, type Cart, type Product, type ProductId } from '@/types';
import Button from "primevue/button"
import Tag from 'primevue/tag';
let props = defineProps<{ cart: Cart }>();
defineEmits<{ validate: [] }>()
</script>
<template>
    <div class="cart">
        <div v-for="(elem, index) in cart.elems_with_subtotal()" class="item">
                <p class="product-name">{{ elem.cart_element.product.name }}</p>
            <div class="name-quantity">
                <Tag :value="'x' + elem.cart_element.quantity"></Tag>
            <p class="subtotal">{{ f_price(cart.elems_with_subtotal()[index].subtotal) }}</p>
            </div>
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
    width: 50vw;
    margin: 10px auto;
    justify-content: space-between;
    align-items: center;
}

.product-name {
    text-transform: capitalize;
}

.name-quantity{
    display: flex;
}

.subtotal {
    margin: auto 10px;
    text-align: center;
}

.input-buttons {
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