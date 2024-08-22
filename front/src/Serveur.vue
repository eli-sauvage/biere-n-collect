<script setup lang="ts">
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import { ref, type Ref } from 'vue';
import DisconnectHeader from './components/DisconnectHeader.vue';
let router = useRouter();

type role = "admin" | "waiter" | null;
let role: Ref<role> = ref(null);
(async () => {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/challenge/get_auth`, { credentials: "include" }).then((r) => r.json())
    if (!res.authenticated) {
        router.push("/login")
    } else if (res.role && res.role == "admin" || res.role == "waiter") {
        role.value = res.role
    }
})()


</script>
<template>
    <DisconnectHeader page="serveur" :isAdmin="role == 'admin'" />
    <h1>Serveur panel #todo</h1>
</template>