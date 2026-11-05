<template>
  <aside class="w-14 bg-slate-950 border-r border-slate-800 flex flex-col items-center py-3 select-none z-30">
    <!-- 顶部应用标识 -->
    <div
      class="w-9 h-9 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-white font-bold text-sm shadow-md shadow-blue-500/20 mb-3 cursor-pointer select-none"
      title="Candyterm"
      @click="emit('update:activeView', 'workspace')"
    >
      CT
    </div>

    <!-- 快捷添加 SSH 连接按钮 (高亮显眼) -->
    <button
      class="w-9 h-9 rounded-lg bg-blue-600 hover:bg-blue-500 active:scale-95 text-white flex items-center justify-center text-lg font-bold shadow-md shadow-blue-600/30 transition-all mb-4"
      title="＋ 新建 SSH 连接 (可视化配置窗口)"
      @click="emit('new-connection')"
    >
      ＋
    </button>

    <!-- 主导航按钮组 -->
    <nav class="flex-1 flex flex-col items-center space-y-3">
      <!-- 终端工作区视图 -->
      <button
        class="w-10 h-10 rounded-lg flex items-center justify-center text-lg transition-colors"
        :class="activeView === 'workspace' ? 'bg-blue-600/20 text-blue-400' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'"
        title="终端分屏工作区"
        @click="emit('update:activeView', 'workspace')"
      >
        🖥️
      </button>

      <!-- 会话管理器 (弹出服务器列表) -->
      <button
        class="w-10 h-10 rounded-lg flex items-center justify-center text-lg transition-colors"
        :class="activeView === 'sessions' ? 'bg-blue-600/20 text-blue-400' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'"
        title="服务器会话管理"
        @click="emit('open-sessions')"
      >
        💻
      </button>

      <!-- 全尺寸 SFTP 标签 -->
      <button
        class="w-10 h-10 rounded-lg flex items-center justify-center text-lg transition-colors"
        :class="activeView === 'sftp' ? 'bg-blue-600/20 text-blue-400' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'"
        title="独立 SFTP 双栏文件管理器"
        @click="emit('update:activeView', 'sftp')"
      >
        📁
      </button>

      <!-- 输入广播模式开关 -->
      <button
        class="w-10 h-10 rounded-lg flex items-center justify-center text-lg transition-colors"
        :class="workspaceStore.isBroadcastActive ? 'bg-amber-500/20 text-amber-400 ring-1 ring-amber-500/40' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'"
        title="输入广播 (将按键同步分发给所有窗格)"
        @click="workspaceStore.toggleBroadcast"
      >
        ⚡
      </button>
    </nav>

    <!-- 底部控制按钮组 (主题、语言、设置) -->
    <div class="flex flex-col items-center space-y-3 pt-4 border-t border-slate-800/80">
      <!-- 主题切换 -->
      <button
        class="w-9 h-9 rounded-lg flex items-center justify-center text-base text-slate-400 hover:text-slate-200 hover:bg-slate-900 transition-colors"
        :title="`切换主题: 当前为 ${themeStore.mode}`"
        @click="cycleTheme"
      >
        <span v-if="themeStore.mode === 'auto'">🌓</span>
        <span v-else-if="themeStore.mode === 'dark'">🌙</span>
        <span v-else>☀️</span>
      </button>

      <!-- 语言切换 -->
      <button
        class="w-9 h-9 rounded-lg flex items-center justify-center text-xs font-bold text-slate-400 hover:text-slate-200 hover:bg-slate-900 transition-colors"
        :title="`切换语言: 当前为 ${i18nStore.locale}`"
        @click="toggleLanguage"
      >
        {{ i18nStore.locale === 'zh-CN' ? '中' : 'EN' }}
      </button>

      <!-- 系统设置 -->
      <button
        class="w-9 h-9 rounded-lg flex items-center justify-center text-base text-slate-400 hover:text-slate-200 hover:bg-slate-900 transition-colors"
        :class="activeView === 'settings' ? 'text-blue-400' : ''"
        title="系统设置"
        @click="emit('update:activeView', 'settings')"
      >
        ⚙️
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useWorkspaceStore } from '@/stores/workspace'
import { useThemeStore } from '@/stores/theme'
import { useI18nStore } from '@/stores/i18n'

defineProps<{
  activeView: 'sessions' | 'sftp' | 'settings' | 'workspace'
}>()

const emit = defineEmits<{
  (e: 'update:activeView', view: 'sessions' | 'sftp' | 'settings' | 'workspace'): void
  (e: 'new-connection'): void
  (e: 'open-sessions'): void
}>()

const workspaceStore = useWorkspaceStore()
const themeStore = useThemeStore()
const i18nStore = useI18nStore()

function cycleTheme() {
  if (themeStore.mode === 'auto') {
    themeStore.setThemeMode('dark')
  } else if (themeStore.mode === 'dark') {
    themeStore.setThemeMode('light')
  } else {
    themeStore.setThemeMode('auto')
  }
}

function toggleLanguage() {
  if (i18nStore.locale === 'zh-CN') {
    i18nStore.setLocale('en-US')
  } else {
    i18nStore.setLocale('zh-CN')
  }
}
</script>
