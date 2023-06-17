<script setup lang="ts">
import { NButton, NSpace, NInput, NSelect, NDynamicInput } from 'naive-ui';
import { ref } from 'vue';
import type { Entry, Exec, Timer } from '@/types';

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

const editModalActionType = ref("");
const editModalTriggerType = ref("");

const editingEntry = ref({} as Entry)

const handleActionTypeUpdate = (value: string) => {
    switch (value) {
        case "exec":
            editingEntry.value.action = {} as Exec;
    }
}

const handleTriggerTypeUpdate = (value: string) => {
    switch (value) {
        case "timer":
            editingEntry.value.trigger = {} as Timer;
    }
}

</script>

<template>
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
        <n-spce vertical v-if='editModalTriggerType == "timer"'>
            <div></div>
        </n-spce>
        <n-button type="primary">Save</n-button>
    </n-space>
</template>