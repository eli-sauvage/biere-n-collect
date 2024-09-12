<script setup lang="ts">
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import ToggleSwitch from 'primevue/toggleswitch';
import InputNumber from 'primevue/inputnumber';
import { ref } from 'vue';
import type { Variation } from '../../scripts/api/products';
let props = defineProps<{ variation: Variation }>()
let emit = defineEmits<{ close: [], save: [newVariation: Variation] }>()
let newVariation = ref({...props.variation})

let price_ht = ref(newVariation.value.price_ht / 100);
let price_ttc = ref(newVariation.value.price_ttc / 100);
let tva = ref(newVariation.value.tva * 100);

function save(e:Event) {
    e.preventDefault()
    newVariation.value.price_ht = Math.round(price_ht.value * 100);
    newVariation.value.price_ttc = Math.round(price_ttc.value * 100);
    emit("save", newVariation.value)
}

function changePriceHT(new_price_ht: {value: number}){
  price_ht.value = new_price_ht.value;
  price_ttc.value = price_ht.value * ( 1 + newVariation.value.tva);
}

function changeTVA(new_tva: {value: number}){
  newVariation.value.tva = new_tva.value / 100;
  price_ttc.value = price_ht.value * ( 1 + newVariation.value.tva);
}

function changePriceTTC(new_price_ttc: {value: number}){
  price_ttc.value = new_price_ttc.value;
  price_ht.value = price_ttc.value / ( 1 + newVariation.value.tva);
}
</script>

<template>
    <form>
        <div class="inputs">
            <label for="name">Nom</label>
            <InputText id="name" v-model="newVariation.name" />
            <label for="price">Prix HT</label>
            <InputNumber id="price" v-model="price_ht" mode="currency" currency="EUR" @input="changePriceHT"/>
            <label for="price">TVA</label>
            <InputNumber id="price" v-model="tva" suffix="%" @input="changeTVA"/>
            <label for="price">Prix TTC</label>
            <InputNumber id="price" v-model="price_ttc" mode="currency" currency="EUR" @input="changePriceTTC"/>
            <label for="price">Volume à déduire au stock par commande</label>
            <InputNumber id="price" v-model="newVariation.volume" mode="decimal" />
            <div class="available">
                <label for="available">Dispo à la commande</label>
                <ToggleSwitch id="available" v-model="newVariation.available_to_order" />
            </div>
        </div>
        <div class="footer">
            <Button type="button" label="Annuler" severity="secondary" @click="$emit('close')"></Button>
            <Button type="submit" label="Valider" @click="save"></Button>
        </div>
    </form>
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
