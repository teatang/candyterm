<template>
  <div class="flex h-screen w-screen overflow-hidden bg-slate-100 dark:bg-slate-900 text-slate-800 dark:text-slate-100 transition-colors">
    <!-- 左侧极简图标导航 -->
    <IconSidebar
      :active-view="activeView"
      @update:active-view="handleViewChange"
      @new-connection="emit('new-connection')"
      @open-sessions="emit('open-sessions')"
    />

    <!-- 主工作区内容容器 -->
    <main class="flex-1 flex flex-col h-full min-w-0 overflow-hidden relative">
      <!-- 顶部工作区工具栏 (Top Action Bar) -->
      <header class="h-10 bg-white dark:bg-slate-950 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between px-3 select-none z-20 transition-colors">
        <div class="flex items-center space-x-2">
          <!-- 核心：添加 SSH 连接可视化窗口触发按钮 -->
          <button
            class="px-3 py-1 bg-blue-600 hover:bg-blue-500 active:scale-95 text-white rounded text-xs font-semibold flex items-center space-x-1 shadow-md shadow-blue-600/30 transition-all"
            :title="t('connection.new')"
            @click="emit('new-connection')"
          >
            <span>＋</span>
            <span>{{ t('connection.new') }}</span>
          </button>

          <!-- 会话管理按钮 -->
          <button
            class="px-2.5 py-1 bg-slate-100 dark:bg-slate-900 hover:bg-slate-200 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-800 rounded text-xs font-medium flex items-center space-x-1 transition-all"
            :title="t('nav.sessions')"
            @click="emit('open-sessions')"
          >
            <span>💻</span>
            <span>{{ t('nav.sessions') }}</span>
            <span v-if="savedHosts && savedHosts.length > 0" class="ml-1 px-1.5 py-0.2 bg-slate-200 dark:bg-slate-800 text-slate-600 dark:text-slate-400 text-[10px] rounded-full">
              {{ savedHosts.length }}
            </span>
          </button>

          <div class="h-4 w-px bg-slate-200 dark:bg-slate-800 mx-1"></div>

          <!-- 分屏快捷操作 -->
          <button
            class="p-1 px-2 text-xs text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-900 rounded flex items-center space-x-1 transition-colors"
            :title="t('terminal.splitHorizontal')"
            @click="splitHorizontal"
          >
            <span>◫</span>
            <span class="hidden sm:inline">{{ t('terminal.splitHorizontal') }}</span>
          </button>
          <button
            class="p-1 px-2 text-xs text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-900 rounded flex items-center space-x-1 transition-colors"
            :title="t('terminal.splitVertical')"
            @click="splitVertical"
          >
            <span>⬓</span>
            <span class="hidden sm:inline">{{ t('terminal.splitVertical') }}</span>
          </button>
        </div>

        <!-- 中部：当前活跃终端窗格指示 -->
        <div class="hidden md:flex items-center space-x-1.5 text-xs text-slate-600 dark:text-slate-400 bg-slate-100 dark:bg-slate-900 px-2.5 py-1 rounded border border-slate-200 dark:border-slate-800 transition-colors">
          <span class="inline-block w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
          <span class="font-medium text-slate-700 dark:text-slate-300">{{ t('terminal.activeTerminal') }}:</span>
          <span class="font-mono text-blue-600 dark:text-blue-400 font-semibold">{{ activePaneTitle }}</span>
        </div>

        <!-- 右侧：随动 SFTP 与广播状态控制 -->
        <div class="flex items-center space-x-2">
          <!-- 快速呼出随动 SFTP 抽屉 -->
          <button
            class="px-2 py-1 text-xs text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-800 rounded flex items-center space-x-1 transition-colors"
            :title="t('terminal.openSftp')"
            @click="toggleActiveDrawer"
          >
            <span>📁</span>
            <span class="hidden sm:inline">SFTP</span>
          </button>

          <!-- 输入广播指示按钮 -->
          <button
            class="px-2 py-1 text-xs rounded flex items-center space-x-1 transition-colors"
            :class="workspaceStore.isBroadcastActive ? 'bg-amber-500/20 text-amber-600 dark:text-amber-400 border border-amber-500/40' : 'text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-800 dark:hover:text-slate-200'"
            :title="t('nav.broadcast')"
            @click="workspaceStore.toggleBroadcast"
          >
            <span>⚡</span>
            <span class="hidden sm:inline">{{ workspaceStore.isBroadcastActive ? t('terminal.broadcastEnabled') : t('nav.broadcast') }}</span>
          </button>
        </div>
      </header>

      <!-- 终端分屏工作区 -->
      <SplitWorkspace v-show="activeView === 'workspace' || activeView === 'sessions'">
        <template #pane-drawer="{ pane, isOpen, close }">
          <slot name="pane-drawer" :pane="pane" :is-open="isOpen" :close="close"></slot>
        </template>
      </SplitWorkspace>

      <!-- 独立全功能 SFTP 标签页视图 -->
      <div v-if="activeView === 'sftp'" class="flex-1 w-full h-full">
        <slot name="full-sftp"></slot>
      </div>

      <!-- 系统设置视图 -->
      <div v-if="activeView === 'settings'" class="flex-1 w-full h-full p-6 overflow-y-auto">
        <slot name="settings-view"></slot>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import IconSidebar from '@/components/sidebar/IconSidebar.vue'
import SplitWorkspace from '@/components/terminal/SplitWorkspace.vue'
import { useWorkspaceStore } from '@/stores/workspace'

defineProps<{
  savedHosts?: Array<{ id: string; label: string; host: string; port: number }>
}>()

const emit = defineEmits<{
  (e: 'new-connection'): void
  (e: 'open-sessions'): void
}>()

const { t } = useI18n()
const workspaceStore = useWorkspaceStore()
const activeView = ref<'sessions' | 'sftp' | 'settings' | 'workspace'>('workspace')

function handleViewChange(view: 'sessions' | 'sftp' | 'settings' | 'workspace') {
  activeView.value = view
}

const activePaneTitle = computed(() => {
  const active = workspaceStore.panes.find(p => p.id === workspaceStore.activePaneId)
  return active ? active.title : 'Local Terminal'
})

function splitHorizontal() {
  workspaceStore.splitPane(workspaceStore.activePaneId, 'horizontal')
}

function splitVertical() {
  workspaceStore.splitPane(workspaceStore.activePaneId, 'vertical')
}

function toggleActiveDrawer() {
  workspaceStore.toggleDrawer(workspaceStore.activePaneId)
}
</script>
