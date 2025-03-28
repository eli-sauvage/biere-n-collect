<script setup lang="ts">
import { ref, type Ref } from 'vue'
import DisplayOrder from '../admin/DisplayOrder.vue'
import {
    search_orders,
    type Order,
} from '../../scripts/api/admin/order-management'

defineEmits<{ selectOrder: [order: Order | null] }>()
let visible = defineModel<boolean>()
let date_search: Ref<[Date, Date] | null> = ref(null)
let email_search: Ref<string | null> = ref(null)
let receipt_search: Ref<string | null> = ref(null)

let search_result: Ref<Order[]> = ref([])

const startSearch = async (e: Event) => {
    e.preventDefault()
    if (date_search.value != null) {
        if (date_search.value[1] == null) {
            date_search.value[1] = new Date(date_search.value[0])
        }
        date_search.value[0].setHours(0, 0, 0, 0)
        date_search.value[1].setHours(23, 59, 59, 999)
    }
    let res = await search_orders(
        email_search.value,
        date_search.value,
        receipt_search.value
    )
    if (res == null) {
        console.log('res is null')
        visible.value = false
        return
    }
    search_result.value = res
    console.log(res)
}
const closeModal = () => {
    search_result.value = []
    visible.value = false
}
</script>
<template>
    <Dialog
        modal
        header="Commande séléctionnée"
        :visible
        @update:visible="
            (v) => {
                if (v == false) closeModal()
            }
        "
        v-if="visible"
        :dismissableMask="true"
    >
        <form class="recherche">
            <label for="date-search-order">Chercher par date</label>
            <DatePicker
                v-model="date_search"
                id="date-search-order"
                :manualInput="false"
                :maxDate="new Date()"
                selection-mode="range"
                showButtonBar
            />
            <label for="email-search-order">Chercher par mail</label>
            <InputText v-model="email_search" id="email-search-order" />
            <label for="receipt-search-order">Chercher par n° de reçu</label>
            <InputText v-model="receipt_search" id="receipt-search-order" />
            <Button
                type="submit"
                icon="pi pi-search"
                size="large"
                @click="startSearch"
            ></Button>
        </form>
        <Panel>
            <DisplayOrder
                v-for="order in search_result"
                :selected_order="order"
                @click="$emit('selectOrder', order)"
            />
            <div class="orders" v-if="search_result.length == 0">
                Les résultats de la recherche s'afficheront ici
            </div>
        </Panel>
    </Dialog>
</template>
<style scoped>
.recherche {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.recherche label {
    margin-top: 10px;
}

.recherche > button {
    margin-top: 20px;
    width: 50%;
}

.orders {
    display: flex;
    flex-direction: column;
    gap: 3px;
}
</style>
