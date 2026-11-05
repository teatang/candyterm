<template>
  <div
    v-if="show"
    class="fixed inset-0 z-40 bg-slate-950/70 backdrop-blur-sm flex items-center justify-center p-4 select-none transition-all"
  >
    <div class="w-full max-w-lg bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl shadow-2xl p-6 text-slate-800 dark:text-slate-100 transition-colors">
      <div class="flex items-center justify-between mb-4 pb-3 border-b border-slate-200 dark:border-slate-800">
        <h3 class="text-base font-bold flex items-center space-x-2 text-slate-900 dark:text-slate-100">
          <span>💻</span>
          <span>{{ t('connection.sessionListTitle') }}</span>
        </h3>
        <div class="flex items-center space-x-2">
          <button
            class="px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded text-xs font-medium"
            @click="emit('new-host')"
          >
            + {{ t('connection.new') }}
          </button>
          <button class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 text-sm" @click="emit('close')">✕</button>
        </div>
      </div>

      <!-- 主机列表 -->
      <div class="max-h-80 overflow-y-auto space-y-2">
        <div
          v-for="host in hosts"
          :key="host.id"
          class="flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-950/60 border border-slate-200 dark:border-slate-800/80 hover:border-blue-500/50 cursor-pointer group transition-all"
          @click="emit('connect', host)"
        >
          <div class="flex items-center space-x-3">
            <span class="text-xl">🖥️</span>
            <div>
              <div class="font-medium text-sm text-slate-800 dark:text-slate-200 group-hover:text-blue-600 dark:group-hover:text-blue-400">
                {{ host.label }}
              </div>
              <div class="text-xs text-slate-500 font-mono">
                {{ host.username }}@{{ host.host }}:{{ host.port }}
                <span v-if="host.jump_host_id" class="text-amber-600 dark:text-amber-400/80 text-[10px] ml-1">[{{ t('connection.jumpHost') }}]</span>
              </div>
            </div>
          </div>

          <div class="flex items-center space-x-2">
            <button
              class="px-3 py-1 bg-blue-600/20 text-blue-600 dark:text-blue-400 hover:bg-blue-600 hover:text-white rounded text-xs font-medium transition-colors"
              @click.stop="emit('connect', host)"
            >
              {{ t('connection.connect') }}
            </button>
            <button
              class="text-slate-400 hover:text-red-500 p-1"
              :title="t('connection.delete')"
              @click.stop="emit('delete', host.id)"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="hosts.length === 0" class="py-12 text-center text-slate-400 dark:text-slate-500 text-xs">
          {{ t('connection.noSavedHosts') }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * 已保存服务器会话列表管理组件
 * 支持快捷连接、新建会话触发以及主机记录移除
 */
import { useI18n } from 'vue-i18n'
import type { HostFormData } from './NewConnectionModal.vue'

defineProps<{
  show: boolean
  hosts: HostFormData[]
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'new-host'): void
  (e: 'connect', host: HostFormData): void
  (e: 'delete', hostId: string): void
}>()

const { t } = useI18n()
</script>
