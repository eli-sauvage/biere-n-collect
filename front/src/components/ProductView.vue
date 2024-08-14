<script setup lang="ts">
import type { Product } from '@/types';
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
        <div class="name">
            <p>{{ product.name + " : " + (product.price/100).toFixed(2) + "€"}}</p>
        </div>
        <div class="ajout">
            <button @click="$emit('addToCart', product)">ajouter</button>
        </div>
    </div>
</template>

<style scoped>
p {
    display: inline;
    margin: 0 10px 0 0;
}

.img{
flex-grow: 1;
}
.name{
    flex-grow: 1;
}
.ajout{
    flex-grow: 1;
    display: flex;
    justify-content: center;
}


.prod {
    margin: 20px 0;
    display: flex;
    align-items: center;
    padding: 20px;
    border-radius: 10px;
    background-color: darkgrey;
    /* width: fit-content; */
    min-width: 30vw;
    justify-content: center;
}
</style>