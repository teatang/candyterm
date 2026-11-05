<template>
  <div class="flex h-screen w-screen overflow-hidden bg-slate-900 text-slate-100">
    <!-- 左侧极简图标导航 -->
    <IconSidebar
      :active-view="activeView"
      @update:active-view="handleViewChange"
    />

    <!-- 主工作区内容容器 -->
    <main class="flex-1 flex flex-col h-full min-w-0 overflow-hidden relative">
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
import { ref } from 'vue'
import IconSidebar from '@/components/sidebar/IconSidebar.vue'
import SplitWorkspace from '@/components/terminal/SplitWorkspace.vue'

const activeView = ref<'sessions' | 'sftp' | 'settings' | 'workspace'>('workspace')

function handleViewChange(view: 'sessions' | 'sftp' | 'settings' | 'workspace') {
  activeView.value = view
}
</script>
