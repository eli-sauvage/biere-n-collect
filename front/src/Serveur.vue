<script setup lang="ts">
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import { ref, type Ref, onMounted, onUnmounted } from 'vue';
import DisconnectHeader from './components/DisconnectHeader.vue';
import { get_current_auth } from './scripts/api/admin/auth';
import { get_orders as get_all_orders, get_order_by_id, get_order_by_receipt, set_served, type Order } from "./scripts/api/admin/order-management";
import QrScanner from "qr-scanner"
import DatePicker from 'primevue/datepicker';
import InputText from 'primevue/inputtext';
import Dialog from 'primevue/dialog';
import DisplayOrder from './components/admin/DisplayOrder.vue';
import Panel from 'primevue/panel';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Row from 'primevue/row';
import ColumnGroup from 'primevue/columngroup';
import { f_price } from './scripts/utils';
let router = useRouter();

type role = "admin" | "waiter" | null;
let role: Ref<role> = ref(null);
(async () => {
  let auth = await get_current_auth()
  if (auth && auth.authenticated && auth.role && (auth.role == "admin" || auth.role == "waiter")) {
    role.value = auth.role
  } else {
    router.push("/login")
  }
})()

let orders: Ref<Order[]> = ref([])
let selected_order: Ref<Order | null> = ref(null)

const select_order = (order: Order) => {
  search_dialog_visible.value = false;
  selected_order.value = order;
}
const setServed = async (order: Order, served: boolean) => {
  let res = await set_served(order, served);
  if (!res) return
  selected_order.value = null
}

const toggle_served = async () => {
  if (selected_order.value == null) return
  let res = await set_served(selected_order.value, !selected_order.value?.served)
  if (!res) return
  let order = await get_order_by_id(selected_order.value.id)
  if (order)
    selected_order.value = order
}

let qrScanner: QrScanner;
let isScanning = ref(true);
let toggleScanning = async () => {
  if (isScanning.value) {
    qrScanner.stop();
    isScanning.value = false;
  } else {
    await qrScanner.start();
    isScanning.value = true;
  }
};


onMounted(async () => {
  qrScanner = new QrScanner(
    document.getElementById("serveurQrScannerVideoElem") as HTMLVideoElement,
    async result => {
      let data = result.data;
      if (data.match(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/)) {
        console.log('decoded qr code:', data)
        qrScanner.stop()
        isScanning.value = false;
        let order = await get_order_by_receipt(data);
        if (order == null) return
        selected_order.value = order;
      }
    },
    {
      highlightCodeOutline: true,
      highlightScanRegion: true
    },
  );

  qrScanner.start()
})
onUnmounted(async ()=>{
  if(isScanning.value){
    await toggleScanning()
  }
});



let search_dialog_visible = ref(false)
let date_search: Ref<[Date, Date] | null> = ref(null);
let email_search: Ref<string | null> = ref(null);
let receipt_search: Ref<string | null> = ref(null);

const startSearch = async (e: Event) => {
  e.preventDefault()
  selected_order.value = null;
  search_dialog_visible.value = true;
  if (date_search.value != null) {
    if (date_search.value[1] == null) {
      date_search.value[1] = new Date(date_search.value[0])
    }
    date_search.value[0].setHours(0, 0, 0, 0);
    date_search.value[1].setHours(23, 59, 59, 999);
  }
  let res = await get_all_orders(email_search.value, date_search.value, receipt_search.value);
  if (res == null) {
    search_dialog_visible.value = false;
    return
  }
  orders.value = res
}

</script>

<template>
  <DisconnectHeader page="serveur" :isAdmin="role == 'admin'" />
  <div class="container">
    <Button v-if="!isScanning" label="Scanner" icon="pi pi-qrcode" iconPos="top" size="large" @click="toggleScanning"
      class="scanBtn"></Button>
    <Button v-else label="Stopper le Scan" icon="pi pi-stop-circle" iconPos="top" size="large" @click="toggleScanning"
      class="scanBtn"></Button>
    <div class="scan-container" :style="`display: ${isScanning ? 'unset' : 'none'}`">
      <video id="serveurQrScannerVideoElem"></video>
    </div>
    <Panel v-if="!isScanning">
      <form class="recherche">
        <label for="date-search-order">Chercher par date</label>
        <DatePicker v-model="date_search" id="date-search-order" :manualInput="false" :maxDate="new Date()"
          selection-mode="range" showButtonBar />
        <label for="email-search-order">Chercher par mail</label>
        <InputText v-model="email_search" id="email-search-order" />
        <label for="receipt-search-order">Chercher par n° de reçu</label>
        <InputText v-model="receipt_search" id="receipt-search-order" />
        <Button type="validate" icon="pi pi-search" size="large" @click="startSearch"></Button>
      </form>
    </Panel>
  </div>
  <Dialog modal header="Commande séléctionnée" :visible="selected_order != null" @after-hide="console.log"
    :closable="false" v-if="selected_order != null">
    <DisplayOrder :selected_order="selected_order" @served_clicked="toggle_served" />
    <DataTable :value="selected_order.detail">
      <Column :field="(e:any)=> `${e.product_name}: ${e.variation_name}`" header="Article"></Column>
      <Column field="quantity" header="Quantité"></Column>
      <Column :field="(e: any) => f_price(e.subtotal_ttc)" header="Sous-total TTC"></Column>
      <ColumnGroup type="footer">
        <Row>
          <Column footer="Total HT:" :colspan="2" footerStyle="text-align:right" />
          <Column v-if="selected_order != null" :footer="f_price(selected_order.total_price_ht)" />
        </Row>
        <Row>
          <Column footer="Total TTC:" :colspan="2" footerStyle="text-align:right" />
          <Column v-if="selected_order != null" :footer="f_price(selected_order.total_price_ttc)" />
        </Row>
      </ColumnGroup>
    </DataTable>
    <div class="close-selected-order-btn">
      <Button v-if="selected_order.served" icon="pi pi-times" label="Marquer comme non-servie"
        @click="setServed(selected_order, false)" severity="secondary"></Button>
      <Button label="fermer" @click="selected_order = null" severity="secondary"></Button>
      <Button v-if="!selected_order.served" icon="pi pi-check" label="Commande servie"
        @click="setServed(selected_order, true)" severity="secondary"></Button>
    </div>
  </Dialog>
  <Dialog modal header="Résultats de la recherche" v-model:visible="search_dialog_visible">
    <div class="orders">
      <DisplayOrder v-for="order in orders" :selected_order="order" @click="select_order(order)" />
    </div>
  </Dialog>
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

.scanBtn {
  min-width: 200px;
  min-height: 100px;
}

.close-selected-order-btn {
  margin-top: 20px;
  display: flex;
  justify-content: space-between;
}

video {
  max-width: 90vw;
  max-height: 90vh;
}

.recherche {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.recherche label {
  margin-top: 10px;
}

.recherche>button {
  margin-top: 20px;
  width: 50%;
}

.orders {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.servie-toggle {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
  background-color: #DDDDDD;
  padding: 10px;
  border-radius: 10px;
}
</style>
