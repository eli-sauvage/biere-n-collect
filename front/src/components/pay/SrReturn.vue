<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

import Button from "primevue/button";
import { base, Error, get_stripe_pub_key } from "@/scripts/api/api";
import { get_payment_status, get_qr_code_url, type PaymentStatus } from "@/scripts/api/order";

const clientSecret: Ref<string | null> = ref(null);
const payment_intent_id: Ref<string | null> = ref(null)

const payment_status: Ref<PaymentStatus | null> = ref(null)

const currentRoute = useRoute().query as any;
console.log(currentRoute)
if (currentRoute && currentRoute.payment_intent_client_secret) {
    clientSecret.value = currentRoute.payment_intent_client_secret as string;
}
if (currentRoute && currentRoute.payment_intent) {
    payment_intent_id.value = currentRoute.payment_intent as string;
}

onMounted(async () => {

    if (clientSecret.value == null) {
        new Error("impossible de récupérer le client_secret depuis l'url", "")
        return
    }
    if (payment_intent_id.value == null) {
        new Error("impossible de récupérer le client_secret depuis l'url", "")
        return
    }

    let res = await get_payment_status(payment_intent_id.value, clientSecret.value)
    if (res) {
        payment_status.value = res
    }
});
let router = useRouter();
function return_home() {
    router.push({ path: "/" })
}
</script>

<template>
    <Button icon="pi pi-home" severity="secondary" class="return" @click="return_home"></Button>
    <div class="container" v-if="payment_status?.receipt != null">
        <span class="merci">Merci pour votre commande</span>
        <span>Voici votre reçu à présenter au bar</span>
        <img class="qr-code" v-if="clientSecret != null && payment_intent_id != null"
            :src="get_qr_code_url(payment_intent_id, clientSecret)"
            :alt="' qr code indisponible! montrez ce code à la place :      ' + payment_status.receipt" />
    </div>

</template>

<style scoped>
.return {
    position: fixed;
    top: 30px;
    left: 30px;
}

.container {
    margin-top: 80px;
    display: flex;
    flex-direction: column;
    gap: 30px;
    align-items: center;
}

span.merci {
    font-size: x-large;
}

img.qr-code {
    width: 90%;
}

.receipt {
    font-size: small;
}
</style>