<script setup lang="ts">
import { useConfirm } from 'primevue/useconfirm'
import { ref, type Ref } from 'vue'
import { watch } from 'vue'
import {
    close_bar,
    get_bar,
    open_bar,
    set_closing_message,
    type Bar,
} from '@/scripts/api/admin/bar-management'
import { get_bar_openings, type BarOpening } from '@/scripts/api/admin/reports'

let bar: Ref<Bar | null> = ref(null)
let new_closing_message = ref('')
let openings: Ref<BarOpening[]> = ref([])
const refresh = async () => {
    bar.value = await get_bar()
    if (bar.value) new_closing_message.value = bar.value?.closing_message
    openings.value = await get_bar_openings()
    console.log(openings.value)
}
refresh()

let confirm = useConfirm()
const confirm_close = (event: Event) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message:
            'Etes-vous sûr ? Les commandes en cours non payées seront annulées et le rapport de la soirée sera généré',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true,
        },
        acceptProps: {
            label: 'Fermer',
            severity: 'danger',
        },
        accept: async () => {
            await close_bar()
            refresh()
        },
        reject: () => {},
    })
}
const confirm_open = (event: Event) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message:
            'Etes-vous sûr ? Les commandes seront ouvertes pour les clients.',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true,
        },
        acceptProps: {
            label: 'Ouvrir',
            severity: 'success',
        },
        accept: async () => {
            await open_bar()
            refresh()
        },
        reject: () => {},
    })
}

let closing_msg_loading = ref(false)
const update_closing_msg = async () => {
    closing_msg_loading.value = true
    let res = await set_closing_message(new_closing_message.value)
    closing_msg_loading.value = false
    if (!res) return
    refresh()
}

let customReportDates: Ref<[Date, Date] | null> = ref(null)
let customReportLink: Ref<string | null> = ref(null)
watch(customReportDates, async (new_dates) => {
    console.log('here')
    if (new_dates == null || new_dates[0] == null || new_dates[1] == null) {
        console.log('ret')
        console.log(new_dates)
        return
    }
    new_dates[0].setHours(0, 0, 0, 0)
    new_dates[1].setHours(23, 59, 59, 999)
    customReportLink.value = `/admin/report?begin=${new_dates[0].getTime()}&end=${new_dates[1].getTime()}`
})
</script>
<template>
    <div class="container">
        <ConfirmPopup></ConfirmPopup>
        <Button
            v-if="bar?.is_open"
            icon="pi pi-lock"
            label="Fermer les commandes"
            severity="danger"
            @click="confirm_close"
            class="open-close-btn"
            size="large"
        ></Button>
        <Button
            v-if="bar && !bar.is_open"
            icon="pi pi-lock-open"
            label="Ouvrir les commandes"
            severity="success"
            @click="confirm_open"
            class="open-close-btn"
            size="large"
        ></Button>
        <Panel header="Message de fermeture" style="margin-top: 10px">
            <p>
                Message affiché sur la page d'acceuil lorsque le bar est fermé :
            </p>
            <div
                style="
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    gap: 10px;
                "
            >
                <Textarea v-model="new_closing_message"></Textarea>
                <Button
                    icon="pi pi-check"
                    :disabled="new_closing_message == bar?.closing_message"
                    @click="update_closing_msg"
                    :loading="closing_msg_loading"
                ></Button>
            </div>
        </Panel>
        <Panel
            header="Historique des comptes-rendus d'ouverture"
            style="margin-top: 10px"
        >
            <div class="custom-range">
                <FloatLabel class="custom-range-flabel">
                    <label for="date-search-order">Période Customisée</label>
                    <DatePicker
                        v-model="customReportDates"
                        id="date-search-order"
                        :manualInput="false"
                        :maxDate="new Date()"
                        selection-mode="range"
                        showButtonBar
                    />
                </FloatLabel>
                <Button
                    v-if="customReportLink"
                    as="router-link"
                    icon="pi pi-file-export"
                    :to="customReportLink"
                    class="open-report"
                ></Button>
            </div>
            <DataTable :value="openings">
                <Column
                    :field="(e: BarOpening) => e.begin.toLocaleString('FR-fr')"
                    header="Début"
                ></Column>
                <Column
                    :field="(e: BarOpening) => e.end.toLocaleString('FR-fr')"
                    header="Fin"
                ></Column>
                <Column header="Compte rendu">
                    <template #body="slotProps">
                        <Button
                            as="router-link"
                            icon="pi pi-file-export"
                            :to="`/admin/report?begin=${slotProps.data.begin.getTime()}&end=${slotProps.data.end.getTime()}`"
                            class="open-report"
                        ></Button>
                    </template>
                </Column>
            </DataTable>
        </Panel>
    </div>
</template>
<style scoped>
.container {
    padding: 20px;
}

.open-close-btn {
    width: 100%;
}

.custom-range {
    margin-top: 20px;
    display: flex;
    justify-content: space-evenly;
}

.open-report {
    text-decoration: none;
}
</style>
