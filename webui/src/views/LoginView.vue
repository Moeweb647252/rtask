<script lang="ts" setup>
import { NCard, NSpace, NInput, NButton } from 'naive-ui';
import { ref } from 'vue';
import axios from 'axios';
import { useRouter } from 'vue-router';

const router = useRouter();

const token = ref('');

const login = () => {
  if (token.value) {
    axios.post('/api/vaildToken', { token: token.value })
      .then((res) => {
        if (res.data.code === 200) {
          router.push('/')
        } else {
          alert('Login failed');
        }
      })
      .catch((err) => {
        alert('Login failed');
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
      <n-space style="margin-top: 10px;" justify="center">
        <n-button type="primary">Login</n-button>
      </n-space>
    </n-card>
  </n-space>
</template>