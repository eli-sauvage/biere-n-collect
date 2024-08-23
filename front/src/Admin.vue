<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import { useRouter } from 'vue-router';
import { type Product } from './types';
import Tabs from 'primevue/tabs';
import Tab from 'primevue/tab';
import TabPanel from 'primevue/tabpanel';
import TabList from 'primevue/tablist';
import TabPanels from 'primevue/tabpanels';
import DisconnectHeader from './components/DisconnectHeader.vue';
import Stock from './components/admin/Stock.vue';
import Users from './components/admin/Users.vue';
let router = useRouter();

let stock: Ref<Product[]> = ref([]);
let currentUserEmail: Ref<string> = ref("");

(async () => {
    let auth_res = await fetch(`${import.meta.env.VITE_API_URL}/challenge/get_auth`, { credentials: "include" }).then((r) => r.json())
    if (!auth_res.authenticated) {
        router.push("/login")
    } else if (auth_res.role) {
        if (auth_res.role != "admin") {
            router.push('/login')
        }
        currentUserEmail.value = auth_res.email
    }
})();

(async () => {
    let res_stock = await fetch(`${import.meta.env.VITE_API_URL}/stock/get`).then(e => e.json());
    stock.value = res_stock
})();


</script>

<template>
    <Tabs value="0" >
        <TabList>
            <Tab value="0">Stock</Tab>
            <Tab value="1">Comptes</Tab>
    <DisconnectHeader page="admin" />
        </TabList>
        <TabPanels>
            <TabPanel value="0">
                <Stock />
            </TabPanel>
            <TabPanel value="1">
                <Users :current-user-email="currentUserEmail" />
            </TabPanel>
        </TabPanels>
    </Tabs>
</template>


<style scoped>
</style>
<style>
.p-tabpanels{
    background: unset!important;
}
.p-tablist-tab-list{
    background: unset!important;
    background-color: #1b6589!important;
}
</style>