<script lang="ts" setup>
import { ref, type Ref } from 'vue'
import { useRouter } from 'vue-router'

import DisconnectHeader from './components/DisconnectHeader.vue'
import Stock from './components/admin/Stock.vue'
import Users from './components/admin/Users.vue'
import Bar from './components/admin/Bar.vue'
import { get_current_auth } from './scripts/api/admin/auth'

let router = useRouter()

// let stock: Ref<Product[]> = ref([]);
let currentUserEmail: Ref<string | null> = ref(null)

;(async () => {
    let auth = await get_current_auth()
    if (
        auth &&
        auth.authenticated &&
        auth.email &&
        auth.role &&
        auth.role == 'admin'
    ) {
        currentUserEmail.value = auth.email
    } else {
        router.push('/login')
    }
})()
</script>

<template>
    <Tabs v-if="currentUserEmail" value="0">
        <TabList>
            <Tab value="0">Bar</Tab>
            <Tab value="1">Stock</Tab>
            <Tab value="2">Comptes</Tab>
            <DisconnectHeader page="admin" />
        </TabList>
        <TabPanels>
            <TabPanel value="0">
                <Bar />
            </TabPanel>
            <TabPanel value="1">
                <Stock />
            </TabPanel>
            <TabPanel value="2">
                <Users :current-user-email="currentUserEmail" />
            </TabPanel>
        </TabPanels>
    </Tabs>
</template>

<style scoped></style>
<style>
.p-tabpanels {
    background: unset !important;
    padding: 0 !important;
}

.p-tablist-tab-list {
    background: unset !important;
    background-color: #1b6589 !important;
}
</style>
