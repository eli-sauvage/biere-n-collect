<script setup lang="ts">
import { close_bar, get_bar, list_reports, open_bar, set_closing_message, type Bar } from '@/scripts/api/admin/bar-management';
import Button from 'primevue/button';
import ConfirmPopup from 'primevue/confirmpopup';
import { useConfirm } from 'primevue/useconfirm';
import Textarea from 'primevue/textarea';
import Panel from 'primevue/panel';
import { ref, type Ref } from 'vue';
import { base } from '@/scripts/api/api';

let bar: Ref<Bar | null> = ref(null);
let new_closing_message = ref("");
let reports: Ref<string[]> = ref([])
const refresh = async () => {
    bar.value = await get_bar();
    if (bar.value)
        new_closing_message.value = bar.value?.closing_message
    reports.value = await list_reports()
}
refresh()

let confirm = useConfirm()
const confirm_close = (event: Event) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message: 'Etes-vous sûr ? Les commandes en cours non payées seront annulées et le rapport de la soirée sera généré',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true
        },
        acceptProps: {
            label: 'Fermer',
            severity: 'danger'
        },
        accept: async () => {
            await close_bar()
            bar.value = await get_bar()
        },
        reject: () => { }
    });
};
const confirm_open = (event: Event) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message: 'Etes-vous sûr ? Les commandes seront ouvertes pour les clients.',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true
        },
        acceptProps: {
            label: 'Ouvrir',
            severity: 'success'
        },
        accept: async () => {
            await open_bar()
            bar.value = await get_bar()
        },
        reject: () => { }
    });
};

let closing_msg_loading = ref(false)
const update_closing_msg = async () => {
    closing_msg_loading.value = true
    let res = await set_closing_message(new_closing_message.value)
    closing_msg_loading.value = false
    if (!res) return
    refresh()
}

</script>
<template>
    <ConfirmPopup></ConfirmPopup>
    <Button v-if="bar?.is_open" icon="pi pi-lock" label="Fermer les commandes" severity="danger" @click="confirm_close"
        class="open-close-btn" size="large"></Button>
    <Button v-if="bar && !bar.is_open" icon="pi pi-lock-open" label="Ouvrir les commandes" severity="success"
        @click="confirm_open" class="open-close-btn" size="large"></Button>
    <Panel header="Message de fermeture" style="margin-top: 10px;">
        <p>Message affiché sur la page d'acceuil lorsque le bar est fermé :</p>
        <div style="display: flex; justify-content: center;align-items: center; gap: 10px;">
            <Textarea v-model="new_closing_message"></Textarea>
            <Button icon="pi pi-check" :disabled="new_closing_message == bar?.closing_message"
                @click="update_closing_msg" :loading="closing_msg_loading"></Button>
        </div>
    </Panel>
    <Panel header="Historique des comptes-rendus d'ouverture" style="margin-top: 10px;">
        <a v-for="report in reports" :href="`${base}/admin/bar/reports/${report}`">{{ report }}</a>
    </Panel>
</template>
<style scoped>
.open-close-btn {
    width: 100%;
}
</style>