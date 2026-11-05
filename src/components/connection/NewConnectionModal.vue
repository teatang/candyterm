<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-4 select-none"
  >
    <div class="w-full max-w-lg bg-slate-900 border border-slate-800 rounded-xl shadow-2xl p-6 text-slate-100 animate-in fade-in zoom-in-95 duration-150">
      <div class="flex items-center justify-between mb-4 pb-3 border-b border-slate-800">
        <h3 class="text-base font-bold text-slate-100 flex items-center space-x-2">
          <span>💻</span>
          <span>{{ initialData ? '编辑主机连接' : '新建 SSH 连接' }}</span>
        </h3>
        <button class="text-slate-400 hover:text-slate-200" @click="emit('close')">✕</button>
      </div>

      <form @submit.prevent="handleSave" class="space-y-4">
        <!-- 会话名称与地址 -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">会话标签</label>
            <input
              v-model="form.label"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
              placeholder="如：生产环境-01"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">主机地址 / IP *</label>
            <input
              v-model="form.host"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
              placeholder="192.168.1.100 或 example.com"
            />
          </div>
        </div>

        <!-- 端口与用户名 -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">端口</label>
            <input
              v-model.number="form.port"
              type="number"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">用户名 *</label>
            <input
              v-model="form.username"
              type="text"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        <!-- 认证类型切换 -->
        <div>
          <label class="block text-xs font-medium text-slate-300 mb-1">认证方式</label>
          <div class="flex space-x-2">
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'Password' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-950 border-slate-700 text-slate-400'"
              @click="form.auth_type = 'Password'"
            >
              密码
            </button>
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'PrivateKey' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-950 border-slate-700 text-slate-400'"
              @click="form.auth_type = 'PrivateKey'"
            >
              私钥文件
            </button>
            <button
              type="button"
              class="flex-1 py-1.5 text-xs font-medium rounded-lg border transition-colors"
              :class="form.auth_type === 'SshAgent' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-950 border-slate-700 text-slate-400'"
              @click="form.auth_type = 'SshAgent'"
            >
              SSH 代理
            </button>
          </div>
        </div>

        <!-- 密码输入框 -->
        <div v-if="form.auth_type === 'Password'">
          <label class="block text-xs font-medium text-slate-300 mb-1">登录密码</label>
          <input
            v-model="form.password"
            type="password"
            class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
            placeholder="输入主机密码..."
          />
        </div>

        <!-- 私钥与口令输入框 -->
        <div v-if="form.auth_type === 'PrivateKey'" class="space-y-3">
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">私钥内容或路径 (~/.ssh/id_rsa)</label>
            <textarea
              v-model="form.private_key"
              rows="3"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-xs font-mono text-slate-100 focus:outline-none focus:border-blue-500"
              placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
            ></textarea>
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-300 mb-1">私钥密码口令 (Passphrase，可选)</label>
            <input
              v-model="form.passphrase"
              type="password"
              class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        <!-- 跳板机选择 (ProxyJump) -->
        <div>
          <label class="block text-xs font-medium text-slate-300 mb-1">前置跳板机 (Jump Host，可选)</label>
          <select
            v-model="form.jump_host_id"
            class="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-xs text-slate-100 focus:outline-none focus:border-blue-500"
          >
            <option :value="undefined">无跳板机 (直连)</option>
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
        <div v-if="errorMessage" class="text-xs text-red-400 bg-red-950/40 border border-red-900/50 rounded p-2">
          {{ errorMessage }}
        </div>

        <div class="flex justify-end space-x-2 pt-2 border-t border-slate-800">
          <button
            type="button"
            class="px-4 py-2 text-xs font-medium text-slate-300 hover:bg-slate-800 rounded-lg"
            @click="emit('close')"
          >
            取消
          </button>
          <button
            data-test="save-host-btn"
            type="button"
            class="px-5 py-2 text-xs font-medium bg-blue-600 hover:bg-blue-500 text-white rounded-lg shadow-md shadow-blue-600/20"
            @click="handleSave"
          >
            保存并连接
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

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
    errorMessage.value = '主机地址不得为空'
    return
  }
  if (!form.username.trim()) {
    errorMessage.value = '用户名不得为空'
    return
  }
  if (!form.label.trim()) {
    form.label = `${form.username}@${form.host}`
  }

  emit('save', { ...form })
}
</script>
