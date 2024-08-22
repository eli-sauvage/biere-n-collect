<script setup lang="ts">
import { f_price, type CartElement, type Product } from '@/types';
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';
import Tag from 'primevue/tag';
let props = defineProps<{ cardElement: CartElement }>();

function display_stock():string{
    let stock = props.cardElement.product.quantity;
    if(stock == 0){
        return "plus de stock !"
    }else if(stock < 10){
        return `${stock} restants`
    }else{
        return ""
    }
}

function allow_add_product(quantity: number):boolean{
    let new_quantity = props.cardElement.quantity + quantity;
    let stock = props.cardElement.product.quantity;
    return stock-new_quantity >= 0;
}

</script>

<template>
    <div class="prod">
        <div class="img">
            <img src="https://placehold.co/100/png" />
        </div>
        <div class="right">
            <div class="titre-price">
                <p class="titre">{{ cardElement.product.name }}</p>
                <Tag v-if="cardElement.quantity > 0" :value="f_price(cardElement.product.price)"></Tag>
            </div>
            <p class="stock">{{ display_stock() }}</p>
            <div class="price-add">
                <InputNumber v-if="cardElement.quantity > 0" :model-value="cardElement.quantity"
                    inputId="horizontal-buttons" showButtons buttonLayout="horizontal" :step="1" fluid
                    class="input-buttons" focused="false">
                    <template #incrementbutton>
                        <Button :disabled="!allow_add_product(1)"icon="pi pi-plus" severity="secondary" class="increment"
                            @click="(e) => { cardElement.add(1); e.preventDefault() }"></Button>
                    </template>
                    <template #decrementbutton="test">
                        <Button :icon="cardElement.quantity > 1 ? 'pi pi-minus' : 'pi pi-trash'" severity="secondary"
                            class="decrement" @click="(e) => { cardElement.add(-1); e.preventDefault() }"></Button>
                    </template>
                </InputNumber>
                <Tag v-if="cardElement.quantity == 0" :value="f_price(cardElement.product.price)"></Tag>
                <Button v-if="cardElement.quantity == 0" icon="pi pi-plus" severity="primary" class="add-to-cart"
                    @click="(e) => { cardElement.add(1); e.preventDefault() }" :disabled="cardElement.product.quantity == 0"></Button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* p {
    display: inline;
    margin: 0 10px 0 0;
} */

.right {
    margin-left: 20px;
    display: flex;
    flex-direction: column;
    justify-content: end;
    /* justify-content: space-between; */
    flex-grow: 1;
}

.increment {
    order: 3;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
}

.decrement {
    order: 1;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.stock {
    text-align: center;
    margin: 0;
}

.titre {
    /* margin-top: 0; */
    text-align: center;
    text-transform: capitalize;
    word-break: break-all;
}

.titre-price {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
}

.price-add {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
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