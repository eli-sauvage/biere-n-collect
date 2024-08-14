<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { loadStripe } from "@stripe/stripe-js";
import type { Stripe, StripeElement, StripeElements } from "@stripe/stripe-js";
import SrMessages from "./SrMessages.vue";
import { useRoute } from "vue-router";
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
        const { publishableKey } = await fetch("http://127.0.0.1:8000/api/config").then((res) => res.json()) as { publishableKey: string };
        let config_res = await loadStripe(publishableKey);
        if (!config_res) {
            return
        }
        stripe = config_res;

        const intent_res = await fetch(`http://127.0.0.1:8000/api/create-payment-intent?order_id=${order_id}`).then((res) => res.json());
        let clientSecret = intent_res.clientSecret as string;
        total_price.value = (intent_res.total_price as number/100).toFixed(2) + "€";

        messages.value.push(`Client secret returned.`);

        elements = stripe.elements({ clientSecret });
        const paymentElement = elements.create('payment');
        paymentElement.mount("#payment-element");
        const linkAuthenticationElement = elements.create("linkAuthentication");
        linkAuthenticationElement.mount("#link-authentication-element");
        isLoading.value = false;
    } catch (e) {
        if (e){
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
</script>
<template>
    <main>
        <h1>Payment</h1>
        <h2>Total à payer : {{ total_price }}</h2>

        <form id="payment-form" @submit.prevent="handleSubmit">
            <div id="link-authentication-element" />
            <div id="payment-element" />
            <button id="submit" :disabled="isLoading">
                Pay now
            </button>
            <sr-messages :messages="messages" />
        </form>
    </main>
</template>