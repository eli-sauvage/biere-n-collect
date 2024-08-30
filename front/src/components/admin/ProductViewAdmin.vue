<script setup lang="ts">
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import ConfirmPopup from 'primevue/confirmpopup';
import ToggleSwitch from 'primevue/toggleswitch';
import { useConfirm } from 'primevue/useconfirm';
import type { Product } from '@/scripts/api/order';
import { f_price } from '@/scripts/utils';
let props = defineProps<{ product: Product, first?: boolean, last?:boolean }>();
let emit = defineEmits<{
    requestEdit: [product: Product],
    requestDelete: [product_id: number],
    requestMove: [product_id: number, direction: "up" | "down"],
    directEdit: [new_product: Product]
}>()
const confirm = useConfirm();
const confirm_delete = (event: Event) => {
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
        accept: () => {
            emit("requestDelete", props.product.product_id)
        },
        reject: () => { }
    });
};

</script>

<template>

    <div class="prod">
        <div class="left">
            <Button icon="pi pi-arrow-up" severity="secondary" v-if="!first"
                @click="$emit('requestMove', product.product_id, 'up')" size="small"></Button>
            <img src="https://placehold.co/100/png" />
            <Button icon="pi pi-arrow-down" severity="secondary" size="small" v-if="!last"
                @click="$emit('requestMove', product.product_id, 'down')"></Button>
        </div>
        <div class="right">
            <div class="titre-price">
                <p class="titre">{{ product.name }}</p>
                <Tag :value="f_price(product.price)"></Tag>
            </div>
            <p class="stock">stock: {{ product.quantity }}</p>
            <div class="footer">
                <div class="available">
                    <label for="available">Dispo</label>
                    <ToggleSwitch id="available" :modelValue="product.available"
                        @click="$emit('directEdit', { ...product, available: !product.available })" />
                </div>
                <div class="btns">
                    <ConfirmPopup></ConfirmPopup>
                    <Button icon="pi pi-trash" severity="danger" @click="confirm_delete"></Button>
                    <Button icon="pi pi-pencil" severity="primary" @click="$emit('requestEdit', product)"></Button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>

.prod {
    /* margin: 20px auto; */
    display: flex;
    /* align-items: center; */
    padding: 10px 20px;
    border-radius: 10px;
    background-color: #1b6589;
    width: 80vw;
    min-width: 30vw;
    justify-content: center;
}

.left {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 5px;
    margin-right: 10px;
}
.right {
    margin-left: 20px;
    display: flex;
    flex-direction: column;
    justify-content: end;
    flex-grow: 1;
}

.stock {
    margin: 0;
}

.titre {
    /* margin-top: 0; */
    text-align: center;
    text-transform: capitalize;
    word-break: break-all;
}

.footer {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 10px 0;
}
.footer > div {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: end;
    gap: 10px;
}

.footer > .available{
    justify-content: start;
}
.footer > .btns {
    justify-content: end;
}

.titre-price {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.ajout {
    margin-top: 10px;
}

</style>