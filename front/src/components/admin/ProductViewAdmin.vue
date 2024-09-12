<script setup lang="ts">
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import ConfirmPopup from 'primevue/confirmpopup';
import ToggleSwitch from 'primevue/toggleswitch';
import { useConfirm } from 'primevue/useconfirm';
import { f_price } from '@/scripts/utils';
import type { Product, Variation } from '@/scripts/api/products';
import { delete_product, edit_product, move_product, remove_variation } from '@/scripts/api/admin/stock/product-management';
import { edit_variation } from '@/scripts/api/admin/stock/variations-management';
import Divider from 'primevue/divider';

let props = defineProps<{ product: Product, first?: boolean, last?: boolean }>();
let emit = defineEmits<{
  refresh_stock: []
  requestEditProduct: [product: Product],
  requestAddVariation: [product: Product],
  requestEditVariation: [variation: Variation]
}>()

const confirm = useConfirm();
const comfirmDeleteProduct = (event: Event) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message: 'Etes-vous sûr ?',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true
        },
        acceptProps: {
            label: 'Supprimer',
            severity: 'danger'
        },
        accept: async () => {
            if (await delete_product(props.product.id)) {
                emit('refresh_stock')
            }

        },
        reject: () => { }
    });
};
function confirmDeleteVariation(event: Event, variation: Variation){
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message: 'Etes-vous sûr ?',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true
        },
        acceptProps: {
            label: 'Supprimer',
            severity: 'danger'
        },
        accept: async () => {
            if (await remove_variation(props.product.id, variation.id)) {
                emit('refresh_stock')
            }

        },
        reject: () => { }
    });
};
async function moveProduct(direction: "up" | "down") {
    if (await move_product(props.product.id, direction)) {
        emit('refresh_stock')
    }
}

async function toogleVariationAvailable(variation: Variation){
    if(await edit_variation(variation.id, {new_available_to_order: !variation.available_to_order})){
        emit('refresh_stock')
  }else{console.log("non")}

}

async function toogleProductAvailable(){
  if(await edit_product(props.product.id, {new_available_to_order: !props.product.available_to_order})){
    emit("refresh_stock")
  }
}
</script>


<template>

  <div class="prod">
    <div class="top">
      <div class="move">
        <Button icon="pi pi-arrow-up" severity="secondary" v-if="!first" @click="moveProduct('up')"
          size="small"></Button>
        <Button icon="pi pi-arrow-down" severity="secondary" size="small" v-if="!last"
          @click="moveProduct('down')"></Button>
      </div>
      <div class="product-details">
        <p class="titre">{{ product.name }}</p>
        <p class="description">{{ product.description }}</p>
        <div>
          <span class="stock">stock: &nbsp;</span>
          <Tag :value="product.stock_quantity" severity="secondary"/>
        </div>
        <!--<div class="available-prod">
          <span>Dispo</span>
          <ToggleSwitch :modelValue="product.available_to_order"
@click="toogleProductAvailable" />
</div>-->
      </div>
    </div>
    <ConfirmPopup></ConfirmPopup>
    <div class="bottom">
      <Divider/>
      <div v-for="variation in product.variations" class="variation">
        <div class="variation-details">
        <span> {{ variation.name }} </span>
          <ToggleSwitch :modelValue="variation.available_to_order"
            @click="toogleVariationAvailable(variation)" />
        </div>
        <div class="edit-variation">
          <Button icon="pi pi-pencil" severity="primary" @click="$emit('requestEditVariation', variation)" :badge="f_price(variation.price_ttc)"></Button>
          <Button icon="pi pi-trash" severity="danger" @click="(e)=>confirmDeleteVariation(e, variation)" ></Button>
        </div>
      </div>
    <Button label="ajouter une variation" icon="pi pi-plus" @click="$emit('requestAddVariation', product)"></Button>
      <Divider/>
      <div class="edit-product">
        <Button icon="pi pi-pencil" severity="primary" @click="$emit('requestEditProduct', product)"></Button>
        <Button icon="pi pi-trash" severity="danger" @click="comfirmDeleteProduct"></Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
* {
  color: black; 
}

.prod {
    /* margin: 20px auto; */
    display: flex;
  flex-direction: column;
    /* align-items: center; */
    padding: 10px 20px;
    border-radius: 10px;
    background-color: #1b6589;
    width: 80vw;
    min-width: 300px;
    justify-content: center;
}

.top {
  display: flex;
  align-items: center;
  gap: 20px;
}

.move {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 5px;
    margin-right: 10px;
}

.product-details {
  margin-left: 20px;
  display: flex;
  flex-direction: column;
  justify-content: end;
  gap: 10px;
  flex-grow: 1;
}

.available-prod {
  display: flex;
  gap: 10px;
  align-items: center;
}

.stock {
    margin: 0;
}

.titre {
  margin:0;
  margin-top: 20px;
}
.description{
  margin: 0;
  margin-bottom: 20px;
}

.bottom {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 10px 0;
}

.variation {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
}


.variation-details, .edit-variation {
  display: flex;
  align-items: center;
  gap: 10px;
}

.edit-product {
  display: flex;
  justify-content: end;
  gap: 10px;
}
</style>
