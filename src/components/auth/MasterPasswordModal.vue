<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-4 select-none"
  >
    <div class="w-full max-w-md bg-slate-900 border border-slate-800 rounded-xl shadow-2xl p-6 text-slate-100 animate-in fade-in zoom-in-95 duration-150">
      <div class="flex items-center space-x-3 mb-4">
        <div class="w-10 h-10 rounded-lg bg-blue-600/20 text-blue-400 flex items-center justify-center text-xl">
          🔒
        </div>
        <div>
          <h3 class="text-base font-bold text-slate-100">
            {{ isInitialized ? '保险箱已锁定' : '首次使用：设置主密码' }}
          </h3>
          <p class="text-xs text-slate-400 mt-0.5">
            {{ isInitialized ? '请输入主密码解密本地服务器凭据与密钥' : '请设置一个高强度主密码，用于加密所有敏感凭据' }}
          </p>
        </div>
      </div>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <div>
          <label class="block text-xs font-medium text-slate-300 mb-1">主密码</label>
          <input
            v-model="password"
            data-test="pwd"
            type="password"
            class="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="输入主密码..."
            autofocus
          />
        </div>

        <div v-if="!isInitialized">
          <label class="block text-xs font-medium text-slate-300 mb-1">确认主密码</label>
          <input
            v-model="confirmPassword"
            data-test="confirm-pwd"
            type="password"
            class="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="再次输入主密码确认..."
          />
        </div>

        <!-- 错误提示 -->
        <div v-if="errorMessage" class="text-xs text-red-400 bg-red-950/40 border border-red-900/50 rounded p-2">
          {{ errorMessage }}
        </div>

        <div class="pt-2">
          <button
            data-test="submit-btn"
            type="button"
            class="w-full py-2.5 px-4 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white font-medium text-sm rounded-lg shadow-lg shadow-blue-600/20 transition-all flex items-center justify-center space-x-2"
            @click="handleSubmit"
          >
            <span>{{ isInitialized ? '解锁进入' : '创建加密保险箱' }}</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  show: boolean
  isInitialized: boolean
}>()

const emit = defineEmits<{
  (e: 'submit', password: string): void
}>()

const password = ref('')
const confirmPassword = ref('')
const errorMessage = ref('')

function handleSubmit() {
  errorMessage.value = ''

  if (!password.value.trim()) {
    errorMessage.value = '主密码不得为空'
    return
  }

  if (!props.isInitialized) {
    if (password.value !== confirmPassword.value) {
      errorMessage.value = '两次输入的密码不一致'
      return
    }
  }

  emit('submit', password.value)
}
</script>
