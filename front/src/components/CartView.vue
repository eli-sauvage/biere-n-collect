<script setup lang="ts">
import { f_price, type Cart, type Product, type ProductId } from '@/types';
import Button from "primevue/button"
import Tag from 'primevue/tag';
import { ref } from 'vue';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
let props = defineProps<{ cart: Cart }>();
const emit = defineEmits<{ validate: [email: string] }>()

let email_dialog_visible = ref(false);
let email = ref(localStorage.getItem("email") || "")

function validateEmail() {
    var re = /^\S+@\S+\.\S+$/;
    return re.test(email.value);
}

function validateCart(){
    if(email.value.length == 0 || !validateEmail()) return
    localStorage.setItem("email", email.value)
    emit("validate", email.value);
}
</script>
<template>
    <Dialog v-model:visible="email_dialog_visible" modal header="Informations" :closable="false">
        <div class="">
            <p for="email">Merci d'entrer votre e-mail :</p>
            <InputText class="email-input" id="email" v-model="email" :invalid="email.length != 0 && !validateEmail()"/>
        </div>
        <div class="">
            <Button type="button" label="Annuler" severity="secondary" @click="email_dialog_visible = false"></Button>
            <Button type="button" label="Valider" @click="validateCart"></Button>
        </div>
    </Dialog>

    <div class="cart">
        <div v-for="(elem, index) in cart.elems_with_subtotal()" class="item">
            <p class="product-name">{{ elem.cart_element.product.name }}</p>
            <div class="name-quantity">
                <Tag :value="'x' + elem.cart_element.quantity"></Tag>
                <p class="subtotal">{{ f_price(cart.elems_with_subtotal()[index].subtotal) }}</p>
            </div>
        </div>
        <div class="button">
            <Button class="valider" @click="email_dialog_visible = true" :badge="'Total: ' + cart.get_total()"
                badge-severity="contrast" label="Valider"></Button>
        </div>
    </div>
</template>
<style scoped>
.cart {
    display: flex;
    flex-direction: column;
    padding: 20px;
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

.email-input{
    margin-bottom: 20px;
}
</style>