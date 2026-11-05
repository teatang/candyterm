<template>
  <div
    v-if="show"
    class="fixed inset-0 z-40 bg-slate-950/70 backdrop-blur-sm flex items-center justify-center p-4 select-none"
  >
    <div class="w-full max-w-lg bg-slate-900 border border-slate-800 rounded-xl shadow-2xl p-6 text-slate-100">
      <div class="flex items-center justify-between mb-4 pb-3 border-b border-slate-800">
        <h3 class="text-base font-bold flex items-center space-x-2">
          <span>💻</span>
          <span>服务器列表 (Sessions)</span>
        </h3>
        <div class="flex items-center space-x-2">
          <button
            class="px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded text-xs font-medium"
            @click="emit('new-host')"
          >
            + 新建连接
          </button>
          <button class="text-slate-400 hover:text-slate-200 text-sm" @click="emit('close')">✕</button>
        </div>
      </div>

      <!-- 主机列表 -->
      <div class="max-h-80 overflow-y-auto space-y-2">
        <div
          v-for="host in hosts"
          :key="host.id"
          class="flex items-center justify-between p-3 rounded-lg bg-slate-950/60 border border-slate-800/80 hover:border-blue-500/50 cursor-pointer group transition-all"
          @click="emit('connect', host)"
        >
          <div class="flex items-center space-x-3">
            <span class="text-xl">🖥️</span>
            <div>
              <div class="font-medium text-sm text-slate-200 group-hover:text-blue-400">
                {{ host.label }}
              </div>
              <div class="text-xs text-slate-500 font-mono">
                {{ host.username }}@{{ host.host }}:{{ host.port }}
                <span v-if="host.jump_host_id" class="text-amber-400/80 text-[10px] ml-1">[跳板机]</span>
              </div>
            </div>
          </div>

          <div class="flex items-center space-x-2">
            <button
              class="px-3 py-1 bg-blue-600/20 text-blue-400 hover:bg-blue-600 hover:text-white rounded text-xs font-medium transition-colors"
              @click.stop="emit('connect', host)"
            >
              连接
            </button>
            <button
              class="text-slate-500 hover:text-red-400 p-1"
              title="删除主机"
              @click.stop="emit('delete', host.id)"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="hosts.length === 0" class="py-12 text-center text-slate-500 text-xs">
          暂无保存的服务器连接，点击右上角 "+ 新建连接" 添加
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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
</script>
