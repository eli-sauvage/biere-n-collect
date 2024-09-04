<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import ProductVue from './components/ProductView.vue';
import { Cart, type ProductId } from './scripts/cart';
import CartVue from './components/CartView.vue';
import Button from "primevue/button"
import Drawer from 'primevue/drawer';
import { get_bar_status, get_stock, type BarStatus } from './scripts/api/order';
let cart: Ref<Cart> = ref(new Cart([]));
let bar_status: Ref<BarStatus | null> = ref(null);
let visible = ref(false);
(async () => {
    bar_status.value = await get_bar_status();
    if (bar_status.value == null) return
    if (bar_status.value.is_open == false) return
    cart.value = new Cart(await get_stock())
})();

</script>

<template>
    <div v-if="bar_status?.is_open">
        <Drawer class="drawer-cart" v-model:visible="visible" header="Panier" position="bottom">
            <CartVue :cart="cart" @validate="cart.validate($router)" />
        </Drawer>
        <div class="product-list">
            <ProductVue v-for="element in cart.elements" :cardElement="element" />
        </div>
        <div class="see-cart-back">
            <Button class="see-cart" icon="pi pi-shopping-cart" label="Voir le panier" :badge="cart.get_total()"
                @click="visible = true" badgeSeverity="contrast"></Button>
        </div>
    </div>
    <div v-else>
        <pre class="closed-message">{{ bar_status?.closed_message }}</pre>
    </div>
</template>

<style scoped>
.product-list {
    margin-bottom: 15vh;
}

.see-cart-back {
    width: 100vw;
    height: 10vh;
    position: fixed;
    bottom: 0;
    background-color: white;
}

.see-cart {
    width: 90vw;
    position: relative;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
.closed-message{
    text-align: center;
    display: block;
    margin-top: 20vh;
    font-size: large;
    font-family: unset;
}
</style>
<style>
.drawer-cart {
    height: fit-content !important;
}

.p-drawer-header {
    padding-bottom: 0 !important;
}
</style>