<template>
  <div
    class="relative flex flex-col h-full w-full bg-white dark:bg-slate-950 border border-slate-200 dark:border-slate-800 transition-colors duration-150 rounded overflow-hidden"
    :class="{
      '!border-blue-500 shadow-md shadow-blue-500/10': isActive,
      'border-slate-200 dark:border-slate-800': !isActive
    }"
    @click="onFocus"
  >
    <!-- 窗格顶部工具栏 -->
    <div class="flex items-center justify-between px-3 py-1.5 bg-slate-50 dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 text-xs text-slate-600 dark:text-slate-400 select-none transition-colors">
      <div class="flex items-center space-x-2 truncate">
        <span class="inline-block w-2 h-2 rounded-full" :class="isActive ? 'bg-emerald-500' : 'bg-slate-400 dark:bg-slate-600'"></span>
        <span class="font-mono font-medium text-slate-700 dark:text-slate-200 truncate">{{ pane.title }}</span>
      </div>

      <div class="flex items-center space-x-1">
        <!-- SFTP 随动抽屉按钮 -->
        <button
          class="p-1 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-200 dark:hover:bg-slate-800 rounded transition-colors"
          :class="{ 'text-blue-500 dark:text-blue-400': pane.isDrawerOpen }"
          :title="t('terminal.openSftp')"
          @click.stop="toggleDrawer"
        >
          📁
        </button>

        <!-- 向右拆分 -->
        <button
          type="button"
          class="p-1 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-200 dark:hover:bg-slate-800 rounded transition-colors cursor-pointer"
          :title="t('terminal.splitRight')"
          data-test="split-right-btn"
          @mousedown.stop
          @click.stop.prevent="splitRight"
        >
          ◫
        </button>

        <!-- 向下拆分 -->
        <button
          type="button"
          class="p-1 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-200 dark:hover:bg-slate-800 rounded transition-colors cursor-pointer"
          :title="t('terminal.splitDown')"
          data-test="split-down-btn"
          @mousedown.stop
          @click.stop.prevent="splitDown"
        >
          ⬓
        </button>

        <!-- 关闭窗格（仅当存在多于1个分屏时允许关闭） -->
        <button
          v-if="workspaceStore.panes.length > 1"
          type="button"
          class="p-1 px-1.5 hover:text-white hover:bg-red-500 dark:hover:bg-red-600 text-slate-500 dark:text-slate-400 rounded transition-colors font-bold cursor-pointer"
          :title="t('terminal.close')"
          data-test="pane-close-btn"
          @mousedown.stop
          @click.stop.prevent="closeThisPane"
        >
          ✕
        </button>
      </div>
    </div>

    <!-- 终端承载区域与抽屉容器 -->
    <div class="relative flex-1 flex w-full h-full overflow-hidden">
      <!-- xterm 挂载容器 -->
      <div ref="terminalContainer" class="flex-1 w-full h-full p-1 bg-white dark:bg-slate-950 transition-colors"></div>

      <!-- 随动 SFTP 抽屉插槽/嵌入 -->
      <slot name="drawer" :is-open="pane.isDrawerOpen ?? false" :close="toggleDrawer"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebglAddon } from '@xterm/addon-webgl'
import type { TerminalPaneItem } from '@/types/terminal'
import { useWorkspaceStore } from '@/stores/workspace'
import { useThemeStore } from '@/stores/theme'
import '@xterm/xterm/css/xterm.css'

const props = defineProps<{
  pane: TerminalPaneItem
  isActive: boolean
}>()

const emit = defineEmits<{
  (e: 'focus'): void
}>()

const { t } = useI18n()
const workspaceStore = useWorkspaceStore()
const themeStore = useThemeStore()
const terminalContainer = ref<HTMLDivElement | null>(null)

let term: Terminal | null = null
let fitAddon: FitAddon | null = null
let webglAddon: WebglAddon | null = null
let resizeObserver: ResizeObserver | null = null
let isDisposed = false

function onFocus() {
  emit('focus')
  term?.focus()
}

function splitRight() {
  workspaceStore.splitPane(props.pane.id, 'horizontal', props.pane.hostId)
}

function splitDown() {
  workspaceStore.splitPane(props.pane.id, 'vertical', props.pane.hostId)
}

function closeThisPane() {
  console.log('[TerminalPane] 点击关闭当前窗格:', props.pane.id)
  workspaceStore.closePane(props.pane.id)
}

function toggleDrawer() {
  workspaceStore.toggleDrawer(props.pane.id)
}

onMounted(() => {
  if (!terminalContainer.value) return

  term = new Terminal({
    cursorBlink: true,
    fontSize: 13,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    theme: {
      background: '#020617', // slate-950
      foreground: '#f8fafc',
      cursor: '#38bdf8'
    }
  })

  fitAddon = new FitAddon()
  term.loadAddon(fitAddon)

  term.open(terminalContainer.value)

  // 尝试加载 WebGL 加速，失败时优雅降级为 Canvas/DOM
  try {
    webglAddon = new WebglAddon()
    webglAddon.onContextLoss(() => {
      try {
        webglAddon?.dispose()
      } catch {}
      webglAddon = null
    })
    term.loadAddon(webglAddon)
  } catch (err) {
    console.warn('[Candyterm] WebGL 加速未启用，自动降级为标准 Canvas 渲染模式', err)
    webglAddon = null
  }

  fitAddon.fit()

  // 发送初始终端输出
  term.writeln(`\x1b[38;5;39m[Candyterm]\x1b[0m 终端窗格已初始化: ${props.pane.title}`)
  term.writeln(`\x1b[90m输入命令或建立 SSH 远程连接...\x1b[0m\r\n`)

  // 监听按键输入并处理广播
  term.onData((data) => {
    if (workspaceStore.isBroadcastActive) {
      // 广播模式：派发至所有窗格
      console.log('[Broadcast Input]', data)
    }
    // 本地回显或发送至后端
    term?.write(data)
  })

  // 监听主题变化，动态调整 xterm 色彩
  watch(
    () => themeStore.isDark,
    (isDark) => {
      if (term) {
        term.options.theme = isDark
          ? {
              background: '#020617',
              foreground: '#f8fafc',
              cursor: '#38bdf8'
            }
          : {
              background: '#ffffff',
              foreground: '#0f172a',
              cursor: '#2563eb'
            }
      }
    },
    { immediate: true }
  )

  // 监听窗口尺寸变化
  window.addEventListener('resize', handleResize)

  // 监听容器 DOM 尺寸变化（分屏添加/关闭、抽屉展开折叠），自动触发 fit
  if (typeof ResizeObserver !== 'undefined' && terminalContainer.value) {
    resizeObserver = new ResizeObserver(() => {
      if (isDisposed || !term || !fitAddon) return
      try {
        fitAddon.fit()
      } catch {
        // 捕获测试环境或瞬态卸载异常
      }
    })
    resizeObserver.observe(terminalContainer.value)
  }
})

function handleResize() {
  if (isDisposed || !term || !fitAddon) return
  try {
    fitAddon.fit()
  } catch {}
}

watch(() => props.pane.isDrawerOpen, () => {
  setTimeout(() => {
    if (isDisposed || !term || !fitAddon) return
    try {
      fitAddon.fit()
    } catch {}
  }, 100)
})

onBeforeUnmount(() => {
  isDisposed = true

  // 1. 彻底断开 ResizeObserver，避免后续尺寸变化触发回调
  if (resizeObserver) {
    try {
      resizeObserver.disconnect()
    } catch (err) {
      console.warn('[Candyterm] 断开 ResizeObserver 异常:', err)
    }
    resizeObserver = null
  }

  // 2. 解绑窗口 Resize 监听
  try {
    window.removeEventListener('resize', handleResize)
  } catch (err) {
    console.warn('[Candyterm] 移除窗口监听异常:', err)
  }

  // 3. 显式销毁 WebglAddon，防止 WebKit 渲染上下文悬挂异常
  if (webglAddon) {
    try {
      webglAddon.dispose()
    } catch (err) {
      console.warn('[Candyterm] 释放 WebglAddon 异常:', err)
    }
    webglAddon = null
  }

  // 4. 显式释放 FitAddon
  if (fitAddon) {
    try {
      fitAddon.dispose()
    } catch (err) {
      console.warn('[Candyterm] 释放 FitAddon 异常:', err)
    }
    fitAddon = null
  }

  // 5. 销毁 xterm.js 实例
  if (term) {
    try {
      term.dispose()
    } catch (err) {
      console.warn('[Candyterm] 释放 Terminal 实例异常:', err)
    }
    term = null
  }
})
</script>
