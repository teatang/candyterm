<template>
  <div class="bg-slate-950 border-t border-slate-800 flex flex-col text-xs select-none">
    <!-- 队列折叠标题栏 -->
    <div
      class="flex items-center justify-between px-4 py-2 bg-slate-900/80 cursor-pointer hover:bg-slate-900 text-slate-300"
      @click="isExpanded = !isExpanded"
    >
      <div class="flex items-center space-x-2">
        <span class="font-bold">传输队列 (Transfers)</span>
        <span class="px-1.5 py-0.2 bg-blue-600/30 text-blue-400 rounded-full text-[10px]">
          {{ activeCount }} 传输中 / {{ sftpStore.transfers.length }} 总计
        </span>
      </div>

      <div class="flex items-center space-x-3 text-slate-400">
        <button
          v-if="sftpStore.transfers.length > 0"
          class="hover:text-slate-200"
          title="清除已完成项"
          @click.stop="sftpStore.clearCompleted"
        >
          清空完成
        </button>
        <span>{{ isExpanded ? '▼' : '▲' }}</span>
      </div>
    </div>

    <!-- 展开的任务列表 -->
    <div v-show="isExpanded" class="max-h-48 overflow-y-auto p-2 space-y-1.5 bg-slate-950">
      <div
        v-for="task in sftpStore.transfers"
        :key="task.id"
        class="flex items-center justify-between p-2 rounded bg-slate-900/60 border border-slate-800/80"
      >
        <div class="flex-1 min-w-0 pr-4">
          <div class="flex items-center justify-between mb-1">
            <span class="font-mono truncate font-medium text-slate-200" :title="task.name">
              {{ task.direction === 'upload' ? '⬆️' : '⬇️' }} {{ task.name }}
            </span>
            <span
              class="text-[10px] font-bold uppercase"
              :class="{
                'text-blue-400': task.status === 'transferring',
                'text-emerald-400': task.status === 'completed',
                'text-red-400': task.status === 'failed',
                'text-slate-500': task.status === 'cancelled'
              }"
            >
              {{ task.status }}
            </span>
          </div>

          <!-- 进度条 -->
          <div class="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
            <div
              class="h-full transition-all duration-200"
              :class="task.status === 'failed' ? 'bg-red-500' : 'bg-blue-500'"
              :style="{ width: `${task.progress}%` }"
            ></div>
          </div>

          <div class="flex justify-between text-[10px] text-slate-500 mt-1">
            <span>{{ formatBytes(task.transferredBytes) }} / {{ formatBytes(task.totalBytes) }}</span>
            <span v-if="task.errorMsg" class="text-red-400 truncate max-w-[200px]" :title="task.errorMsg">
              {{ task.errorMsg }}
            </span>
            <span v-else>{{ task.progress }}%</span>
          </div>
        </div>

        <!-- 取消操作 -->
        <button
          v-if="task.status === 'transferring'"
          class="text-slate-500 hover:text-red-400 px-2 py-1"
          title="取消传输"
          @click="sftpStore.cancelTransfer(task.id)"
        >
          ✕
        </button>
      </div>

      <div v-if="sftpStore.transfers.length === 0" class="py-4 text-center text-slate-600 text-xs">
        当前无传输任务
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSftpStore, formatBytes } from '@/stores/sftp'

const sftpStore = useSftpStore()
const isExpanded = ref(true)

const activeCount = computed(() => {
  return sftpStore.transfers.filter(t => t.status === 'transferring').length
})
</script>
