<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import ProductViewAdmin from './ProductViewAdmin.vue';
import EditProduct from './EditProduct.vue';
import Tabs from 'primevue/tabs';
import Tab from 'primevue/tab';
import TabList from 'primevue/tablist';
import TabPanel from 'primevue/tabpanel';
import TabPanels from 'primevue/tabpanels';
import { get_categories, get_stock, type Category, type Product } from '@/scripts/api/products';
import { delete_product, insert_product, move_product, edit_product } from '@/scripts/api/admin/stock/product-management';
import { get_stock, type Product } from '@/scripts/api/products';

let categories: Ref<Category[]> = ref([]);
let products: Ref<Product[]> = ref([]);

let refresh_stock = async () => {
    products.value = await get_stock()
    categories.value = await get_categories()
};
refresh_stock()



let editing_product: Ref<Product | null> = ref(null);
async function saveEdit(new_prod: Product) {
    console.log(new_prod)
    if (editing_product == null) return;
    //TODO
    // if (await update_stock(new_prod)) {
    //     editing_product.value = null;
    //     refresh_stock()
    // }
}



async function requestCreateProduct() {
    editing_product.value = { id: 0, name: "", description: "", stock_quantity: 0, available_to_order: true, variations: [] };
}

async function createProduct(new_prod: Product) {
    if (editing_product.value == null) return;
    if (await insert_product(new_prod)) {
        editing_product.value = null;
        refresh_stock()
    }
}


</script>

<template>
    <Dialog :visible="editing_product != null" modal
        :header="editing_product?.id ? editing_product.name : 'Ajout d\'un produit'" :draggable="false"
        :closable="false">
        <EditProduct @close="editing_product = null" @save="(p) => p.id ? saveEdit(p) : createProduct(p)"
            v-if="editing_product != null" :product="editing_product" />
    </Dialog>
    <div class="add-product-container">
        <Button label="Ajouter un produit" icon="pi pi-plus" class="btn-add-product"
            @click="requestCreateProduct"></Button>
    </div>
    <Tabs :value="categories[0]?.id.toString()" scrollable>
        <TabList class="home-tab-list">
            <Tab v-for="category in categories" :value="category.id.toString()">{{ category.name }}</Tab>
        </TabList>
        <TabPanels class="home-tab-panel">
            <TabPanel v-for="category in categories" :value="category.id.toString()">
                <ProductViewAdmin v-for="(product, index) in products.filter(p => p.category?.id == category.id)"
                    :product="product" class="product" :first="index == 0" :last="index == products.length - 1"
                    @request-edit="(prod) => editing_product = prod" @refresh_stock="refresh_stock" />
            </TabPanel>
        </TabPanels>
    </Tabs>
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