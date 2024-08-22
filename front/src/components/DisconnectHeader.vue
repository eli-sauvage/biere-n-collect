<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import Button from 'primevue/button';
useRoute()
defineProps<{ page: "admin" | "serveur", isAdmin?: boolean }>()
const router = useRouter()

let disconnecting = ref(false)

async function disconnect() {
    disconnecting.value = true;
    await fetch(`${import.meta.env.VITE_API_URL}/session/end`, { method: "POST", credentials: "include" }).then((r) => r.json());
    router.push("/login")
}

</script>
<template>
    <div class="header">
        <Button icon="pi pi-sign-out" @click="disconnect" :loading="disconnecting" severity="secondary"></Button>
        <Button v-if="page == 'admin'" icon="pi pi-user" iconPos="top" label="serveur" @click="router.push('/serveur')"
            :loading="disconnecting" severity="secondary"></Button>
        <Button icon="pi pi-cog" iconPos="top" label="admin" @click="router.push('/admin')"
            v-if="isAdmin && page == 'serveur'" :loading="disconnecting" severity="secondary"></Button>
    </div>
</template>

<style scoped>
.header {
    display: flex;
    flex-direction: row-reverse;
    justify-content: end;
    gap: 10px;
    width: 100%;
    padding: 10px;
    align-items: center;
}
</style>