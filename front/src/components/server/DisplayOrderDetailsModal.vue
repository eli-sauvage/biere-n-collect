<script setup lang="ts">
import DisplayOrder from '../admin/DisplayOrder.vue'
import {
    type OrderDetailElement,
    get_order_by_id,
    set_served,
    type Order,
    notify_client,
} from '../../scripts/api/admin/order-management'
import { f_price } from '../../scripts/utils'

let order = defineModel<Order | null>()
const refresh_order = async () => {
    if (order.value == null) return
    order.value = await get_order_by_id(order.value.id)
}
const setServed = async (order: Order, served: boolean) => {
    let res = await set_served(order, served)
    if (!res) return
    await refresh_order()
}

const notifyClient = async (order: Order) => {
    let res = await notify_client(order)
    if (!res) return
    await refresh_order()
}

const toggle_served = async () => {
    if (order.value == null) return
    let res = await set_served(order.value, !order.value?.served)
    if (!res) return
    refresh_order()
}
</script>

<template>
    <Dialog
        modal
        header="Commande séléctionnée"
        :visible="order != null"
        @after-hide="console.log"
        :closable="false"
        v-if="order != null"
    >
        <DisplayOrder :selected_order="order" @served_clicked="toggle_served" />
        <DataTable :value="order.detail">
            <Column
                :field="(e: OrderDetailElement) => e.item_name"
                header="Article"
            ></Column>
            <Column field="quantity" header="Quantité"></Column>
            <Column
                :field="(e: OrderDetailElement) => f_price(e.subtotal_ttc)"
                header="Sous-total TTC"
            ></Column>
            <ColumnGroup type="footer">
                <Row>
                    <Column
                        footer="Total TTC:"
                        :colspan="2"
                        footerStyle="text-align:right"
                    />
                    <Column
                        v-if="order != null"
                        :footer="f_price(order.total_price_ttc)"
                    />
                </Row>
            </ColumnGroup>
        </DataTable>
        <div class="close-selected-order-btn">
            <Button
                v-if="order.served"
                icon="pi pi-times"
                label="Marquer comme non-servie"
                @click="setServed(order, false)"
                severity="secondary"
            ></Button>
            <Button
                v-if="!order.served"
                icon="pi pi-check"
                label="Commande servie"
                @click="setServed(order, true)"
                severity="secondary"
            ></Button>
        </div>
        <div class="selected-order-footer">
            <Button
                label="Fermer"
                @click="order = null"
                severity="secondary"
            ></Button>
            <Button
                icon="pi pi-bell"
                label="Notifier client"
                :disabled="order.client_notified"
                @click="notifyClient(order)"
                severity="secondary"
            ></Button>
        </div>
    </Dialog>
</template>
<style scoped>
.close-selected-order-btn {
    margin-top: 20px;
    display: flex;
    justify-content: end;
}

.selected-order-footer {
    margin-top: 5px;
    display: flex;
    justify-content: space-between;
}
</style>
