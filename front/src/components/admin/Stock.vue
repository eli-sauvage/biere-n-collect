<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import ProductViewAdmin from './ProductViewAdmin.vue';
import EditProduct from './EditProduct.vue';
import { get_stock, type Product } from '@/scripts/api/order';
import { delete_stock, insert_stock, move_stock, update_stock } from '@/scripts/api/admin/stock-management';

let stock: Ref<Product[]> = ref([]);

let refresh_stock = async () => {
    stock.value = await get_stock()
};
refresh_stock()



let editing_product: Ref<Product | null> = ref(null);
async function saveEdit(new_prod: Product) {
    console.log(new_prod)
    if (editing_product == null) return;
    if (await update_stock(new_prod)) {
        editing_product.value = null;
        refresh_stock()
    }
}

async function delete_product(product_id: number) {
    if (await delete_stock(product_id)) {
        editing_product.value = null;
        refresh_stock()
    }
}

async function requestCreateProduct() {
    editing_product.value = { product_id: 0, name: "", price: 0, quantity: 0, available: true };
}

async function createProduct(new_prod: Product) {
    if (editing_product.value == null) return;
    if (await insert_stock(new_prod)) {
        editing_product.value = null;
        refresh_stock()
    }
}

async function moveProduct(product_id: number, direction: "up" | "down") {
    if (await move_stock(product_id, direction)) {
        refresh_stock()
    }
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
    <ProductViewAdmin v-for="(product, index) in stock" :product="product" class="product" :first="index == 0"
        :last="index == stock.length - 1" @request-edit="(prod) => editing_product = prod"
        @request-delete="delete_product" @request-move="moveProduct" @direct-edit="saveEdit" />
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