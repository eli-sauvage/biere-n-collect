<script setup lang="ts">
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import { ref, type Ref } from 'vue';
import DisconnectHeader from './components/DisconnectHeader.vue';
import { get_current_auth } from './scripts/api/admin/auth';
let router = useRouter();

type role = "admin" | "waiter" | null;
let role: Ref<role> = ref(null);
(async () => {
    let auth = await get_current_auth()
    if(auth && auth.authenticated && auth.role && (auth.role == "admin" || auth.role == "waiter")){
        role.value = auth.role
    }else{
        router.push("/login")
    }
})()


</script>
<template>
    <DisconnectHeader page="serveur" :isAdmin="role == 'admin'" />
    <h1>Serveur panel #todo</h1>
</template>