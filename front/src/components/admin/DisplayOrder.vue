<script setup lang="ts">
import type { Order } from '@/scripts/api/admin/order-management';
import { f_price } from '@/scripts/utils';
import Tag from 'primevue/tag';

defineProps<{ selected_order: Order }>()
defineEmits<{served_clicked: []}>()
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
    <div class="data">
        <div class="infos">
            <span>{{ selected_order.user_email }}</span>
            <span>{{ fmtDate(selected_order) }}</span>
            <span class="receipt">{{ selected_order.receipt }}</span>
            <div class="served" @click="$emit('served_clicked')">
                <span>Servie:</span>
                <i v-if="selected_order.served" class="pi pi-check" style="color: green"></i>
                <i v-else class="pi pi-times" style="color: red"></i>
            </div>
        </div>
        <Tag :value="f_price(selected_order.total_price)"></Tag>
    </div>
</template>

<style scoped>
.receipt {
    font-size: small;
}

.served {
    display: flex;
    justify-content: left;
    align-items: center;
    gap: 10px;
}

.orders {
    width: 100%;
}

.data {
    display: flex;
    gap: 20px;
    background-color: transparent;
    padding: 10px;
    border-radius: 5px;
    align-items: center;
    justify-content: space-between;
    border: 1px solid #666666;
}


.infos {
    display: flex;
    flex-direction: column;
    gap: 5px;
}
</style>