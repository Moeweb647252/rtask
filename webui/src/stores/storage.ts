import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useStore = defineStore('storage', {
  state: () => ({
    token: ref<string | null>(null)
  }),
  actions: {
    setToken(token: string) {
      this.token = token
    }
  },
  getters: {
    getToken(): string | null {
      return this.token
    }
  },
}
)
