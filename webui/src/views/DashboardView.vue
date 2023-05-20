<script setup lang="ts">
import { NLayout, NLayoutHeader, NSpace, NDropdown, NAvatar, NLayoutSider, NIcon, NH2, NMenu } from 'naive-ui';
import { CaretDownOutline, List } from '@vicons/ionicons5'
import type { MenuOption } from 'naive-ui';
import { h, ref } from 'vue';
import { useStore } from '@/stores/storage';
import type { Component } from 'vue';
import Entries from '@/components/EntriesComponents.vue';

const store = useStore();

const collapsed = ref(false);
const currentSection = ref(Entries)

const expandIcon = () => h(NIcon, null, { default: () => h(CaretDownOutline) });
const renderIcon = (icon: Component) => () => h(NIcon, null, { default: () => h(icon) })
const avatarOptions = [
  {
    label: 'Logout',
    key: 'logout',
  },
];

const menuOptions: MenuOption[] = [
  {
    label: 'Entries',
    icon: renderIcon(List),
  }
]

const handleAvatarDpSelect = (option: string) => {
  if (option == 'logout') {
    console.log('logout')
    store.setToken(null)
  }
}

//const renderMenuLabel = (option: MenuOption) => option.label;

const onUpdate = (value: MenuOption) => {
  if (value.label == 'Entries') {
    currentSection.value = Entries
  }
}

</script>

<template>
  <n-layout style="height: 100%" position="absolute">
    <n-layout-header style="height: 64px; padding: 12px" bordered>
      <n-space justify="space-between">
        <n-h2 style="display: flex;align-items: center;">Rtodo</n-h2>
        <n-space>
          <n-dropdown trigger="hover" :options="avatarOptions" @select="handleAvatarDpSelect">
            <n-avatar round size="medium" src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" />
          </n-dropdown>
        </n-space>
      </n-space>
    </n-layout-header>
    <n-layout position="absolute" style="top: 64px; bottom: 64px" has-sider>
      <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed"
        show-trigger @collapse="collapsed = true" @expand="collapsed = false">
        <n-menu :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions"
          :expand-icon="expandIcon" :on-update:value="onUpdate" />
      </n-layout-sider>
      <n-layout content-style="padding: 24px;" :native-scrollbar="false">
        <component :is="currentSection" />
      </n-layout>
    </n-layout>
  </n-layout>
</template>