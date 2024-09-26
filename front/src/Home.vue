<script setup lang="ts">
import { ref, type Ref, watch } from 'vue'

import ProductVue from './components/ProductView.vue'
import { Cart } from './scripts/cart'
import {
    get_bar_status,
    get_available_products,
    type BarStatus,
    type Product,
} from './scripts/api/products'
import CartView from './components/CartView.vue'

let cart: Ref<Cart> = ref(new Cart([]))
let bar_status: Ref<BarStatus | null> = ref(null)
let visible = ref(false)
let products: Ref<Product[]> = ref([])
;(async () => {
    bar_status.value = await get_bar_status()
    if (bar_status.value == null) return
    if (bar_status.value.is_open == false) return
    products.value = await get_available_products()
    cart.value = new Cart(products.value)
})()

let about_visible = ref(false)

watch(
    cart,
    (cart) => {
        cart.updateCache()
    },
    { deep: true }
)
</script>

<template>
    <i
        class="pi pi-info-circle about"
        @click="about_visible = !about_visible"
    ></i>
    <Drawer
        class="drawer-cart"
        v-model:visible="about_visible"
        header="A propos"
        position="bottom"
    >
        <span>© 2024 - E. Sauvage</span>
    </Drawer>
    <div v-if="bar_status?.is_open">
        <Drawer
            class="drawer-cart"
            v-model:visible="visible"
            header="Panier"
            position="bottom"
        >
            <CartView :cart="cart" />
        </Drawer>
        <div class="product-list">
            <ProductVue
                v-for="product in products"
                :cart="cart"
                :product="product"
            />
        </div>
        <div class="see-cart-back">
            <Button
                class="see-cart"
                icon="pi pi-shopping-cart"
                label="Voir le panier"
                :badge="cart.get_total()"
                @click="visible = true"
                badgeSeverity="contrast"
            ></Button>
        </div>
    </div>
    <div v-else>
        <pre class="closed-message">{{ bar_status?.closed_message }}</pre>
    </div>
</template>

<style scoped>
.about {
    position: fixed;
    top: 10px;
    right: 10px;
    font-size: 1rem;
    background-color: #dddddd;
    opacity: 0.5;
    padding: 2px;
    border-radius: 4px;
}

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

.closed-message {
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
    max-height: 100vh;
}

.p-drawer-header {
    padding-bottom: 0 !important;
}
</style>
