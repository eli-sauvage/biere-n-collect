<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import {
    create_challenge,
    get_current_auth,
    verify_challenge,
} from '@/scripts/api/admin/auth'

let router = useRouter()
const route = useRoute()
let code = ref('')

let challenge_created = ref(false)

let btn_loading = ref(false)
let email = ref(localStorage.getItem('email') || '')

;(async () => {
    let auth = await get_current_auth()
    if (auth && auth.authenticated) {
        router.push('/serveur')
    }
    if (route && route.query && route.query.email && route.query.code) {
        code.value = route.query.code as string
        email.value = route.query.email as string
        btn_loading.value = true
        challenge_created.value = true
        validate()
    }
})()

async function validate() {
    if (!challenge_created.value) {
        //email
        if (email.value.length == 0) return
        btn_loading.value = true
        localStorage.setItem('email', email.value)
        let challenge = await create_challenge(email.value)
        btn_loading.value = false
        if (challenge) {
            challenge_created.value = true
        }
    }
    if (challenge_created.value) {
        if (code.value.length == 0) return
        btn_loading.value = true
        let code_fmt = code.value.replace(/ - /g, '')
        let verify = await verify_challenge(email.value, code_fmt)
        btn_loading.value = false
        if (verify) {
            router.push('/serveur')
        }
    }
}
</script>

<template>
    <form class="container">
        <div v-if="!challenge_created" class="email-container">
            <p>Entrez votre adresse mail :</p>
            <InputText
                id="email"
                aria-describedby="email-help"
                v-if="!challenge_created"
                type="email"
                v-model="email"
            />
        </div>

        <div v-if="challenge_created" class="otp-container">
            <p>Entrez le code reçu par mail :</p>
            <InputMask
                v-model="code"
                mask="99 - 99 - 99"
                placeholder="xx - xx - xx"
                class="otp"
            />
        </div>
        <Button
            type="submit"
            label="Valider"
            class="btn-valider"
            @click="validate"
            :loading="btn_loading"
        ></Button>
    </form>
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
    align-items: center;
}

.email-container {
    display: flex;
    align-items: center;
    flex-direction: column;
}

.email-container small {
    color: #e2b42c;
}

.otp-container * {
    text-align: center;
}

.btn-valider {
    margin-top: 5%;
}
</style>
