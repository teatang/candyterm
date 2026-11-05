<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 bg-slate-950/70 backdrop-blur-md flex items-center justify-center p-4 select-none transition-all"
  >
    <div class="w-full max-w-md bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl shadow-2xl p-6 text-slate-800 dark:text-slate-100 animate-in fade-in zoom-in-95 duration-150 transition-colors">
      <div class="flex items-center space-x-3 mb-4">
        <div class="w-10 h-10 rounded-lg bg-blue-600/20 text-blue-500 dark:text-blue-400 flex items-center justify-center text-xl">
          🔒
        </div>
        <div>
          <h3 class="text-base font-bold text-slate-900 dark:text-slate-100">
            {{ isInitialized ? t('auth.title') : t('auth.initTitle') }}
          </h3>
          <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
            {{ isInitialized ? t('auth.subtitle') : t('auth.initSubtitle') }}
          </p>
        </div>
      </div>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <div>
          <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('auth.masterPassword') }}</label>
          <input
            v-model="password"
            data-test="pwd"
            type="password"
            class="w-full px-3 py-2 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            :placeholder="t('auth.masterPassword')"
            autofocus
          />
        </div>

        <div v-if="!isInitialized">
          <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('auth.confirmPassword') }}</label>
          <input
            v-model="confirmPassword"
            data-test="confirm-pwd"
            type="password"
            class="w-full px-3 py-2 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            :placeholder="t('auth.confirmPassword')"
          />
        </div>

        <!-- 错误提示 -->
        <div v-if="errorMessage" class="text-xs text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-900/50 rounded p-2">
          {{ errorMessage }}
        </div>

        <div class="pt-2">
          <button
            data-test="submit-btn"
            type="button"
            class="w-full py-2.5 px-4 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white font-medium text-sm rounded-lg shadow-lg shadow-blue-600/20 transition-all flex items-center justify-center space-x-2"
            @click="handleSubmit"
          >
            <span>{{ isInitialized ? t('auth.unlock') : t('auth.initialize') }}</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 主密码安全解锁与首次初始化模态框
 * 提供 Argon2id + AES-256-GCM 本地加密保险箱的解锁保护
 */
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  show: boolean
  isInitialized: boolean
}>()

const emit = defineEmits<{
  (e: 'submit', password: string): void
}>()

const { t } = useI18n()
const password = ref('')
const confirmPassword = ref('')
const errorMessage = ref('')

function handleSubmit() {
  errorMessage.value = ''

  if (!password.value.trim()) {
    errorMessage.value = t('auth.masterPassword') + ' ' + (t('connection.hostRequired') ? '不得为空 / cannot be empty' : '不得为空')
    return
  }

  if (!props.isInitialized) {
    if (password.value !== confirmPassword.value) {
      errorMessage.value = t('auth.passwordMismatch')
      return
    }
  }

  emit('submit', password.value)
}
</script>
