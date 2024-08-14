<script setup lang="ts">
</script>

<template>
  <nav>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/admin">Go to Admin</RouterLink>
    <RouterLink to="/checkout">Go to checkout</RouterLink>
    <RouterLink to="/return">Go to return</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
</template>

<style scoped>
</style>
