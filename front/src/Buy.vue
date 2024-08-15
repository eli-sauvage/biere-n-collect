<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import ProductVue from './components/ProductView.vue';
import { Cart, type Product, type ProductId } from './types';
import CartVue from './components/CartView.vue';
import { useToast } from 'primevue/usetoast';
import Button from "primevue/button"
import Toast from 'primevue/toast';
import Drawer from 'primevue/drawer';
let prods: Ref<Product[]> = ref([]);
let cart: Ref<Cart> = ref(new Cart());
let visible = ref(false);
(async () => {
    prods.value = await (await fetch(`${import.meta.env.VITE_API_URL}/stocks`)).json();
})();

const toast = useToast();

const showErrorToast = (msg: string) => {
    toast.add({ severity: 'error', summary: 'Error', detail: msg, life: 3000 });
}

</script>

<template>
    <div class="card flex justify-center">
        <Toast />
        <Drawer class="drawer-cart" v-model:visible="visible" header="Panier" position="bottom">
            <CartVue :cart="cart" @update-cart="(prod_id, count)=>cart.updateElemCount(prod_id, count)"
                @validate="cart.validate($router, showErrorToast)" />
        </Drawer>
        <div class="product-list">
            <ProductVue v-for="prod in prods" :product="prod" @addToCart="(e)=>cart.addOneElem(e)" />
        </div>
        <div class="see-cart-back">
            <Button class="see-cart" icon="pi pi-shopping-cart" label="Voir le panier" :badge="cart.get_total()"
                @click="visible = true" badgeSeverity="contrast"></Button>
        </div>
    </div>
</template>

<style scoped>
.product-list{
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
.p-drawer-header{
    padding-bottom: 0 !important;
}
</style>