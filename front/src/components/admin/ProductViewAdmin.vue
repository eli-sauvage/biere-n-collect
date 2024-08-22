<script setup lang="ts">
import { f_price, type CartElement, type Product } from '@/types';
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import ConfirmPopup from 'primevue/confirmpopup';
import ToggleSwitch from 'primevue/toggleswitch';
import { useConfirm } from 'primevue/useconfirm';
import { useToast } from 'primevue/usetoast';
let props = defineProps<{ product: Product }>();
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
        <div class="btns-move">
            <Button icon="pi pi-arrow-up" severity="secondary"
                @click="$emit('requestMove', product.product_id, 'up')"></Button>
            <Button icon="pi pi-arrow-down" severity="secondary"
                @click="$emit('requestMove', product.product_id, 'down')"></Button>
        </div>
        <div class="img">
            <img src="https://placehold.co/100/png" />
        </div>
        <div class="infos">
            <div class="titre-price">
                <p class="titre">{{ product.name }}</p>
                <Tag :value="f_price(product.price)"></Tag>
            </div>
            <p class="stock">stock: {{ product.quantity }}</p>
            <div class="footer">
                <div>
                    <label for="available">Dispo</label>
                    <ToggleSwitch id="available" :modelValue="product.available"
                        @click="$emit('directEdit', { ...product, available: !product.available })" />
                </div>
                <div>
                    <ConfirmPopup></ConfirmPopup>
                    <Button icon="pi pi-trash" severity="danger" @click="confirm_delete"></Button>
                    <Button icon="pi pi-pencil" severity="primary" @click="$emit('requestEdit', product)"></Button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.infos {
    margin-left: 20px;
    display: flex;
    flex-direction: column;
    justify-content: end;
    flex-grow: 1;
}

.btns-move {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin-right: 10px;
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
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-top: 10px;
}
.footer > div {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: end;
    gap: 10px;
}

.titre-price {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.ajout {
    margin-top: 10px;
}

.prod {
    /* margin: 20px auto; */
    display: flex;
    /* align-items: center; */
    padding: 20px;
    border-radius: 10px;
    background-color: #1b6589;
    width: 80vw;
    min-width: 30vw;
    justify-content: center;
}
</style>