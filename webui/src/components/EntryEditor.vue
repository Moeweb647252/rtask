<script setup lang="ts">
import { NButton, NSpace, NInput, NSelect, NDynamicInput } from 'naive-ui';
import { ref, computed, onMounted } from 'vue';
import type { Entry, ExecAction, Execute, Timer, Duration } from '@/types';
import Record

const actionTypeOptions = [
  {
    label: "Exec",
    value: "exec"
  }
]
const triggerTypeOptions = [
  {
    label: "Timer",
    value: "timer"
  }
]

const timerTypeOptions = [
  {
    label: "Repeat",
    value: "repeat"
  },
  {
    label: "Once",
    value: "once"
  },
  {
    label: "ManyTimes",
    value: "manytimes"
  },
  {
    label: "Never",
    value: "never"
  }
]

const editingActionType = ref("");
const editingTriggerType = ref("");
const editingTimerType = ref("");

const editingEntry = ref({} as Entry)

const handleActionTypeUpdate = (value: string) => {
  switch (value) {
    case "exec":
      editingEntry.value.action = {
        Exec: {} as Execute
      };
  }
}

const handleTriggerTypeUpdate = (value: string) => {
  switch (value) {
    case "timer":
      editingEntry.value.trigger = {
        Timer: {} as Timer
      };
  }
}

const handleTimerTypeUpdate = (value: string) => {
  switch (value) {
    case "Repeat":
      (editingEntry.value.trigger as { Timer: Timer }).Timer = {
        Repeat: {} as Duration
      };
  }
}

const env_arr = ref([] as { key: string, value: string }[])
const args_arr = ref([] as string[])

const waitforuse_env = (value: { key: string, value: string }[]) => {
  (editingEntry.value.action as ExecAction).Exec.env = value.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);
}

onMounted(() => {
  env_arr.value = Object.entries((editingEntry.value.action as ExecAction).Exec.env || {}).map(([key, value]) => {
    return {
      key,
      value
    }
  })
})

</script>

<template>
  <n-space vertical>
    <div>
      Name:
      <n-input v-model:value="editingEntry.name" label="Name"></n-input>
    </div>
    <div>
      ActionType:
      <n-select @update:value="handleActionTypeUpdate" v-model:value="editingActionType" label="ActionType"
        :options="actionTypeOptions">
      </n-select>
    </div>
    <n-spce vertical v-if='editingActionType == "exec"'>
      <div>
        Executable:
        <n-input v-model:value="(editingEntry.action as ExecAction).Exec.executable" placeholer="Executable"></n-input>
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
        <n-input v-model:value="(editingEntry.action as ExecAction).Exec.working_dir" placeholder="WorkingDir"></n-input>
      </div>
    </n-spce>
    <div>
      TriggerType:
      <n-select @update:value="handleTriggerTypeUpdate" v-model:value="editingTriggerType" label="TriggerType"
        :options="triggerTypeOptions">
      </n-select>
    </div>
    <n-spce vertical v-if='editingTriggerType == "timer"'>
      <div>
        <n-select @update:value="handleTimerTypeUpdate" v-model:value="editingTimerType" label="TimerType"
          :options="timerTypeOptions">
        </n-select>

      </div>
    </n-spce>
    <n-button type="primary">Save</n-button>
  </n-space>
</template>