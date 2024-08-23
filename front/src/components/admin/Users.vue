<script setup lang="ts">
import type { User } from '@/types';
import Accordion from 'primevue/accordion';
import AccordionContent from 'primevue/accordioncontent';
import AccordionHeader from 'primevue/accordionheader';
import AccordionPanel from 'primevue/accordionpanel';
import ConfirmPopup from 'primevue/confirmpopup';
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import Select from 'primevue/select';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import { useConfirm } from 'primevue/useconfirm';
import { ref, type Ref } from 'vue';
import { escapeLeadingUnderscores } from 'typescript';
let props = defineProps<{ currentUserEmail: string }>()
const confirm = useConfirm();

let selectedUser = ref(null);

let users: Ref<User[]> = ref([]);
let existing_roles: Ref<{ role: "admin" | "waiter", translated: string }[]> = ref([
    { role: "admin", translated: "admin" },
    { role: "waiter", translated: "serveur" }
]);

let user_to_add: Ref<{ email: string, role: "admin" | "waiter" } | null> = ref(null);
async function addUser() {
    if (user_to_add.value == null) return
    let res = await fetch(`${import.meta.env.VITE_API_URL}/users?email=${encodeURIComponent(user_to_add.value.email)}&role=${user_to_add.value.role}`, { method: "POST", credentials: "include" }).then(e => e.json());
    console.log(res)
    user_to_add.value = null
    refreshUsers()
}

async function refreshUsers() {
    let res_users = await fetch(`${import.meta.env.VITE_API_URL}/users/get_all`, { credentials: "include" }).then(e => e.json());
    users.value = res_users
}
refreshUsers()

async function change_role(email: string, new_role: string) {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/users/update_role?email=${encodeURIComponent(email)}&new_role=${new_role}`, { method: "PATCH", credentials: "include" }).then(e => e.json());
    console.log(res)
    refreshUsers()
}

async function disconnect_user(email: string) {
    let res = await fetch(`${import.meta.env.VITE_API_URL}/users/disconnect?email=${encodeURIComponent(email)}`, { method: "PATCH", credentials: "include" }).then(e => e.json());
    console.log(res)
    selectedUser.value = null
    refreshUsers()
}

const confirm_delete = (event: Event, email: string) => {
    confirm.require({
        target: event.currentTarget as HTMLInputElement,
        message: 'Etes-vous sûr ?',
        icon: 'pi pi-info-circle',
        rejectProps: {
            label: 'Annuler',
            severity: 'secondary',
            outlined: true
        },
        acceptProps: {
            label: 'Supprimer',
            severity: 'danger'
        },
        accept: async () => {
            let res = await fetch(`${import.meta.env.VITE_API_URL}/users?email=${encodeURIComponent(email)}`, { method: "DELETE", credentials: "include" }).then(e => e.json());
            console.log(res)
            refreshUsers()
        },
        reject: () => { }
    });
};
</script>

<template>
    <Button icon="pi pi-user" label="ajouter un utilisateur" class="add-user"
        @click="user_to_add = { email: '', role: 'waiter' }"></Button>

    <Dialog v-if="user_to_add != null" :visible="user_to_add != null" modal header="Edit Profile" :draggable="false"
        :closable="false">
        <div class="inputs">
            <label for="email">Email</label>
            <InputText id="email" v-model="user_to_add.email" />
            <label for="role">Role</label>
            <Select id="role" v-model="user_to_add.role" optionLabel="translated" :options="existing_roles"
                option-value="role"></Select>
        </div>
        <div class="footer">
            <Button type="button" label="Annuler" severity="secondary" @click="user_to_add = null"></Button>
            <Button type="button" label="Valider" @click="addUser"></Button>
        </div>
    </Dialog>
    <Accordion :value="selectedUser">
        <AccordionPanel v-for="(user, index) in users" :value="index.toString()">
            <AccordionHeader>
                <span>{{ user.email }}</span>
                <Tag v-if="user.role == 'waiter'" icon="pi pi-user" value="serveur"></Tag>
                <Tag v-if="user.role == 'admin'" icon="pi pi-shield" value="admin"></Tag>
            </AccordionHeader>
            <AccordionContent>
                <div class="user-details">
                    <div class="left">
                        <div class="change-role">
                            <span>Role: </span>
                            <Select :model-value="users[index].role" optionLabel="translated" :options="existing_roles"
                                option-value="role" @change="(e) => change_role(user.email, e.value)"
                                :disabled="user.email == currentUserEmail"></Select>
                        </div>
                        <p>Déconnecter l'utilisateur</p>
                        <Button icon="pi pi-sign-out" :badge="user.sessions.toString() + ' sessions'"
                            @click="disconnect_user(user.email)" :disabled="user.email == currentUserEmail"></Button>
                    </div>

                    <ConfirmPopup></ConfirmPopup>
                    <Button icon="pi pi-trash" severity="danger" @click="(e) => confirm_delete(e, user.email)"
                        :disabled="user.email == currentUserEmail" size="large"></Button>

                </div>
            </AccordionContent>
        </AccordionPanel>
    </Accordion>
</template>

<style scoped>
.add-user {
    margin-bottom: 10px;
}

.left {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.left>p {
    margin-bottom: 0;
}


.user-details {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.footer {
    margin-top: 20px;
    display: flex;
    justify-content: space-between;
}

.inputs {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.inputs>label {
    margin-top: 10px;
}
</style>