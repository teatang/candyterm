<template>
  <div class="flex flex-col h-full w-full bg-slate-900 text-slate-200 select-none overflow-hidden">
    <!-- 顶部操作栏 -->
    <header class="flex items-center justify-between px-4 py-2.5 bg-slate-950 border-b border-slate-800">
      <div class="flex items-center space-x-2">
        <span class="text-lg">📁</span>
        <h2 class="font-bold text-sm text-slate-100">全功能 SFTP 双栏文件管理器</h2>
        <span class="text-xs text-slate-500">(支持本地/远程双向拖放与断点续传)</span>
      </div>

      <div class="flex items-center space-x-2">
        <button
          class="px-2.5 py-1 text-xs bg-blue-600 hover:bg-blue-500 text-white rounded font-medium transition-colors"
          @click="simulateUpload"
        >
          ⬆️ 上传测试
        </button>
        <button
          class="px-2.5 py-1 text-xs bg-slate-800 hover:bg-slate-700 text-slate-200 rounded font-medium transition-colors"
          @click="refreshAll"
        >
          🔄 刷新双端
        </button>
      </div>
    </header>

    <!-- 双栏浏览主体 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 左栏：本地磁盘 -->
      <section class="flex-1 flex flex-col border-r border-slate-800 min-w-0 bg-slate-950/40">
        <div class="flex items-center justify-between px-3 py-2 bg-slate-900/90 border-b border-slate-800 text-xs">
          <div class="flex items-center space-x-1.5 truncate">
            <span class="font-bold text-emerald-400">💻 本地</span>
            <span class="text-slate-600">|</span>
            <span class="font-mono text-slate-400 truncate">{{ sftpStore.currentLocalPath }}</span>
          </div>
          <button class="p-1 hover:text-slate-100 hover:bg-slate-800 rounded" @click="goUpLocal">⬆️</button>
        </div>

        <div class="flex-1 overflow-y-auto p-2 space-y-1">
          <div
            v-for="item in localFiles"
            :key="item.path"
            class="flex items-center justify-between px-3 py-1.5 rounded hover:bg-slate-800/80 cursor-pointer text-xs"
            @click="item.is_dir ? enterLocalDir(item.name) : null"
          >
            <div class="flex items-center space-x-2 truncate">
              <span>{{ item.is_dir ? '📁' : '📄' }}</span>
              <span class="truncate font-mono" :class="item.is_dir ? 'text-emerald-300 font-medium' : 'text-slate-300'">
                {{ item.name }}
              </span>
            </div>
            <span class="text-[10px] text-slate-500 font-mono">{{ item.is_dir ? '目录' : formatBytes(item.size) }}</span>
          </div>
        </div>
      </section>

      <!-- 右栏：远程服务器 -->
      <section class="flex-1 flex flex-col min-w-0 bg-slate-950/20">
        <div class="flex items-center justify-between px-3 py-2 bg-slate-900/90 border-b border-slate-800 text-xs">
          <div class="flex items-center space-x-1.5 truncate">
            <span class="font-bold text-blue-400">☁️ 远程</span>
            <span class="text-slate-600">|</span>
            <span class="font-mono text-slate-400 truncate">{{ sftpStore.currentRemotePath }}</span>
          </div>
          <button class="p-1 hover:text-slate-100 hover:bg-slate-800 rounded" @click="goUpRemote">⬆️</button>
        </div>

        <div class="flex-1 overflow-y-auto p-2 space-y-1">
          <div
            v-for="item in remoteFiles"
            :key="item.path"
            class="flex items-center justify-between px-3 py-1.5 rounded hover:bg-slate-800/80 cursor-pointer text-xs"
            @click="item.is_dir ? enterRemoteDir(item.name) : null"
          >
            <div class="flex items-center space-x-2 truncate">
              <span>{{ item.is_dir ? '📁' : '📄' }}</span>
              <span class="truncate font-mono" :class="item.is_dir ? 'text-blue-300 font-medium' : 'text-slate-300'">
                {{ item.name }}
              </span>
            </div>
            <span class="text-[10px] text-slate-500 font-mono">{{ item.is_dir ? '目录' : formatBytes(item.size) }}</span>
          </div>
        </div>
      </section>
    </div>

    <!-- 底部传输队列监控 -->
    <TransferQueue />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useSftpStore, joinSftpPath, getParentSftpPath, formatBytes } from '@/stores/sftp'
import type { SftpFileItem } from '@/types/sftp'
import TransferQueue from './TransferQueue.vue'

const sftpStore = useSftpStore()

const localFiles = ref<SftpFileItem[]>([
  { name: 'Documents', path: '/Users/tea/Documents', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'Downloads', path: '/Users/tea/Downloads', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'dist.tar.gz', path: '/Users/tea/dist.tar.gz', is_dir: false, size: 15420300, permissions: 0o644, modified_at: Date.now() }
])

const remoteFiles = ref<SftpFileItem[]>([
  { name: 'etc', path: '/etc', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'var', path: '/var', is_dir: true, size: 4096, permissions: 0o755, modified_at: Date.now() },
  { name: 'nginx.conf', path: '/etc/nginx.conf', is_dir: false, size: 3420, permissions: 0o644, modified_at: Date.now() }
])

function goUpLocal() {
  sftpStore.currentLocalPath = getParentSftpPath(sftpStore.currentLocalPath)
}

function enterLocalDir(name: string) {
  sftpStore.currentLocalPath = joinSftpPath(sftpStore.currentLocalPath, name)
}

function goUpRemote() {
  sftpStore.currentRemotePath = getParentSftpPath(sftpStore.currentRemotePath)
}

function enterRemoteDir(name: string) {
  sftpStore.currentRemotePath = joinSftpPath(sftpStore.currentRemotePath, name)
}

function simulateUpload() {
  const taskId = sftpStore.enqueueTransfer({
    name: 'dist.tar.gz',
    localPath: '/Users/tea/dist.tar.gz',
    remotePath: joinSftpPath(sftpStore.currentRemotePath, 'dist.tar.gz'),
    direction: 'upload',
    totalBytes: 15420300
  })

  // 模拟进度跳变
  setTimeout(() => {
    sftpStore.updateProgress({
      transfer_id: taskId,
      transferred_bytes: 8000000,
      total_bytes: 15420300,
      is_finished: false,
      error: null
    })
  }, 200)

  setTimeout(() => {
    sftpStore.updateProgress({
      transfer_id: taskId,
      transferred_bytes: 15420300,
      total_bytes: 15420300,
      is_finished: true,
      error: null
    })
  }, 500)
}

function refreshAll() {
  console.log('[SFTP] 刷新双端文件列表')
}
</script>
