<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import ProductVue from './components/ProductView.vue';
import { Cart, type ProductId } from './scripts/cart';
import CartVue from './components/CartView.vue';
import Button from "primevue/button"
import Drawer from 'primevue/drawer';
import Tabs from 'primevue/tabs';
import Tab from 'primevue/tab';
import TabList from 'primevue/tablist';
import TabPanel from 'primevue/tabpanel';
import TabPanels from 'primevue/tabpanels';

import { get_bar_status, get_categories, get_stock, type BarStatus, type Category, type Product } from './scripts/api/products';

let cart: Ref<Cart> = ref(new Cart([]));
let bar_status: Ref<BarStatus | null> = ref(null);
let visible = ref(false);
let products: Ref<Product[]> = ref([]);
let categories: Ref<Category[]> = ref([]);

(async () => {
    bar_status.value = await get_bar_status();
    if (bar_status.value == null) return
    if (bar_status.value.is_open == false) return
    products.value = await get_stock()
    categories.value = await get_categories()
    cart.value = new Cart(products.value)
})();

</script>

<template>
    <div v-if="bar_status?.is_open">
        <Drawer class="drawer-cart" v-model:visible="visible" header="Panier" position="bottom">
            <CartVue :cart="cart" @validate="cart.validate($router)" />
        </Drawer>
        <div class="product-list">
            <ProductVue v-for="product in products" :cart="cart" :product="product"/>
            <Tabs :value="categories[0]?.id.toString()" scrollable>
                <TabList class="home-tab-list">
                    <Tab v-for="category in categories" :value="category.id.toString()">{{ category.name }}</Tab>
                </TabList>
                <TabPanels class="home-tab-panel">
                    <TabPanel v-for="category in categories" :value="category.id.toString()">
                        <ProductVue v-for="product in products.filter(prod=>prod.category?.id == category.id)"
                            :product="product" :cart="cart"/>
                    </TabPanel>
                </TabPanels>
            </Tabs>
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

.home-tab-panel{
    padding: 0 !important;
    background-color: var(--background) !important;
}
/* .home-tab-panel p {
    color: black !important;
} */
.home-tab-list .p-tablist-tab-list{
    justify-content: center;
}
</style>