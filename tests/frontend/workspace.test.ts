import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useWorkspaceStore, clampSplitRatio, throttleResize } from '@/stores/workspace'

/**
 * 验证分屏树结构操作、焦点管理、广播模式及尺寸节流保护
 */
describe('多窗格分屏与工作区测试 (Workspace Store Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('初始状态具备一个默认聚焦窗格', () => {
    const store = useWorkspaceStore()
    expect(store.panes.length).toBe(1)
    expect(store.activePaneId).toBe(store.panes[0].id)
  })

  it('支持将窗格水平（向右）或垂直（向下）切分', () => {
    const store = useWorkspaceStore()
    const firstId = store.panes[0].id

    // 向右切分 (horizontal split)
    const newPaneId = store.splitPane(firstId, 'horizontal', 'host_1')
    expect(store.panes.length).toBe(2)
    expect(store.activePaneId).toBe(newPaneId)
    expect(store.panes.find(p => p.id === newPaneId)?.hostId).toBe('host_1')

    // 向下切分 (vertical split)
    const thirdPaneId = store.splitPane(newPaneId, 'vertical', 'host_2')
    expect(store.panes.length).toBe(3)
    expect(store.activePaneId).toBe(thirdPaneId)
  })

  it('关闭窗格时自动转移焦点至相邻存活窗格', () => {
    const store = useWorkspaceStore()
    const p1 = store.panes[0].id
    const p2 = store.splitPane(p1, 'horizontal')

    expect(store.activePaneId).toBe(p2)
    store.closePane(p2)

    expect(store.panes.length).toBe(1)
    expect(store.activePaneId).toBe(p1)
  })

  it('关闭第一个窗格时应正确保留后续窗格并更新活跃焦点', () => {
    const store = useWorkspaceStore()
    const p1 = store.panes[0].id
    const p2 = store.splitPane(p1, 'horizontal')

    // 聚焦在 p1
    store.focusPane(p1)
    expect(store.activePaneId).toBe(p1)

    // 关闭 p1
    store.closePane(p1)

    expect(store.panes.length).toBe(1)
    expect(store.panes[0].id).toBe(p2)
    expect(store.activePaneId).toBe(p2)
  })

  it('最后一个窗格不可被关闭', () => {
    const store = useWorkspaceStore()
    const p1 = store.panes[0].id
    store.closePane(p1)
    expect(store.panes.length).toBe(1)
  })

  it('closeActivePane 应该正确关闭当前活跃窗格，但保留至少一个窗格', () => {
    const store = useWorkspaceStore()
    const p1 = store.panes[0].id
    const p2 = store.splitPane(p1, 'horizontal')
    expect(store.panes.length).toBe(2)
    expect(store.activePaneId).toBe(p2)

    // 关闭当前聚焦的 p2
    store.closeActivePane()
    expect(store.panes.length).toBe(1)
    expect(store.activePaneId).toBe(p1)

    // 当只剩 1 个窗格时调用，保持安全不删除
    store.closeActivePane()
    expect(store.panes.length).toBe(1)
  })

  it('支持切换输入广播状态并在活跃窗格间路由', () => {
    const store = useWorkspaceStore()
    expect(store.isBroadcastActive).toBe(false)

    store.toggleBroadcast()
    expect(store.isBroadcastActive).toBe(true)

    // 广播目标应包含当前所有窗格
    const p1 = store.panes[0].id
    const p2 = store.splitPane(p1, 'horizontal')
    const targets = store.getBroadcastTargetIds()
    expect(targets).toEqual([p1, p2])
  })

  it('分屏拖拽尺寸比例具备合理范围钳制 (Review Focus 4)', () => {
    // 比例必须在 0.1 到 0.9 之间，防止零宽度或遮挡
    expect(clampSplitRatio(0.02)).toBe(0.1)
    expect(clampSplitRatio(0.98)).toBe(0.9)
    expect(clampSplitRatio(0.5)).toBe(0.5)
  })

  it('分屏高频拖拽 Resize 具备节流保护防止频繁重绘 (Review Focus 4)', async () => {
    vi.useFakeTimers()
    const callback = vi.fn()
    const throttled = throttleResize(callback, 50)

    // 模拟高频连续触发 10 次
    for (let i = 0; i < 10; i++) {
      throttled(i * 10)
    }

    // 节流内只执行首次
    expect(callback).toHaveBeenCalledTimes(1)

    // 快进时间超过 50ms
    vi.advanceTimersByTime(60)
    throttled(100)
    expect(callback).toHaveBeenCalledTimes(2)

    vi.useRealTimers()
  })
})
