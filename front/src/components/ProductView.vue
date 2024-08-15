<script setup lang="ts">
import type { Product } from '@/types';
import Button from 'primevue/button';
import Card from 'primevue/card';
defineProps<{ product: Product }>();
defineEmits<{
    'addToCart': [Product]
}>();
</script>

<template>
    <div class="prod">
        <div class="img">
            <img src="https://placehold.co/100/png" />
        </div>
        <div class="infos">
            <p class="titre">{{ product.name }}</p>
            <div class="price-add">
                <Button @click="$emit('addToCart', product)" icon="pi pi-plus" class="ajout"
                    :badge="(product.price / 100).toFixed(2) + '€'" badgeSeverity="contrast"></Button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* p {
    display: inline;
    margin: 0 10px 0 0;
} */

.infos {
    margin-left: 20px;
    display: flex;
    flex-direction: column;
    /* justify-content: space-between; */
    flex-grow: 1;
}

.titre {
    text-align: center;
    text-transform: capitalize;
    word-break: break-all;
}
.price-add{
    display: flex;
    flex-direction: column;
    align-items: center;
}

.ajout {
    margin-top: 10px;
}


.prod {
    margin: 20px auto;
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