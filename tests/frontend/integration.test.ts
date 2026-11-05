import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { i18n } from '@/locales'
import MasterPasswordModal from '@/components/auth/MasterPasswordModal.vue'
import NewConnectionModal from '@/components/connection/NewConnectionModal.vue'
import MainWorkspace from '@/views/MainWorkspace.vue'

/**
 * 验证主密码弹窗表单校验与新建连接模态框数据装配
 */
describe('应用集成与模态框测试 (Integration & Modals Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    // 明确设置测试环境默认语言为 zh-CN
    i18n.global.locale.value = 'zh-CN'
    // 模拟 jsdom 缺失的 getContext 与 matchMedia，保证纯净测试输出
    HTMLCanvasElement.prototype.getContext = (() => ({
      fillRect: () => {},
      clearRect: () => {},
      getImageData: () => ({ data: [] }),
      putImageData: () => {},
      createImageData: () => [],
      setTransform: () => {},
      drawImage: () => {},
      save: () => {},
      fillText: () => {},
      restore: () => {},
      beginPath: () => {},
      moveTo: () => {},
      lineTo: () => {},
      closePath: () => {},
      stroke: () => {},
      translate: () => {},
      scale: () => {},
      rotate: () => {},
      arc: () => {},
      fill: () => {},
      measureText: () => ({ width: 0 }),
      transform: () => {},
      rect: () => {},
      clip: () => {}
    })) as unknown as typeof HTMLCanvasElement.prototype.getContext

    window.matchMedia = window.matchMedia || (() => ({
      matches: false,
      media: '',
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => false
    } as unknown as MediaQueryList))
  })

  it('首次初始化主密码时两次输入不一致应产生校验错误', async () => {
    const wrapper = mount(MasterPasswordModal, {
      props: {
        show: true,
        isInitialized: false
      },
      global: {
        plugins: [i18n]
      }
    })

    const pwdInput = wrapper.find('input[data-test="pwd"]')
    const confirmInput = wrapper.find('input[data-test="confirm-pwd"]')
    const submitBtn = wrapper.find('button[data-test="submit-btn"]')

    await pwdInput.setValue('Password123!')
    await confirmInput.setValue('DifferentPassword!')
    await submitBtn.trigger('click')

    expect(wrapper.text()).toContain('两次输入的密码不一致')
  })

  it('主密码正常提交时触发 submit 事件并传递密码', async () => {
    const wrapper = mount(MasterPasswordModal, {
      props: {
        show: true,
        isInitialized: true
      },
      global: {
        plugins: [i18n]
      }
    })

    const pwdInput = wrapper.find('input[data-test="pwd"]')
    const submitBtn = wrapper.find('button[data-test="submit-btn"]')

    await pwdInput.setValue('CorrectMasterPassword!')
    await submitBtn.trigger('click')

    expect(wrapper.emitted('submit')).toBeTruthy()
    expect(wrapper.emitted('submit')?.[0]).toEqual(['CorrectMasterPassword!'])
  })

  it('新建连接模态框表单空地址校验与默认配置', async () => {
    const wrapper = mount(NewConnectionModal, {
      props: {
        show: true,
        existingHosts: []
      },
      global: {
        plugins: [i18n]
      }
    })

    const saveBtn = wrapper.find('button[data-test="save-host-btn"]')
    await saveBtn.trigger('click')

    expect(wrapper.text()).toContain('主机地址不得为空')
  })

  it('工作区工具栏的新建连接和会话列表按钮应正确触发事件', async () => {
    const wrapper = mount(MainWorkspace, {
      props: {
        savedHosts: [
          { id: 'h1', label: 'Server-1', host: '192.168.1.1', port: 22 }
        ]
      },
      global: {
        plugins: [i18n],
        stubs: {
          SplitWorkspace: true,
          IconSidebar: true
        }
      }
    })

    // 查找新建连接按钮并触发点击
    const newConnBtn = wrapper.findAll('button').find(b => b.text().includes('新建连接'))
    expect(newConnBtn).toBeDefined()
    await newConnBtn?.trigger('click')
    expect(wrapper.emitted('new-connection')).toBeTruthy()

    // 查找会话列表按钮并触发点击
    const sessionListBtn = wrapper.findAll('button').find(b => b.text().includes('会话列表'))
    expect(sessionListBtn).toBeDefined()
    await sessionListBtn?.trigger('click')
    expect(wrapper.emitted('open-sessions')).toBeTruthy()
  })

  it('切换语言时工作区文本应即时响应', async () => {
    const wrapper = mount(MainWorkspace, {
      global: {
        plugins: [i18n],
        stubs: {
          SplitWorkspace: true,
          IconSidebar: true
        }
      }
    })

    // 中文状态
    i18n.global.locale.value = 'zh-CN'
    await wrapper.vm.$nextTick()
    expect(wrapper.text()).toContain('新建连接')
    expect(wrapper.text()).toContain('会话列表')

    // 切换至英文
    i18n.global.locale.value = 'en-US'
    await wrapper.vm.$nextTick()
    expect(wrapper.text()).toContain('New Connection')
    expect(wrapper.text()).toContain('Sessions')
  })

  it('外观模式切换时应当正确更新 DOM 根节点 dark 类', async () => {
    const { useThemeStore } = await import('@/stores/theme')
    const themeStore = useThemeStore()

    themeStore.setThemeMode('dark')
    expect(document.documentElement.classList.contains('dark')).toBe(true)

    themeStore.setThemeMode('light')
    expect(document.documentElement.classList.contains('dark')).toBe(false)
  })

  it('分屏后点击窗格关闭按钮应成功移除该分屏', async () => {
    const { default: SplitWorkspace } = await import('@/components/terminal/SplitWorkspace.vue')
    const { useWorkspaceStore } = await import('@/stores/workspace')
    const store = useWorkspaceStore()

    // 初始状态为 1 个窗格，切分出第 2 个窗格
    expect(store.panes.length).toBe(1)
    const newPaneId = store.splitPane(store.panes[0].id, 'horizontal')
    expect(store.panes.length).toBe(2)

    const wrapper = mount(SplitWorkspace, {
      global: {
        plugins: [i18n]
      }
    })

    // 查找包含 ✕ 的按钮
    const closeBtns = wrapper.findAll('button').filter(b => b.text() === '✕')
    expect(closeBtns.length).toBe(2)

    // 点击第二个窗格的关闭按钮
    await closeBtns[1].trigger('click')
    await wrapper.vm.$nextTick()

    // 验证 store 中窗格已被成功移除
    expect(store.panes.length).toBe(1)
    expect(store.panes.some(p => p.id === newPaneId)).toBe(false)
    // 窗格降为 1 时，窗格右上角的关闭按钮应隐藏
    expect(wrapper.findAll('button').filter(b => b.text() === '✕').length).toBe(0)
  })

  it('交互式动态分屏并关闭第一个窗格应平滑移除且不崩溃', async () => {
    const { default: SplitWorkspace } = await import('@/components/terminal/SplitWorkspace.vue')
    const { useWorkspaceStore } = await import('@/stores/workspace')
    const store = useWorkspaceStore()

    // 初始单窗格
    expect(store.panes.length).toBe(1)
    const initialPaneId = store.panes[0].id

    const wrapper = mount(SplitWorkspace, {
      global: {
        plugins: [i18n]
      }
    })

    // 初始状态下无关闭窗格按钮
    expect(wrapper.findAll('button').filter(b => b.text() === '✕').length).toBe(0)

    // 点击向右拆分按钮动态切分
    const splitBtn = wrapper.find('button[data-test="split-right-btn"]')
    expect(splitBtn.exists()).toBe(true)
    await splitBtn.trigger('click')
    await wrapper.vm.$nextTick()

    // 验证切分出第二个窗格，出现 2 个关闭按钮
    expect(store.panes.length).toBe(2)
    const closeBtns = wrapper.findAll('button').filter(b => b.text() === '✕')
    expect(closeBtns.length).toBe(2)

    // 点击第一个窗格的关闭按钮
    await closeBtns[0].trigger('click')
    await wrapper.vm.$nextTick()

    // 验证初始窗格被移除，只剩新增窗格，关闭按钮自动隐藏
    expect(store.panes.length).toBe(1)
    expect(store.panes[0].id).not.toBe(initialPaneId)
    expect(wrapper.findAll('button').filter(b => b.text() === '✕').length).toBe(0)
  })

  it('主工作区工具栏在多窗格时显示关闭分屏按钮，点击后关闭当前活跃分屏', async () => {
    const { useWorkspaceStore } = await import('@/stores/workspace')
    const store = useWorkspaceStore()

    const wrapper = mount(MainWorkspace, {
      global: {
        plugins: [i18n],
        stubs: {
          SplitWorkspace: true,
          IconSidebar: true
        }
      }
    })

    // 单窗格时，工具栏不应显示关闭分屏按钮
    expect(wrapper.find('[data-test="close-active-split-btn"]').exists()).toBe(false)

    // 切分出第 2 个分屏
    const newId = store.splitPane(store.panes[0].id, 'horizontal')
    expect(store.panes.length).toBe(2)
    expect(store.activePaneId).toBe(newId)
    await wrapper.vm.$nextTick()

    // 此时顶部工具栏应出现关闭分屏按钮
    const closeSplitBtn = wrapper.find('[data-test="close-active-split-btn"]')
    expect(closeSplitBtn.exists()).toBe(true)
    expect(closeSplitBtn.text()).toContain('关闭当前分屏')

    // 点击关闭分屏按钮
    await closeSplitBtn.trigger('click')
    await wrapper.vm.$nextTick()

    // 验证分屏已被成功移除，恢复为 1 个窗格，关闭按钮自动隐藏
    expect(store.panes.length).toBe(1)
    expect(store.panes.some(p => p.id === newId)).toBe(false)
    expect(wrapper.find('[data-test="close-active-split-btn"]').exists()).toBe(false)
  })

  it('键盘快捷键 Ctrl+W 应该关闭当前活跃分屏', async () => {
    const { useWorkspaceStore } = await import('@/stores/workspace')
    const store = useWorkspaceStore()

    const wrapper = mount(MainWorkspace, {
      global: {
        plugins: [i18n],
        stubs: {
          SplitWorkspace: true,
          IconSidebar: true
        }
      }
    })

    const newId = store.splitPane(store.panes[0].id, 'horizontal')
    expect(store.panes.length).toBe(2)
    await wrapper.vm.$nextTick()

    // 触发 Ctrl+W 键盘事件
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'w', ctrlKey: true }))
    await wrapper.vm.$nextTick()

    expect(store.panes.length).toBe(1)
    expect(store.panes.some(p => p.id === newId)).toBe(false)
  })

  it('垂直分屏时应计算出对应的两行网格布局 (gridTemplateRows: 1fr 1fr)', async () => {
    const { default: SplitWorkspace } = await import('@/components/terminal/SplitWorkspace.vue')
    const { useWorkspaceStore } = await import('@/stores/workspace')
    const store = useWorkspaceStore()

    store.splitPane(store.panes[0].id, 'vertical')
    expect(store.panes.length).toBe(2)

    const wrapper = mount(SplitWorkspace, {
      global: {
        plugins: [i18n]
      }
    })

    const grid = wrapper.find('.grid')
    expect(grid.attributes('style')).toContain('grid-template-rows: 1fr 1fr')
    expect(grid.attributes('style')).toContain('grid-template-columns: 1fr')
  })
})
