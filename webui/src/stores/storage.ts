import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useStore = defineStore('storage', {
  state: () => ({
    token_session: ref<string | null>(null),
    token_local: ref<string | null>(null),
    remember_me: ref<boolean>(false)
  }),
  actions: {
    setToken(token: string | null) {
      if (this.remember_me) {
        this.token_local = token
      } else {
        this.token_session = token
      }
    },
    setRememberMe(remember_me: boolean) {
      this.remember_me = remember_me
    }
  },
  getters: {
    getToken(): string | null {
      if (this.remember_me) {
        return this.token_local
      } else {
        return this.token_session
      }
    },
    getRememberMe(): boolean {
      return this.remember_me
    }
  },
  persist: [
    {
      paths: ['token_session'],
      storage: sessionStorage
    },
    {
      paths: ['token_local', 'remember_me'],
      storage: localStorage
    }
  ]
}
)
