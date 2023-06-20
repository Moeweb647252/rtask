<script lang="ts" setup>
import { NCard, NSpace, NInput, NButton, NCheckbox } from 'naive-ui';
import { ref } from 'vue';
import axios from 'axios';
import { useRouter } from 'vue-router';
import { useStore } from '../stores/storage'
import { CONFIG } from '../config';
import { useDialog } from 'naive-ui';

document.title = 'Rtask Login';

const dialog = useDialog();

const router = useRouter();

const token = ref('');

const store = useStore();

const remember_me = ref(false);

const login = () => {
  if (token.value) {
    axios.post(CONFIG.api_addr + '/api/validateToken', { token: token.value })
      .then((res) => {
        if (res.data.code === 200) {
          store.setRememberMe(remember_me.value);
          store.setToken(token.value);
          router.push('/')
        } else {
          dialog.error({
            content: 'Login failed',
          });
        }
      })
      .catch((err) => {
        dialog.error({
          content: 'Login failed',
        });
      });
  }
}

</script>

<template>
  <div style="min-height: 300px;"></div>
  <br>
  <n-space justify="center">
    <n-card style="min-width: 300px;" size="large">
      <h2 style="text-align: center">Rtodo Login</h2>
      <hr>
      <n-input v-model:value="token" placeholder="token" />
      <br>
      <n-checkbox v-model:checked="remember_me">Remember me</n-checkbox>
      <n-space style="margin-top: 10px;" justify="center">
        <n-button type="primary" v-on:click="login">Login</n-button>
      </n-space>
    </n-card>
  </n-space>
</template>