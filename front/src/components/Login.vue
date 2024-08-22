<script setup lang="ts">
import InputOtp from 'primevue/inputotp';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import { ref, type Ref } from 'vue';
import { useRouter } from 'vue-router';
let router = useRouter()
let code = ref("");

let challenge_created = ref(false);

let btn_loading = ref(false);
let email = ref("");

let message = ref("");

(async () => {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/challenge/get_auth`, {credentials: "include"}).then((r) => r.json())
    if(res.authenticated && res.authenticated == true){
        router.push("/serveur")
    }
})()

async function validate() {
    if (!challenge_created.value) { //email
        if (email.value.length == 0) return
        btn_loading.value = true;
        let res = await fetch(`${import.meta.env.VITE_API_URL}/challenge/create?email=${email.value}`, { method: "POST", credentials: "same-origin" }).then((r) => r.json())
        btn_loading.value = false
        if (res.error) {
            message.value = res.error
        } else if (res.success == true) {
            challenge_created.value = true
        }
    }
    if (challenge_created.value) {
        if (code.value.length == 0) return
        btn_loading.value = true
        let res = await fetch(`${import.meta.env.VITE_API_URL}/challenge/verify?email=${email.value}&code=${code.value}`, { method: "POST", credentials: "include" }).then((r) => r.json())
        btn_loading.value = false;
        if (res.error) {
            message.value = res.error;
        } else if (res.success == true) {
            router.push("/serveur")
        }
    }
}
</script>

<template>
    <div class="container">
        <div v-if="!challenge_created" class="email-container">
            <p>Entrez votre adresse mail :</p>
            <InputText id="email" aria-describedby="email-help" v-if="!challenge_created" type="email"
                v-model="email" />
            <small id="email-help">{{ message || "\xa0" }}</small>
        </div>

        <div v-if="challenge_created" class="otp-container">
            <p>Entrez le code reçu par mail :</p>
            <InputOtp v-model="code" :length="6">
                <template #default="{ attrs, events, index }">
                    <input type="number" v-bind="attrs" v-on="events" class="custom-otp-input" />
                    <div v-if="index == 2 || index == 4" class="px-4">
                        <i class="pi pi-minus separator"></i>
                    </div>
                </template>
            </InputOtp>
        </div>
        <Button label="Valider" class="btn-valider" @click="validate" :loading="btn_loading"></Button>
    </div>
</template>

<style scoped>
.container {
    background-color: #1b6589;
    margin: 3%;
    padding: 3%;
    border-radius: 10px;
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center
}

.email-container {
    display: flex;
    align-items: center;
    flex-direction: column;
}

.email-container small {
    color: #e2b42c;
}

.otp-container p {
    text-align: center;
}

.btn-valider {
    margin-top: 5%;
}

.custom-otp-input {
    width: 10%;
    height: 48px;
    font-size: 24px;
    appearance: none;
    text-align: center;
    transition: all 0.2s;
    border-radius: 0;
    border: 1px solid black;
    background: white;
    outline-offset: -2px;
    outline-color: transparent;
    border-right: 0 none;
    transition: outline-color 0.3s;
    color: black;
}

.custom-otp-input:focus {
    outline: 2px solid var(--p-focus-ring-color);
}

.custom-otp-input:first-child,
.custom-otp-input:nth-child(4),
.custom-otp-input:nth-child(7) {
    border-top-left-radius: 12px;
    border-bottom-left-radius: 12px;
}

.custom-otp-input:nth-child(2),
.custom-otp-input:nth-child(5),
.custom-otp-input:last-child {
    border-top-right-radius: 12px;
    border-bottom-right-radius: 12px;
    border-right-width: 1px;
    border-right-style: solid;
}

.separator {
    margin: 0 5px;
    color: black;
}

/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

/* Firefox */
input[type=number] {
    -moz-appearance: textfield;
}

.container .p-inputotp {
    gap: 0;
    justify-content: center !important;
}
</style>