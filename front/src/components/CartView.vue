<script setup lang="ts">
import Button from "primevue/button"
import { type CartElemWithSubtotal, type Cart } from "@/scripts/cart";
import { f_price } from "@/scripts/utils";
import DataTable from "primevue/datatable"
import Column from "primevue/column"
import ColumnGroup from "primevue/columngroup"
import Row from "primevue/row"
import { ref } from "vue";
defineProps<{ cart: Cart }>();
const emit = defineEmits<{ validate: [] }>()
let loading = ref(false)

function validate(){
    loading.value = true;
    emit('validate')
}

</script>
<template>
    <div class="container">
        <div class="cart">
            <DataTable
                :value="cart.elems_with_subtotal()">
        <Column :field="(e: CartElemWithSubtotal) => e.cart_element.product.name + e.cart_element.variation.name ? ` ${e.cart_element.product.name} (${e.cart_element.variation.name})` : ''"
                    header="Article"></Column>
                <Column :field="(e: CartElemWithSubtotal) => e.cart_element.quantity" header="Quantité"></Column>
                <Column :field="(e: CartElemWithSubtotal) => f_price(e.subtotal)" header="Sous-total"></Column>
                <ColumnGroup type="footer">
                    <Row>
                        <Column footer="Total:" :colspan="2" footerStyle="text-align:right" />
                        <Column :footer="cart.get_total()" />
                    </Row>
                </ColumnGroup>
            </DataTable>
            <div class="button">
                <Button class="valider" @click="validate" :badge="cart.get_total()" badge-severity="contrast"
                    label="Valider" icon="pi pi-credit-card" :loading="loading"></Button>
            </div>
        </div>
    </div>
</template>
<style scoped>
.container {
    width: 100%;
}

.cart {
    display: flex;
    flex-direction: column;
    padding: 20px;
    max-width: 500px;
    margin: 0 auto;
}

.item {
    display: flex;
    width: 50vw;
    margin: 10px auto;
    justify-content: space-between;
    align-items: center;
}

.product-name {
    text-transform: capitalize;
}

.name-quantity {
    display: flex;
}

.subtotal {
    margin: auto 10px;
    text-align: center;
}

.input-buttons {
    width: 50%;
}



.item-content {
    margin: 0;
}

.remove {
    height: fit-content;
    margin-left: 10px;
}

.button {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    align-items: end;
}

.valider {
    min-width: 100px;
    position: relative;
    bottom: 25%;
    margin-top: 20px;
    font-size: larger;
}
</style>
