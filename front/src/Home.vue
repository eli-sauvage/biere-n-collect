<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import ProductVue from './components/ProductView.vue';
import { Cart, type Product, type ProductId } from './types';
import CartVue from './components/CartView.vue';
import Button from "primevue/button"
import Drawer from 'primevue/drawer';
let cart: Ref<Cart> = ref(new Cart([]));
let visible = ref(false);
(async () => {
    cart.value = new Cart(await (await fetch(`${import.meta.env.VITE_API_URL}/stock/get`)).json());
})();

</script>

<template>
    <div class="card flex justify-center">
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
</style>
<style>
.drawer-cart {
    height: fit-content !important;
}

.p-drawer-header {
    padding-bottom: 0 !important;
}
</style>