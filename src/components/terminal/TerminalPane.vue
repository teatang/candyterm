<template>
  <div
    class="relative flex flex-col h-full w-full bg-slate-950 border border-transparent transition-colors duration-150 rounded overflow-hidden"
    :class="{
      '!border-blue-500 shadow-md shadow-blue-500/10': isActive,
      'border-slate-800': !isActive
    }"
    @click="onFocus"
  >
    <!-- 窗格顶部工具栏 -->
    <div class="flex items-center justify-between px-3 py-1.5 bg-slate-900 border-b border-slate-800 text-xs text-slate-400 select-none">
      <div class="flex items-center space-x-2 truncate">
        <span class="inline-block w-2 h-2 rounded-full" :class="isActive ? 'bg-emerald-500' : 'bg-slate-600'"></span>
        <span class="font-mono font-medium text-slate-200 truncate">{{ pane.title }}</span>
      </div>

      <div class="flex items-center space-x-1">
        <!-- SFTP 随动抽屉按钮 -->
        <button
          class="p-1 hover:text-slate-100 hover:bg-slate-800 rounded transition-colors"
          :class="{ 'text-blue-400': pane.isDrawerOpen }"
          title="SFTP 文件面板"
          @click.stop="toggleDrawer"
        >
          📁
        </button>

        <!-- 向右拆分 -->
        <button
          class="p-1 hover:text-slate-100 hover:bg-slate-800 rounded transition-colors"
          title="向右拆分"
          @click.stop="splitRight"
        >
          ◫
        </button>

        <!-- 向下拆分 -->
        <button
          class="p-1 hover:text-slate-100 hover:bg-slate-800 rounded transition-colors"
          title="向下拆分"
          @click.stop="splitDown"
        >
          ⬓
        </button>

        <!-- 关闭窗格 -->
        <button
          class="p-1 hover:text-red-400 hover:bg-slate-800 rounded transition-colors"
          title="关闭窗格"
          @click.stop="closeThisPane"
        >
          ✕
        </button>
      </div>
    </div>

    <!-- 终端承载区域与抽屉容器 -->
    <div class="relative flex-1 flex w-full h-full overflow-hidden">
      <!-- xterm 挂载容器 -->
      <div ref="terminalContainer" class="flex-1 w-full h-full p-1 bg-slate-950"></div>

      <!-- 随动 SFTP 抽屉插槽/嵌入 -->
      <slot name="drawer" :is-open="pane.isDrawerOpen" :close="toggleDrawer"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebglAddon } from '@xterm/addon-webgl'
import type { TerminalPaneItem } from '@/types/terminal'
import { useWorkspaceStore } from '@/stores/workspace'
import '@xterm/xterm/css/xterm.css'

const props = defineProps<{
  pane: TerminalPaneItem
  isActive: boolean
}>()

const emit = defineEmits<{
  (e: 'focus'): void
}>()

const workspaceStore = useWorkspaceStore()
const terminalContainer = ref<HTMLDivElement | null>(null)

let term: Terminal | null = null
let fitAddon: FitAddon | null = null

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
    const webglAddon = new WebglAddon()
    webglAddon.onContextLoss(() => {
      webglAddon.dispose()
    })
    term.loadAddon(webglAddon)
  } catch (err) {
    console.warn('[Candyterm] WebGL 加速未启用，自动降级为标准 Canvas 渲染模式', err)
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

  // 监听窗口尺寸变化
  window.addEventListener('resize', handleResize)
})

function handleResize() {
  fitAddon?.fit()
}

watch(() => props.pane.isDrawerOpen, () => {
  setTimeout(() => fitAddon?.fit(), 100)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
  term?.dispose()
})
</script>
