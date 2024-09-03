<script setup lang="ts">
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import { ref, type Ref, onMounted } from 'vue';
import DisconnectHeader from './components/DisconnectHeader.vue';
import { get_current_auth } from './scripts/api/admin/auth';
import { get_all as get_all_orders, type Order } from "./scripts/api/admin/order-management";
import QrScanner from "qr-scanner"
import Tag from 'primevue/tag';
import { f_price } from './scripts/utils';
import type { RefSymbol } from '@vue/reactivity';
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

let qrScanner: QrScanner;
let isScanning = ref(false);
let toggleScanning = async () => {
  if (isScanning.value) {
    qrScanner.stop();
    isScanning.value = false;
  } else {
    await qrScanner.start();
    isScanning.value = true;
  }
};

(async () => {
  orders.value = await get_all_orders();
  console.log(orders.value)
})()

onMounted(async () => {
  qrScanner = new QrScanner(
    document.getElementById("serveurQrScannerVideoElem") as HTMLVideoElement,
    result => {
      let data = result.data;
      if (data.match(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/)) {
        console.log('decoded qr code:', data)
        qrScanner.stop()
        isScanning.value = false;
      }
    },
    {
      highlightCodeOutline: true,
      highlightScanRegion: true
    },
  );

})

const fmtDate = (order: Order): string => {
  let timestamp = order.timestamp
  try {
    let date = new Date(timestamp);
    var options = { year: "2-digit", month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" };
    return date.toLocaleString("FR-fr", options as any)
  } catch (e) {
    console.error(e)
    return timestamp.toString()
  }
}
</script>

<template>
  <DisconnectHeader page="serveur" :isAdmin="role == 'admin'" />
  <div class="container">
    <Button v-if="!isScanning" label="Scanner" icon="pi pi-qrcode" iconPos="top" size="large"
      @click="toggleScanning"></Button>
    <Button v-else label="Stop" icon="pi pi-stop-circle" iconPos="top" size="large" @click="toggleScanning"></Button>
    <video id="serveurQrScannerVideoElem" :style="`display: ${isScanning?'unset':'none'}`"></video>
    <div class="orders">
      <div v-for="order in orders" class="data">
        <div class="infos">
          <span>{{ order.user_email }}</span>
          <span>{{ fmtDate(order) }}</span>
          <span class="receipt">{{ order.receipt }}</span>
        </div>
        <Tag :value="f_price(order.total_price)"></Tag>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  margin: 5px
}

video {
  max-width: 90vw;
  max-height: 90vh;
}

.receipt {
  font-size: small;
}

.served {
  display: flex;
  justify-content: left;
  align-items: center;
  gap: 10px;
}

.data {
  display: flex;
  gap: 20px;
  background-color: white;
  padding: 10px;
  border-radius: 5px;
  margin-bottom: 3px;
  align-items: center;
  min-width: 95vw;
  justify-content: space-between;
}

.infos {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
</style>
