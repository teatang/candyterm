import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { TerminalPaneItem, SplitDirection } from '@/types/terminal'

/**
 * 钳制分屏比例在 0.1 到 0.9 之间，防止零宽度或遮挡 (Review Focus 4)
 */
export function clampSplitRatio(ratio: number): number {
  if (ratio < 0.1) return 0.1
  if (ratio > 0.9) return 0.9
  return ratio
}

/**
 * 节流函数，避免高频拖拽 Resize 导致性能抖动 (Review Focus 4)
 */
export function throttleResize<T extends (...args: unknown[]) => void>(
  fn: T,
  delayMs: number
): (...args: Parameters<T>) => void {
  let lastCall = 0
  return (...args: Parameters<T>) => {
    const now = Date.now()
    if (now - lastCall >= delayMs) {
      lastCall = now
      fn(...args)
    }
  }
}

/**
 * 工作区分屏与终端窗格状态管理
 */
export const useWorkspaceStore = defineStore('workspace', () => {
  const panes = ref<TerminalPaneItem[]>([
    {
      id: 'pane_default_1',
      title: 'Local Terminal',
      ratio: 0.5,
      isDrawerOpen: false
    }
  ])

  const activePaneId = ref<string>('pane_default_1')
  const isBroadcastActive = ref<boolean>(false)

  // 聚焦到指定窗格
  function focusPane(paneId: string) {
    if (panes.value.some(p => p.id === paneId)) {
      activePaneId.value = paneId
    }
  }

  // 拆分窗格（向右水平拆分或向下垂直拆分）
  function splitPane(
    targetPaneId: string,
    direction: SplitDirection,
    hostId?: string
  ): string {
    const newId = `pane_${Date.now()}_${Math.floor(Math.random() * 1000)}`
    const targetIdx = panes.value.findIndex(p => p.id === targetPaneId)

    const newPane: TerminalPaneItem = {
      id: newId,
      hostId,
      title: hostId ? `Session-${hostId.slice(0, 6)}` : 'Terminal',
      ratio: 0.5,
      splitDirection: direction,
      isDrawerOpen: false
    }

    if (targetIdx !== -1) {
      panes.value.splice(targetIdx + 1, 0, newPane)
    } else {
      panes.value.push(newPane)
    }

    activePaneId.value = newId
    return newId
  }

  // 关闭指定窗格
  function closePane(paneId: string) {
    if (panes.value.length <= 1) {
      // 最后一个窗格不可关闭
      return
    }

    const idx = panes.value.findIndex(p => p.id === paneId)
    if (idx === -1) return

    panes.value.splice(idx, 1)

    // 如果关闭的是当前聚焦窗格，转移焦点至最近窗格
    if (activePaneId.value === paneId) {
      const nextIdx = Math.max(0, idx - 1)
      activePaneId.value = panes.value[nextIdx].id
    }
  }

  // 切换输入广播模式
  function toggleBroadcast() {
    isBroadcastActive.value = !isBroadcastActive.value
  }

  // 获取所有需要接收广播的窗格 ID
  function getBroadcastTargetIds(): string[] {
    return panes.value.map(p => p.id)
  }

  // 切换指定窗格的 SFTP 随动抽屉
  function toggleDrawer(paneId: string) {
    const pane = panes.value.find(p => p.id === paneId)
    if (pane) {
      pane.isDrawerOpen = !pane.isDrawerOpen
    }
  }

  return {
    panes,
    activePaneId,
    isBroadcastActive,
    focusPane,
    splitPane,
    closePane,
    toggleBroadcast,
    getBroadcastTargetIds,
    toggleDrawer
  }
})
