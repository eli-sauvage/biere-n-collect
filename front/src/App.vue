<script setup lang="ts">
import { useToast } from 'primevue/usetoast';
import { set_toast } from './scripts/api/api';
import Toast from 'primevue/toast';
set_toast(useToast())
</script>

<template>
  <main>
    <Toast class="error-toast"/>
    <RouterView />
  </main>
</template>

<style>
.error-toast{
  max-width: calc(100vw - 40px);
}
</style>
