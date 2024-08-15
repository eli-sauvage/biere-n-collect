<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import ProductVue from './components/ProductView.vue';
import { Cart, type Product, type ProductId } from './types';
import CartVue from './components/CartView.vue';
let prods: Ref<Product[]> = ref([]);
let cart: Ref<Cart> = ref(new Cart());
(async () => {
    prods.value = await (await fetch("http://127.0.0.1:8000/api/stocks")).json();
})();

</script>

<template>
    <div class="container">
        <div class="product-list">
            <ProductVue v-for="prod in prods" :product="prod" @addToCart="(item) => cart.addOneElem(item)" />
        </div>
        <div class="separator"></div>
        <CartVue :cart="cart" @removeFromCart="(id) => cart.removeOneElem(id)" @validate="cart.validate($router)"/>
    </div>

</template>

<style scoped>
.container {
    display: flex;
}

.separator {
    width: 1px;
    position: sticky;
    background-color: black;
    margin-left: 5px;
}
</style>