<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref, type Ref } from 'vue'

import DisconnectHeader from './components/DisconnectHeader.vue'
import SearchOrders from './components/server/SearchModal.vue'
import DisplayOrderDetails from './components/server/DisplayOrderDetailsModal.vue'
import QrScanner from './components/server/QrScannerModal.vue'

import { get_current_auth } from './scripts/api/admin/auth'
import { type Order } from './scripts/api/admin/order-management'

let router = useRouter()

type role = 'admin' | 'waiter' | null
let role: Ref<role> = ref(null)
;(async () => {
    let auth = await get_current_auth()
    if (
        auth &&
        auth.authenticated &&
        auth.role &&
        (auth.role == 'admin' || auth.role == 'waiter')
    ) {
        role.value = auth.role
    } else {
        router.push('/login')
    }
})()

// let orders: Ref<Order[]> = ref([])
let selected_order: Ref<Order | null> = ref(null)

const select_order = (order: Order | null) => {
    search_dialog_visible.value = false
    selected_order.value = order
}

let search_dialog_visible = ref(false)
let isScanning = ref(false)
</script>

<template>
    <DisconnectHeader page="serveur" :isAdmin="role == 'admin'" />
    <div class="container">
        <div class="sticky-buttons">
            <Button
                label="Recherche commande"
                icon="pi pi-search"
                iconPos="left"
                size="large"
                @click="search_dialog_visible = !search_dialog_visible"
            ></Button>
            <Button
                :label="isScanning ? 'Stopper le scan' : 'Scanner'"
                :icon="`pi pi-${isScanning ? 'stop-circle' : 'qrcode'}`"
                iconPos="left"
                size="large"
                @click="isScanning = !isScanning"
                class="scan-btn"
            ></Button>
        </div>
    </div>
    <QrScanner v-model="isScanning" @select-order="select_order" />
    <DisplayOrderDetails v-model="selected_order" />
    <SearchOrders
        v-model="search_dialog_visible"
        @select-order="select_order"
    />
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 50px;
    margin: 5px;
    margin-top: 30px;
    min-width: 95vw;
}

.sticky-buttons {
    position: fixed;
    display: flex;
    flex-direction: column;
    gap: 5px;
    bottom: 10px;
    right: 10px;
}

.sticky-buttons Button.scan-btn {
    padding-top: 20px;
    padding-bottom: 20px;
    /* min-width: 200px;
    min-height: 100px; */
}
</style>
