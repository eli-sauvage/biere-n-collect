<script setup lang="ts">
import { ref, onMounted, type Ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { loadStripe, type Stripe, type StripeElements } from '@stripe/stripe-js'

import { f_price } from '@/scripts/utils'
import {
    get_payment_infos,
    set_email,
    get_stripe_pub_key,
} from '@/scripts/api/order'
import { Error } from '@/scripts/api/api'

let order_id = useRoute().query.order_id

let is_component_mounted = ref(false)

const isLoading = ref(false)
const total_price: Ref<string> = ref('')
const client_secret: Ref<string | null> = ref(null)
const email: Ref<string> = ref('')

let stripe: Stripe
let elements: StripeElements

let loading_canceled = ref(false)

onMounted(async () => {
    try {
        const publishableKey = await get_stripe_pub_key()
        if (publishableKey == null) return
        let config_res = await loadStripe(publishableKey)
        if (!config_res) {
            loading_canceled.value = true
            return
        }
        stripe = config_res

        let parsed_order_id = parseInt(order_id as string)
        if (
            typeof parsed_order_id != 'number' ||
            Number.isNaN(parsed_order_id)
        ) {
            loading_canceled.value = true
            return
        }
        let payment_infos = await get_payment_infos(parsed_order_id)
        console.log(payment_infos)
        if (payment_infos == null) {
            loading_canceled.value = true
            return
        }
        client_secret.value = payment_infos.client_secret
        total_price.value = f_price(payment_infos.total_price)

        elements = stripe.elements({
            clientSecret: payment_infos.client_secret,
        })
        const paymentElement = elements.create('payment')
        paymentElement.mount('#payment-element')
        const linkAuthenticationElement = elements.create(
            'linkAuthentication',
            {
                defaultValues: { email: localStorage.getItem('email') || '' },
            }
        )
        linkAuthenticationElement.mount('#link-authentication-element')
        linkAuthenticationElement.on('change', (e) => {
            email.value = e.value.email
        })
        isLoading.value = false
        is_component_mounted.value = true
    } catch (e: any) {
        new Error('erreur inattendue', e.toString())
    }
})

const handleSubmit = async () => {
    if (isLoading.value) {
        return
    }

    isLoading.value = true

    if (client_secret.value) {
        localStorage.setItem('email', email.value)
        let res = await set_email(client_secret.value, email.value)
        if (!res) return
    } else {
        new Error("erreur lors de l'envoi de l'adresse mail", '')
    }

    const { error } = await stripe.confirmPayment({
        elements,
        confirmParams: {
            return_url: `${window.location.origin}/return`,
        },
    })

    if (error.message) {
        new Error('erreur de paiement', `${error.type || ''} ${error.message}`)
    } else {
        new Error('erreur de paiement', error.toString() || error.type)
    }

    isLoading.value = false
}

let router = useRouter()
function return_home() {
    router.push({ path: '/' })
}
</script>
<template>
    <Button
        icon="pi pi-home"
        severity="secondary"
        class="return"
        @click="return_home"
    ></Button>
    <h1>Paiement</h1>
    <div class="form-container">
        <ProgressSpinner
            v-if="!(is_component_mounted || loading_canceled)"
            style="display: block"
        />
        <h2 v-if="is_component_mounted">Total à payer : {{ total_price }}</h2>

        <form id="payment-form" @submit.prevent="handleSubmit">
            <div id="link-authentication-element"></div>
            <div id="payment-element"></div>
            <Button
                v-if="is_component_mounted"
                id="submit"
                type="submit"
                :disabled="isLoading"
                :loading="isLoading"
                :label="`Payer ${total_price}`"
            />
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
