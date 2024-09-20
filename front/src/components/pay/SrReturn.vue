<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

import Button from "primevue/button";
import { Error} from "@/scripts/api/api";
import { get_payment_status, get_qr_code_url, type PaymentStatus } from "@/scripts/api/order";
import { f_price } from "@/scripts/utils"
import DataTable from "primevue/datatable"
import Column from "primevue/column"
import ColumnGroup from "primevue/columngroup"
import Row from "primevue/row"
import ProgressSpinner from "primevue/progressspinner"
import { type OrderDetailElement } from "@/scripts/api/admin/order-management";

const clientSecret: Ref<string | null> = ref(null);
const payment_intent_id: Ref<string | null> = ref(null)

const payment_status: Ref<PaymentStatus | null> = ref(null)

const currentRoute = useRoute().query as any;
console.log(currentRoute)
if (currentRoute && currentRoute.payment_intent_client_secret) {
  clientSecret.value = currentRoute.payment_intent_client_secret as string;
}
if (currentRoute && currentRoute.payment_intent) {
  payment_intent_id.value = currentRoute.payment_intent as string;
}

onMounted(async () => {

  if (clientSecret.value == null) {
    new Error("impossible de récupérer le client_secret depuis l'url", "")
    return
  }
  if (payment_intent_id.value == null) {
    new Error("impossible de récupérer le client_secret depuis l'url", "")
    return
  }

  let res = await get_payment_status(clientSecret.value)
  if (res) {
    payment_status.value = res
  }
});
let router = useRouter();
function return_home() {
  router.push({ path: "/" })
}
</script>

<template>
  <Button icon="pi pi-home" severity="secondary" class="return" @click="return_home"></Button>
  <div v-if="payment_status == null" class="loading">
    <span>Récupération du paiement</span>
    <ProgressSpinner />
  </div>
  <div class="container" v-if="payment_status?.receipt != null">
    <span class="merci">Merci pour votre commande</span>
    <span>Voici votre reçu à présenter au bar</span>
    <span v-if="payment_status.email">Un email avec ce reçu a également été envoyé à {{ payment_status.email
      }}</span>
    <img class="qr-code" v-if="clientSecret != null && payment_intent_id != null" :src="get_qr_code_url(clientSecret)"
      :alt="' qr code indisponible! montrez ce code à la place :      ' + payment_status.receipt" />
    <div v-if="payment_status.detail.length != 0" class="recap">
      <p>Récapitulatif de la commande :</p>
      <DataTable :value="payment_status.detail">
        <Column :field="(e:OrderDetailElement) => e.item_name" header="Article"></Column>
        <Column field="quantity" header="Quantité"></Column>
        <Column :field="(e: OrderDetailElement) => f_price(e.subtotal_ttc)" header="Sous-total"></Column>
        <ColumnGroup type="footer">
          <Row>
            <Column footer="Total:" :colspan="2" footerStyle="text-align:right" />
            <Column :footer="f_price(payment_status.total_price)" />
          </Row>
        </ColumnGroup>
      </DataTable>
    </div>
  </div>

</template>

<style scoped>
.return {
  position: fixed;
  top: 30px;
  left: 30px;
}

.loading {
  margin-top: 80px;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.container {
  margin-top: 80px;
  display: flex;
  flex-direction: column;
  gap: 30px;
  align-items: center;
}

.container>span {
  text-align: center;
}

span.merci {
  font-size: x-large;
}

img.qr-code {
  width: 90%;
  max-height: 50vh;
}

.recap {
  margin-bottom: 30px;
}
</style>
