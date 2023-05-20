<script setup lang="ts">
import { NList } from 'naive-ui';
import axios from 'axios';
import { CONFIG } from '../config';
import { useStore } from '@/stores/storage';
import { ref } from 'vue';

const store = useStore();

const entries: Ref<> = ref([]);

</script>

<template>
  <n-list>
    <n-list-item>1</n-list-item>

  </n-list>
</template>