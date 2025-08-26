import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAuthStore = defineStore('auth', () => {
  const userId = ref<string | null>(localStorage.getItem('userId'));
  const isAuthenticated = ref(!!userId.value);

  const login = (id: string) => {
    userId.value = id;
    isAuthenticated.value = true;
    localStorage.setItem('userId', id);
  };

  const logout = () => {
    userId.value = null;
    isAuthenticated.value = false;
    localStorage.removeItem('userId');
  };

  return {
    userId,
    isAuthenticated,
    login,
    logout
  };
});
