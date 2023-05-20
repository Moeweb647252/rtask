<script setup lang="ts">
import { RouterView } from 'vue-router'
import { NDialogProvider } from 'naive-ui';
</script>

<template>
  <n-dialog-provider>
    <RouterView />
  </n-dialog-provider>
</template>
