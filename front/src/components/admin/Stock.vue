<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import ProductViewAdmin from './ProductViewAdmin.vue';
import type { Product } from '@/types';
import EditProduct from './EditProduct.vue';

let stock: Ref<Product[]> = ref([]);

let refresh_stock = async () => {
    let res_stock = await fetch(`${import.meta.env.VITE_API_URL}/stock/get_all`, { credentials: "include" }).then(e => e.json());
    stock.value = res_stock
};
refresh_stock()

let editing_product: Ref<Product | null> = ref(null);
async function saveEdit(new_prod: Product) {
    if (editing_product == null) return;
    let res = await fetch(`${import.meta.env.VITE_API_URL}/stock`, { method: "PUT", credentials: "include", headers: { ContentType: "application/json" }, body: JSON.stringify(new_prod) }).then(e => e.json());
    console.log(res);
    editing_product.value = null;
    refresh_stock()
}

async function delete_product(product_id: number) {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/stock?product_id=${product_id}`, { method: "DELETE", credentials: "include" }).then(e => e.json());
    console.log(res);
    editing_product.value = null;
    refresh_stock()
}

async function requestCreateProduct() {
    editing_product.value = { product_id: 0, name: "", price: 0, quantity: 0, available: true };
}

async function createProduct(new_prod: Product) {
    if (editing_product.value == null) return;
    let res = await fetch(`${import.meta.env.VITE_API_URL}/stock?name=${new_prod.name}&price=${new_prod.price}&quantity=${new_prod.quantity}&available=${new_prod.available}`,
        { method: "POST", credentials: "include" }).then(e => e.json());
    console.log(res);
    editing_product.value = null;
    refresh_stock()
}

async function moveProduct(product_id: number, direction: "up" | "down") {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/stock/move?product_id=${product_id}&direction=${direction}`,
        { method: "PATCH", credentials: "include" }).then(e => e.json());
    console.log(res);
    refresh_stock()
}
</script>

<template>
    <Dialog :visible="editing_product != null" modal
        :header="editing_product?.product_id ? editing_product.name : 'Ajout d\'un produit'" :draggable="false"
        :closable="false">
        <EditProduct @close="editing_product = null" @save="(p) => p.product_id ? saveEdit(p) : createProduct(p)"
            v-if="editing_product != null" :product="editing_product" />
    </Dialog>
    <div class="add-product-container">
        <Button label="Ajouter un produit" icon="pi pi-plus" class="btn-add-product"
            @click="requestCreateProduct"></Button>
    </div>
    <ProductViewAdmin v-for="product in stock" :product="product" class="product"
        @request-edit="(prod) => editing_product = prod" @request-delete="delete_product" @request-move="moveProduct"
        @direct-edit="saveEdit" />
</template>


<style scoped>
.product {
    margin: 20px auto;
}

.btn-add-product {
    width: 80vw;
    margin: 0 auto;
}

.add-product-container {
    margin-top: 20px;
    width: 100%;
    display: flex;
    justify-content: center;
}
</style>