<script setup lang="ts">
import { NList, NListItem, NThing, NCard, NButton, NSpace, NModal, NInput, NSelect, NDynamicInput } from 'naive-ui';
import axios from 'axios';
import { CONFIG } from '../config';
import { useStore } from '@/stores/storage';
import { useDialog } from 'naive-ui';
import { ref, onMounted } from 'vue';
import type { Ref } from 'vue';
import type { Entry, Exec } from '@/types';

const store = useStore();
const dialog = useDialog();

const showEditModal = ref(false);

const entries: Ref<Entry[]> = ref([]);

const fetchEntries = async () => {
  axios.post(CONFIG.api_addr + "/api/getEntries",
    {
      token: store.getToken
    }).then(
      (resp) => {
        if (resp.data.code == 200) {
          entries.value = resp.data.data
        } else {
          dialog.error({
            content: "cannot get entries!"
          })
        }
      }
    ).catch(
      () => {
        dialog.error({
          content: "cannot get entries!"
        })
      }
    )
};

const handleEditEntry = (entry: Entry) => {

}

onMounted(() => {
  fetchEntries()
})


</script>

<template>
  <n-card style="word-break: keep-all;">
    <n-list>
      <n-list-item v-for="entry in entries">
        <template #prefix>
          <h2>{{ entry.name?.toString() }}</h2>
        </template>
        <template #suffix>
          <n-button type="info" @click="handleEditEntry(entry)">Edit</n-button>
        </template>
        <n-space>
          <n-thing title="Action" :description="Object.entries(entry.action)[0][0]"></n-thing>
          <n-thing title="Trigger" :description="Object.entries(entry.trigger)[0][0]"></n-thing>
        </n-space>
      </n-list-item>
    </n-list>
  </n-card>
  <n-modal v-model:show="showEditModal" title="Edit Entry" preset="card" style="width: auto;">

  </n-modal>
</template>