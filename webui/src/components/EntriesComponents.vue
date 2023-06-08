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

const actionTypeOptions = [
  {
    label: "Exec",
    value: "exec"
  }
]
const triggerTypeOptions = [
  {
    label: "Cron",
    value: "cron"
  }
]

const showEditModal = ref(false);
const editModalActionType = ref("");
const editModalTriggerType = ref("");

const editingEntry = ref({} as Entry)

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
  editingEntry.value = entry;
  showEditModal.value = true;
  switch (typeof entry.action) {
    case "object":
      console.log(Object.entries(entry.action)[0][0].toLowerCase())
      editModalActionType.value = Object.entries(entry.action)[0][0].toLowerCase();
      break;
  }
}

const handleActionTypeUpdate = (value: string) => {
  switch (value) {
    case "exec":
      editingEntry.value.action = {} as Exec;
  }
}

const handleTriggerTypeUpdate = (value: string) => {
  switch (value) {
    case "cron":
      editingEntry.value.trigger = {} as Cron;
  }
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
    <n-space vertical>
      <div>
        Name:
        <n-input v-model:value="editingEntry.name" label="Name"></n-input>
      </div>
      <div>
        ActionType:
        <n-select @update:value="handleActionTypeUpdate" v-model:value="editModalActionType" label="ActionType"
          :options="actionTypeOptions">
        </n-select>
      </div>
      <n-spce vertical v-if='editModalActionType == "exec"'>
        <div>
          Executable:
          <n-input v-model:value="(editingEntry.action as Exec).Exec.executable" placeholer="Executable"></n-input>
        </div>
        <div>
          Args:
          <n-dynamic-input v-model:value="(editingEntry.action as Exec).Exec.args" placeholder="Args" :min="1" />
        </div>
        <div>
          Env:
          <n-dynamic-input v-model:value="(editingEntry.action as Exec).Exec.env" placeholder="Env" :min="1" />
        </div>
        <div>
          WorkingDir:
          <n-input v-model:value="(editingEntry.action as Exec).Exec.working_dir" placeholder="WorkingDir"></n-input>
        </div>
      </n-spce>
      <div>
        TriggerType:
        <n-select @update:value="handleTriggerTypeUpdate" v-model:value="editModalTriggerType" label="TriggerType"
          :options="triggerTypeOptions">
        </n-select>
      </div>
      <n-button type="primary">Save</n-button>
    </n-space>
  </n-modal>
</template>