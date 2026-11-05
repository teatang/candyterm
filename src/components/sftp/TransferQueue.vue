<template>
  <div class="bg-white dark:bg-slate-950 border-t border-slate-200 dark:border-slate-800 flex flex-col text-xs select-none transition-colors">
    <!-- 队列折叠标题栏 -->
    <div
      class="flex items-center justify-between px-4 py-2 bg-slate-100 dark:bg-slate-900/80 cursor-pointer hover:bg-slate-200 dark:hover:bg-slate-900 text-slate-700 dark:text-slate-300 transition-colors"
      @click="isExpanded = !isExpanded"
    >
      <div class="flex items-center space-x-2">
        <span class="font-bold">{{ t('sftp.transfers') }}</span>
        <span class="px-2 py-0.5 bg-blue-100 dark:bg-blue-600/30 text-blue-700 dark:text-blue-400 rounded-full text-[10px]">
          {{ activeCount }} {{ t('sftp.transferring') }} / {{ sftpStore.transfers.length }} {{ t('sftp.total') }}
        </span>
      </div>

      <div class="flex items-center space-x-3 text-slate-500 dark:text-slate-400">
        <button
          v-if="sftpStore.transfers.length > 0"
          class="hover:text-blue-600 dark:hover:text-slate-200 transition-colors font-medium"
          :title="t('sftp.clearCompleted')"
          @click.stop="sftpStore.clearCompleted"
        >
          {{ t('sftp.clearCompleted') }}
        </button>
        <span>{{ isExpanded ? '▼' : '▲' }}</span>
      </div>
    </div>

    <!-- 展开的任务列表 -->
    <div v-show="isExpanded" class="max-h-48 overflow-y-auto p-2 space-y-1.5 bg-slate-50 dark:bg-slate-950 transition-colors">
      <div
        v-for="task in sftpStore.transfers"
        :key="task.id"
        class="flex items-center justify-between p-2 rounded bg-white dark:bg-slate-900/60 border border-slate-200 dark:border-slate-800/80 shadow-sm transition-colors"
      >
        <div class="flex-1 min-w-0 pr-4">
          <div class="flex items-center justify-between mb-1">
            <span class="font-mono truncate font-medium text-slate-800 dark:text-slate-200" :title="task.name">
              {{ task.direction === 'upload' ? '⬆️' : '⬇️' }} {{ task.name }}
            </span>
            <span
              class="text-[10px] font-bold uppercase"
              :class="{
                'text-blue-600 dark:text-blue-400': task.status === 'transferring',
                'text-emerald-600 dark:text-emerald-400': task.status === 'completed',
                'text-red-600 dark:text-red-400': task.status === 'failed',
                'text-slate-400 dark:text-slate-500': task.status === 'cancelled'
              }"
            >
              {{ task.status }}
            </span>
          </div>

          <!-- 进度条 -->
          <div class="w-full bg-slate-200 dark:bg-slate-800 rounded-full h-1.5 overflow-hidden">
            <div
              class="h-full transition-all duration-200"
              :class="task.status === 'failed' ? 'bg-red-500' : 'bg-blue-500'"
              :style="{ width: `${task.progress}%` }"
            ></div>
          </div>

          <div class="flex justify-between text-[10px] text-slate-500 dark:text-slate-400 mt-1">
            <span>{{ formatBytes(task.transferredBytes) }} / {{ formatBytes(task.totalBytes) }}</span>
            <span v-if="task.errorMsg" class="text-red-500 dark:text-red-400 truncate max-w-[200px]" :title="task.errorMsg">
              {{ task.errorMsg }}
            </span>
            <span v-else>{{ task.progress }}%</span>
          </div>
        </div>

        <!-- 取消操作 -->
        <button
          v-if="task.status === 'transferring'"
          class="text-slate-400 hover:text-red-500 px-2 py-1 transition-colors"
          :title="t('sftp.cancelTransfer')"
          @click="sftpStore.cancelTransfer(task.id)"
        >
          ✕
        </button>
      </div>

      <div v-if="sftpStore.transfers.length === 0" class="py-4 text-center text-slate-400 dark:text-slate-600 text-xs">
        {{ t('sftp.noTransfers') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSftpStore, formatBytes } from '@/stores/sftp'

const { t } = useI18n()
const sftpStore = useSftpStore()
const isExpanded = ref(true)

const activeCount = computed(() => {
  return sftpStore.transfers.filter(t => t.status === 'transferring').length
})
</script>
