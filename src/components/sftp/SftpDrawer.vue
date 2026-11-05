<template>
  <div
    v-if="isOpen"
    class="absolute right-0 top-0 bottom-0 w-72 bg-white/95 dark:bg-slate-900/95 backdrop-blur border-l border-slate-200 dark:border-slate-800 flex flex-col z-20 shadow-2xl transition-all duration-200 select-none text-slate-800 dark:text-slate-200"
  >
    <!-- 抽屉头部导航 -->
    <div class="flex items-center justify-between px-3 py-2 bg-slate-100/90 dark:bg-slate-950/80 border-b border-slate-200 dark:border-slate-800 text-xs text-slate-700 dark:text-slate-300">
      <div class="flex items-center space-x-1.5 truncate">
        <span class="text-blue-600 dark:text-blue-400 font-bold">📁 SFTP</span>
        <span class="text-slate-400 dark:text-slate-500">|</span>
        <span class="truncate font-mono text-slate-600 dark:text-slate-400" :title="currentPath">{{ currentPath }}</span>
      </div>
      <div class="flex items-center space-x-1">
        <button
          class="p-1 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-200 dark:hover:bg-slate-800 rounded text-slate-500 dark:text-slate-400 transition-colors"
          :title="t('sftp.goUp')"
          @click="goUp"
        >
          ⬆️
        </button>
        <button
          class="p-1 hover:text-red-600 dark:hover:text-red-400 hover:bg-slate-200 dark:hover:bg-slate-800 rounded text-slate-500 dark:text-slate-400 transition-colors"
          :title="t('sftp.closeDrawer')"
          @click="emit('close')"
        >
          ✕
        </button>
      </div>
    </div>

    <!-- 拖拽上传感应区域 -->
    <div
      class="flex-1 overflow-y-auto p-2 space-y-1 relative"
      :class="{ 'bg-blue-50 dark:bg-blue-900/20 ring-2 ring-blue-500/50': isDraggingOver }"
      @dragover.prevent="isDraggingOver = true"
      @dragleave.prevent="isDraggingOver = false"
      @drop.prevent="handleDrop"
    >
      <!-- 拖拽悬浮提示 -->
      <div
        v-if="isDraggingOver"
        class="absolute inset-0 z-30 bg-blue-100/90 dark:bg-blue-950/80 backdrop-blur-sm flex flex-col items-center justify-center text-blue-700 dark:text-blue-300 text-xs pointer-events-none"
      >
        <span class="text-2xl mb-1">📥</span>
        <span>{{ t('sftp.dropToUpload') }}</span>
      </div>

      <!-- 文件树列表 -->
      <div
        v-for="item in files"
        :key="item.path"
        class="flex items-center justify-between px-2.5 py-1.5 rounded text-xs hover:bg-slate-100 dark:hover:bg-slate-800/80 cursor-pointer group transition-colors"
        @click="item.is_dir ? enterDir(item.name) : selectFile(item)"
      >
        <div class="flex items-center space-x-2 truncate">
          <span>{{ item.is_dir ? '📁' : '📄' }}</span>
          <span
            class="truncate font-mono"
            :class="item.is_dir ? 'text-blue-600 dark:text-blue-300 font-medium' : 'text-slate-700 dark:text-slate-200'"
          >
            {{ item.name }}
          </span>
        </div>
        <span class="text-[10px] text-slate-400 dark:text-slate-500 font-mono group-hover:text-slate-600 dark:group-hover:text-slate-400">
          {{ item.is_dir ? t('sftp.directory') : formatBytes(item.size) }}
        </span>
      </div>

      <div v-if="files.length === 0" class="py-8 text-center text-slate-400 dark:text-slate-500 text-xs">
        {{ t('sftp.emptyOrLoading') }}
      </div>
    </div>

    <!-- 底部状态说明 -->
    <div class="px-3 py-2 bg-slate-50 dark:bg-slate-950 border-t border-slate-200 dark:border-slate-800 text-[11px] text-slate-500 flex justify-between items-center transition-colors">
      <span>{{ t('sftp.dragHint') }}</span>
      <span class="text-blue-600 dark:text-blue-400 font-mono">{{ files.length }} {{ t('sftp.items') }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { SftpFileItem } from '@/types/sftp'
import { joinSftpPath, getParentSftpPath, formatBytes, useSftpStore } from '@/stores/sftp'

defineProps<{
  isOpen: boolean
  paneId: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { t } = useI18n()
const sftpStore = useSftpStore()
const currentPath = ref('/var/www')
const isDraggingOver = ref(false)

// 初始默认演示/模拟文件列表
const files = ref<SftpFileItem[]>([
  { name: 'api', path: '/var/www/api', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'assets', path: '/var/www/assets', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'index.html', path: '/var/www/index.html', is_dir: false, size: 2450, permissions: 0o644, modified_at: Date.now() },
  { name: 'config.json', path: '/var/www/config.json', is_dir: false, size: 890, permissions: 0o600, modified_at: Date.now() }
])

function goUp() {
  currentPath.value = getParentSftpPath(currentPath.value)
}

function enterDir(name: string) {
  currentPath.value = joinSftpPath(currentPath.value, name)
}

function selectFile(item: SftpFileItem) {
  console.log('[SFTP Drawer] 选中文件:', item.name)
}

function handleDrop(event: DragEvent) {
  isDraggingOver.value = false
  if (!event.dataTransfer?.files) return

  const droppedFiles = Array.from(event.dataTransfer.files)
  for (const file of droppedFiles) {
    sftpStore.enqueueTransfer({
      name: file.name,
      localPath: (file as unknown as { path?: string }).path || file.name,
      remotePath: joinSftpPath(currentPath.value, file.name),
      direction: 'upload',
      totalBytes: file.size
    })
  }
}
</script>
