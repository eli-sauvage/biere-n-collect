<script setup lang="ts">
import { ref } from 'vue'
import type { Product } from '@/scripts/api/products'

let props = defineProps<{ product: Product }>()
let emit = defineEmits<{ close: []; save: [newProduct: Product] }>()
let prod = ref({ ...props.product })

function save(e: Event) {
    e.preventDefault()
    // prod.value.price = Math.round(price.value * 100);
    emit('save', prod.value)
}
</script>

<template>
    <form>
        <div class="inputs">
            <label for="name">Nom</label>
            <InputText id="name" v-model="prod.name" />
            <label for="name">Description</label>
            <InputText id="name" v-model="prod.description" />
            <label for="stock">Stock</label>
            <InputNumber
                id="stock"
                v-model="prod.stock_quantity"
                :min-fraction-digits="0"
                :max-fraction-digits="2"
                :use-grouping="false"
            />
        </div>
        <div class="footer">
            <Button
                type="button"
                label="Annuler"
                severity="secondary"
                @click="$emit('close')"
            ></Button>
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

.inputs > label,
.available {
    margin-top: 10px;
}

.available {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
</style>
