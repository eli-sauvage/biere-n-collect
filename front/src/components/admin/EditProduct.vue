<script setup lang="ts">
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import ToggleSwitch from 'primevue/toggleswitch';
import InputNumber from 'primevue/inputnumber';
import { ref } from 'vue';
import type { Product } from '@/scripts/api/order';
let props = defineProps<{ product: Product }>()
let emit = defineEmits<{ close: [], save: [newProduct: Product] }>()
let prod = ref({ ...props.product })

let price = ref(prod.value.price / 100);

function save() {
    prod.value.price = Math.round(price.value * 100);
    emit("save", prod.value)
}
</script>

<template>
    <div class="inputs">
        <label for="name">Nom</label>
        <InputText id="name" v-model="prod.name" />
        <label for="price">Prix</label>
        <InputNumber id="price" v-model="price" mode="currency" currency="EUR" />
        <label for="stock">Stock</label>
        <InputNumber locale="fr-FR" id="stock" v-model="prod.quantity" />
        <div class="available">
            <label for="available">Dispo à la commande</label>
            <ToggleSwitch id="available" v-model="prod.available" />
        </div>
    </div>
    <div class="footer">
        <Button type="button" label="Annuler" severity="secondary" @click="$emit('close')"></Button>
        <Button type="button" label="Valider" @click="save"></Button>
    </div>
</template>

<style scoped>
.footer {
    margin-top: 20px;
    display: flex;
    justify-content: space-between;
}

.inputs {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.inputs>label,
.available {
    margin-top: 10px;
}

.available {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
</style>