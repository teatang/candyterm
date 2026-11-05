<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 bg-slate-950/70 backdrop-blur-md flex items-center justify-center p-4 select-none transition-all"
  >
    <div class="w-full max-w-lg bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl shadow-2xl p-6 text-slate-800 dark:text-slate-100 animate-in fade-in zoom-in-95 duration-150 transition-colors">
      <div class="flex items-center justify-between mb-4 pb-3 border-b border-slate-200 dark:border-slate-800">
        <h3 class="text-base font-bold flex items-center space-x-2 text-slate-900 dark:text-slate-100">
          <span>💻</span>
          <span>{{ initialData ? t('connection.edit') : t('connection.new') }}</span>
        </h3>
        <button class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200" @click="emit('close')">✕</button>
      </div>

      <form @submit.prevent="handleSave" class="space-y-4">
        <!-- 会话名称与地址 -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.label') }}</label>
            <input
              v-model="form.label"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
              :placeholder="t('connection.label')"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.host') }} *</label>
            <input
              v-model="form.host"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
              placeholder="192.168.1.100 / example.com"
            />
          </div>
        </div>

        <!-- 端口与用户名 -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.port') }}</label>
            <input
              v-model.number="form.port"
              type="number"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.username') }} *</label>
            <input
              v-model="form.username"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        <!-- 认证类型切换 -->
        <div>
          <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.authType') }}</label>
          <div class="flex space-x-2">
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'Password' ? 'bg-blue-600/20 border-blue-500 text-blue-600 dark:text-blue-400 font-semibold' : 'bg-slate-100 dark:bg-slate-950 border-slate-300 dark:border-slate-700 text-slate-600 dark:text-slate-400'"
              @click="form.auth_type = 'Password'"
            >
              {{ t('connection.password') }}
            </button>
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'PrivateKey' ? 'bg-blue-600/20 border-blue-500 text-blue-600 dark:text-blue-400 font-semibold' : 'bg-slate-100 dark:bg-slate-950 border-slate-300 dark:border-slate-700 text-slate-600 dark:text-slate-400'"
              @click="form.auth_type = 'PrivateKey'"
            >
              {{ t('connection.privateKey') }}
            </button>
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'SshAgent' ? 'bg-blue-600/20 border-blue-500 text-blue-600 dark:text-blue-400 font-semibold' : 'bg-slate-100 dark:bg-slate-950 border-slate-300 dark:border-slate-700 text-slate-600 dark:text-slate-400'"
              @click="form.auth_type = 'SshAgent'"
            >
              {{ t('connection.sshAgent') }}
            </button>
          </div>
        </div>

        <!-- 密码输入框 -->
        <div v-if="form.auth_type === 'Password'">
          <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.password') }}</label>
          <input
            v-model="form.password"
            type="password"
            class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
            :placeholder="t('connection.password')"
          />
        </div>

        <!-- 私钥与口令输入框 -->
        <div v-if="form.auth_type === 'PrivateKey'" class="space-y-3">
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.privateKey') }}</label>
            <textarea
              v-model="form.private_key"
              rows="3"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-xs font-mono text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
              placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
            ></textarea>
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.passphrase') }}</label>
            <input
              v-model="form.passphrase"
              type="password"
              class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-sm text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        <!-- SSH 代理提示说明 -->
        <div v-if="form.auth_type === 'SshAgent'" class="bg-blue-50 dark:bg-blue-950/30 border border-blue-200 dark:border-blue-800/40 rounded-lg p-3 text-xs text-blue-700 dark:text-blue-300">
          <div class="font-medium flex items-center space-x-1.5 mb-1">
            <span>🛡️</span>
            <span>{{ t('connection.sshAgent') }}</span>
          </div>
          <p class="text-slate-600 dark:text-slate-400 leading-relaxed text-[11px]">
            {{ t('connection.sshAgentHint') || '将直接复用系统正在运行的 SSH 代理（如 ssh-agent、Pageant 或 1Password 等），无需在本地保存私钥。' }}
          </p>
        </div>

        <!-- 跳板机选择 (ProxyJump) -->
        <div>
          <label class="block text-xs font-medium text-slate-700 dark:text-slate-300 mb-1">{{ t('connection.jumpHost') }}</label>
          <select
            v-model="form.jump_host_id"
            class="w-full px-3 py-1.5 bg-slate-50 dark:bg-slate-950 border border-slate-300 dark:border-slate-700 rounded-lg text-xs text-slate-900 dark:text-slate-100 focus:outline-none focus:border-blue-500"
          >
            <option :value="undefined">{{ t('connection.none') }}</option>
            <option
              v-for="h in existingHosts.filter(x => x.id !== form.id)"
              :key="h.id"
              :value="h.id"
            >
              {{ h.label }} ({{ h.host }}:{{ h.port }})
            </option>
          </select>
        </div>

        <!-- 错误提示 -->
        <div v-if="errorMessage" class="text-xs text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-900/50 rounded p-2">
          {{ errorMessage }}
        </div>

        <div class="flex justify-end space-x-2 pt-2 border-t border-slate-200 dark:border-slate-800">
          <button
            type="button"
            class="px-4 py-2 text-xs font-medium text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg"
            @click="emit('close')"
          >
            {{ t('connection.cancel') }}
          </button>
          <button
            data-test="save-host-btn"
            type="button"
            class="px-5 py-2 text-xs font-medium bg-blue-600 hover:bg-blue-500 text-white rounded-lg shadow-md shadow-blue-600/20"
            @click="handleSave"
          >
            {{ t('connection.saveAndConnect') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 新建与编辑主机连接表单模态框
 * 支持密码、私钥（含口令）、SSH Agent 与 ProxyJump 跳板机级联配置
 */
import { ref, reactive } from 'vue'
import { useI18n } from 'vue-i18n'

export interface HostFormData {
  id: string
  label: string
  host: string
  port: number
  username: string
  auth_type: 'Password' | 'PrivateKey' | 'SshAgent'
  password?: string
  private_key?: string
  passphrase?: string
  jump_host_id?: string
}

const props = defineProps<{
  show: boolean
  initialData?: HostFormData | null
  existingHosts: HostFormData[]
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', data: HostFormData): void
}>()

const { t } = useI18n()

const form = reactive<HostFormData>({
  id: props.initialData?.id || `host_${Date.now()}`,
  label: props.initialData?.label || '',
  host: props.initialData?.host || '',
  port: props.initialData?.port || 22,
  username: props.initialData?.username || 'root',
  auth_type: props.initialData?.auth_type || 'Password',
  password: props.initialData?.password || '',
  private_key: props.initialData?.private_key || '',
  passphrase: props.initialData?.passphrase || '',
  jump_host_id: props.initialData?.jump_host_id || undefined
})

const errorMessage = ref('')

function handleSave() {
  errorMessage.value = ''
  if (!form.host.trim()) {
    errorMessage.value = t('connection.hostRequired')
    return
  }
  if (!form.username.trim()) {
    errorMessage.value = t('connection.userRequired')
    return
  }
  if (!form.label.trim()) {
    form.label = `${form.username}@${form.host}`
  }

  emit('save', { ...form })
}
</script>
