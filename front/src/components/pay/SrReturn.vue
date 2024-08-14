<script setup lang="ts">
import { ref, onMounted, computed, type Ref } from "vue";
import { useRoute } from "vue-router";
import { loadStripe, type Stripe } from "@stripe/stripe-js";

import SrMessages from "./SrMessages.vue";

const messages: Ref<string[]> = ref([]);
const clientSecret = ref('');


const currentRoute = computed(() => {
    return useRoute().query;
});
clientSecret.value = currentRoute.value?.payment_intent_client_secret as string;

let stripe: Stripe;

onMounted(async () => {
    const { publishableKey } = await fetch("/api/config").then((res) => res.json());
    let s = await loadStripe(publishableKey);
    if (!s) return
    stripe = s;

    const { error, paymentIntent } = await stripe.retrievePaymentIntent(
        clientSecret.value,
    );

    if (error) {
        messages.value.push(error.message || "undef error");
    }

    if (!paymentIntent){
        messages.value.push("payment intent in undef")
        return
    }
    messages.value.push(`Payment ${paymentIntent.status}: ${paymentIntent.id}`)
});

</script>

<template>

    <body>
        <main>
            <a href="/">home</a>
            <h1>Thank you!</h1>
            <sr-messages v-if="clientSecret" :messages="messages" />
        </main>
    </body>
</template>