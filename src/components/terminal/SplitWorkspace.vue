<template>
  <div class="relative flex-1 w-full h-full bg-slate-900 p-1 overflow-hidden">
    <!-- 广播模式激活指示条 -->
    <div
      v-if="workspaceStore.isBroadcastActive"
      class="absolute top-2 right-4 z-20 px-3 py-1 bg-amber-500/90 text-slate-950 font-bold text-xs rounded-full shadow-lg backdrop-blur flex items-center space-x-1 animate-pulse"
    >
      <span>⚡</span>
      <span>输入广播模式已开启 (所有窗格同步接收按键)</span>
    </div>

    <!-- 动态分屏网格 -->
    <div
      class="w-full h-full grid gap-1.5"
      :style="gridStyle"
    >
      <div
        v-for="pane in workspaceStore.panes"
        :key="pane.id"
        class="h-full w-full min-h-0 min-w-0"
      >
        <TerminalPane
          :pane="pane"
          :is-active="workspaceStore.activePaneId === pane.id"
          @focus="workspaceStore.focusPane(pane.id)"
        >
          <template #drawer="{ isOpen, close }">
            <slot name="pane-drawer" :pane="pane" :is-open="isOpen" :close="close"></slot>
          </template>
        </TerminalPane>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useWorkspaceStore } from '@/stores/workspace'
import TerminalPane from './TerminalPane.vue'

const workspaceStore = useWorkspaceStore()

// 根据当前窗格数量动态计算网格布局
const gridStyle = computed(() => {
  const count = workspaceStore.panes.length
  if (count <= 1) {
    return { gridTemplateColumns: '1fr', gridTemplateRows: '1fr' }
  }
  if (count === 2) {
    return { gridTemplateColumns: '1fr 1fr', gridTemplateRows: '1fr' }
  }
  if (count === 3) {
    return { gridTemplateColumns: '1fr 1fr', gridTemplateRows: '1fr 1fr' }
  }
  return { gridTemplateColumns: 'repeat(2, 1fr)', gridTemplateRows: 'repeat(2, 1fr)' }
})
</script>
