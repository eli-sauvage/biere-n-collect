<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import ProductViewAdmin from './ProductViewAdmin.vue';
import EditProduct from './EditProduct.vue';
import EditVariation from './EditVariation.vue';
import { insert_product, edit_product, add_variation, get_all_products } from '@/scripts/api/admin/stock/product-management';
import { type Product, type Variation } from '@/scripts/api/products';
import { edit_variation } from '@/scripts/api/admin/stock/variations-management';

let products: Ref<Product[]> = ref([]);

let refresh_stock = async () => {
    products.value = await get_all_products()
};
refresh_stock()



let editing_product: Ref<Product | null> = ref(null);
let editing_variation: Ref<Variation | null> = ref(null);
async function saveEditProd(new_prod: Product) {
  if (editing_product == null || new_prod.id == 0) return;
  if (await edit_product(new_prod.id, {
    new_name: new_prod.name,
    new_description: new_prod.description,
    new_stock_quantity: new_prod.stock_quantity,
    new_available_to_order: new_prod.available_to_order
  })) {
    editing_product.value = null;
    refresh_stock()
  }
}

async function requestCreateProduct() {
    editing_product.value = { id: 0, name: "", description: "", stock_quantity: 0, available_to_order: true, variations: [] };
}

async function createProduct(new_prod: Product) {
    if (editing_product.value == null || new_prod.id != 0) return;
    if (await insert_product(new_prod)) {
        editing_product.value = null;
        refresh_stock()
    }
}


async function saveEditVariation(new_variation: Variation) {
  console.log("ui")
  if (editing_product == null || new_variation.id == 0) return;
  if (await edit_variation(new_variation.id, {
    new_name: new_variation.name,
    new_price_ht: new_variation.price_ht,
    new_volume: new_variation.volume,
    new_tva: new_variation.tva,
    new_available_to_order: new_variation.available_to_order
  })) {
    editing_variation.value = null
    refresh_stock()
  }
}

async function requestCreateVariation(product: Product) {
  editing_variation.value = {id: 0, product_id: product.id, name: "", price_ht: 0, price_ttc: 0, tva: 0.2, volume: 1, available_to_order: true}
}

async function createVariation(new_variation: Variation) {
  console.log(new_variation)
  if (editing_variation.value == null || new_variation.id != 0) return;
  if (await add_variation(new_variation.product_id,
    new_variation.name,
    new_variation.price_ht,
    new_variation.tva,
    new_variation.volume,
    new_variation.available_to_order
  )){
    editing_variation.value = null
    refresh_stock()
    }
}
</script>

<template>
    <Dialog :visible="editing_product != null" modal
    :header="editing_product?.id ? `Produit: ${editing_product.name}`: 'Ajout d\'un produit'" :draggable="false"
        :closable="false">
        <EditProduct @close="editing_product = null" @save="(p) => p.id ? saveEditProd(p) : createProduct(p)"
            v-if="editing_product != null" :product="editing_product" />
    </Dialog>
    <Dialog :visible="editing_variation != null" modal
    :header="editing_variation?.id ? `Variation: ${editing_variation.name}`: 'Ajout d\'une variation'" :draggable="false"
        :closable="false">
    <EditVariation @close="editing_variation = null" @save="(v) => v.id ? saveEditVariation(v) : createVariation(v)"
      v-if="editing_variation" :variation="editing_variation"/>
    </Dialog>
    <div class="add-product-container">
        <Button label="Ajouter un produit" icon="pi pi-plus" class="btn-add-product"
            @click="requestCreateProduct"></Button>
    </div>
    <ProductViewAdmin v-for="(product, index) in products" :product="product"
      class="product" :first="index == 0" :last="index == products.length - 1"
      @request-edit-product="(prod) => editing_product = prod" @refresh_stock="refresh_stock"
      @request-add-variation="requestCreateVariation" @request-edit-variation="(variation) => editing_variation = variation"/>
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
