<script setup lang="ts">
import { defineProps, computed, toRef } from 'vue';

const props = defineProps<{
    messages: string[]
}>();

const messagesRef = toRef(props, 'messages');

const splitMessages = computed(() => {
    return messagesRef.value.map((x) => {

        const paymentIntentRe = /(pi_(\S*)\b)/
        const paymentIntentMatch = x.match(paymentIntentRe)
        return {
            ...(paymentIntentMatch && { paymentIntent: paymentIntentMatch[0] }),
            content: x.replace(paymentIntentRe, '') || x
        }
    });
})

const addDashboardLinks = (paymentIntent: string) => {
    return `https://dashboard.stripe.com/test/payments/${paymentIntent}`;
};

</script>
<template>
    <div id="messages" role="alert">
        <span v-for="message in splitMessages">
            > {{ message.content }}<a v-if="message.paymentIntent" :href="addDashboardLinks(message.paymentIntent)">{{
                message.paymentIntent }}</a>
            <br>
        </span>
    </div>
</template>