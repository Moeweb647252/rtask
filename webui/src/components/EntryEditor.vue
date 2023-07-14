<script setup lang="ts">
import { NButton, NSpace, NInput, NSelect, NDynamicInput } from 'naive-ui';
import { ref, onMounted } from 'vue';
import type { EditingEntry } from '@/types';

const actionTypeOptions = [
  {
    label: "Exec",
    value: "Exec"
  }
]

const triggerTypeOptions = [
  {
    label: "Timer",
    value: "Eimer"
  }
]

const timerTypeOptions = [
  {
    label: "Repeat",
    value: "Eepeat"
  },
  {
    label: "Once",
    value: "Once"
  },
  {
    label: "ManyTimes",
    value: "Manytimes"
  },
  {
    label: "Never",
    value: "Never"
  }
]



const editingEntry = ref({} as EditingEntry)


const env_arr = ref([] as { key: string, value: string }[])
const args_arr = ref([] as string[])

/*
const waitforuse_env = (value: { key: string, value: string }[]) => {
  (editingEntry.value.action as ExecAction).Exec.env = value.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);
}
*/

</script>

<template>
  <n-space vertical>
    <div>
      Name:
      <n-input v-model:value="editingEntry.name" label="Name"></n-input>
    </div>
    <div>
      ActionType:
      <n-select v-model:value="editingEntry.action.type" label="ActionType" :options="actionTypeOptions">
      </n-select>
    </div>
    <n-spce vertical v-if='editingEntry.action.type == "Exec"'>
      <div>
        Executable:
        <n-input v-model:value="editingEntry.action.content.Exec.executable" placeholer="Executable"></n-input>
      </div>
      <div>
        Args:
        <n-dynamic-input v-model:value="args_arr" placeholder="Args" :min="1" />
      </div>
      <div>
        Env:
        <n-dynamic-input v-model:value="env_arr" preset="pair" placeholder="Env" :min="1" />
      </div>
      <div>
        WorkingDir:
        <n-input v-model:value="editingEntry.action.content.Exec.working_dir" placeholder="WorkingDir"></n-input>
      </div>
    </n-spce>
    <div>
      TriggerType:
      <n-select v-model:value="editingEntry.trigger.type" label="TriggerType" :options="triggerTypeOptions">
      </n-select>
    </div>
    <n-spce vertical v-if='editingEntry.trigger.type == "Timer"'>
      <div>
        <n-select v-model:value="editingEntry.trigger.content.Timer.type" label="TimerType" :options="timerTypeOptions">
        </n-select>
        <n-space vertical, v-if="editingEntry.trigger.content.Timer.type == 'Repeat'">
          Repeat Every:
          
          <n-date-picker v-model:value="editingEntry.trigger.content.Timer.content.timestamp" type="datetime" clearable />
        </n-space>
      </div>
    </n-spce>
    <n-button type="primary">Save</n-button>
  </n-space>
</template>