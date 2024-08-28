<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { loadStripe } from "@stripe/stripe-js";
import type { Stripe, StripeElement, StripeElements } from "@stripe/stripe-js";
import SrMessages from "./SrMessages.vue";
import { useRoute, useRouter } from "vue-router";
import { f_price } from "@/types";
import Button from "primevue/button";
// let props = defineProps<{ order_id: number }>();
let order_id = useRoute().query.order_id
console.log("-----")
console.log(order_id)

const isLoading = ref(false);
const messages: Ref<string[]> = ref([]);
const total_price: Ref<string> = ref("")

let stripe: Stripe;
let elements: StripeElements;

onMounted(async () => {
    try {
        const { publishableKey } = await fetch(`${import.meta.env.VITE_API_URL}/config`).then((res) => res.json()) as { publishableKey: string };
        let config_res = await loadStripe(publishableKey);
        if (!config_res) {
            return
        }
        stripe = config_res;

        const intent_res = await fetch(`${import.meta.env.VITE_API_URL}/create-payment-intent?order_id=${order_id}`).then((res) => res.json());
        let clientSecret = intent_res.clientSecret as string;
        total_price.value = f_price(intent_res.total_price as number);

        // messages.value.push(`Client secret returned.`);

        elements = stripe.elements({ clientSecret });
        const paymentElement = elements.create('payment');
        paymentElement.mount("#payment-element");
        isLoading.value = false;
    } catch (e) {
        if (e) {
            messages.value.push(e.toString())
        }
        console.error(e)
    }
});

const handleSubmit = async () => {
    if (isLoading.value) {
        return;
    }

    isLoading.value = true;
    console.log(elements)

    const { error } = await stripe.confirmPayment({
        elements,
        confirmParams: {
            return_url: `${window.location.origin}/return`
        }
    });

    if (error.message && (error.type === "card_error" || error.type === "validation_error")) {
        messages.value.push(error.message);
    } else {
        messages.value.push("An unexpected error occured.");
    }

    isLoading.value = false;
}

let router = useRouter();
function return_home() {
    router.push({ path: "/" })
    // router.push({ path: "/", query: { continue_order: order_id } })
}
</script>
<template>
    <Button icon="pi pi-home" severity="secondary" class="return" @click="return_home"></Button>
    <h1>Paiement</h1>
    <div class="form-container">
        <h2>Total à payer : {{ total_price }}</h2>

        <form id="payment-form" @submit.prevent="handleSubmit">
            <!-- <div id="link-authentication-element" @change="console.log"></div> -->
            <div id="payment-element" />
            <button id="submit" :disabled="isLoading">
                Payer {{ total_price }}
            </button>
            <sr-messages :messages="messages" />
        </form>
    </div>
</template>

<style scoped>
h1 {
    text-align: center;
    flex-grow: 1;
}

.return {
    position: fixed;
    top: 30px;
    left: 30px;
}

.form-container {
    background-color: #1b6589;
    margin: 3%;
    padding: 3%;
    border-radius: 10px;
}


#submit {
    margin-top: 40px;
    width: 100%;
    padding: 10px;
    font-size: larger;
    background-color: #e2b42c;
    color: black;
    box-shadow: none;
    border: none;
    border-radius: 5px;
}
</style>