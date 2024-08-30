<script setup lang="ts">
import { ref, onMounted, computed, type Ref } from "vue";
import { useRoute } from "vue-router";
import { loadStripe, type Stripe } from "@stripe/stripe-js";

import SrMessages from "./SrMessages.vue";
import { Error, get_stripe_pub_key } from "@/scripts/api/api";
import { get_payment_status } from "@/scripts/api/order";

const clientSecret = ref('');

const receipt = ref("...")


const currentRoute = computed(() => {
    return useRoute().query;
});
clientSecret.value = currentRoute.value?.payment_intent_client_secret as string;

let stripe: Stripe;

onMounted(async () => {
    const publishableKey = await get_stripe_pub_key()
    if (publishableKey == null) return
    let s = await loadStripe(publishableKey);
    if (!s) {
        new Error("impossible de charger l'api stripe", "")
        return
    }
    stripe = s;

    let error_title = "Erreur lors de la récupération du paiement auprès de Stripe"

    const { error, paymentIntent } = await stripe.retrievePaymentIntent(
        clientSecret.value,
    );

    if (error) {
        new Error(error_title, error.message || "error inatendue");
    }

    if (!paymentIntent) {
        new Error(error_title, "payment intent in undefined");
        return
    }
    

    let payment_status = await get_payment_status(paymentIntent.id, clientSecret.value)
    if(payment_status && payment_status.receipt){
        receipt.value = payment_status.receipt
    }
});

</script>

<template>
    <a href="/">home</a>
    <h1>Thank you!</h1>
    <p>{{ receipt }}</p>
</template>