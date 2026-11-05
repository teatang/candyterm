import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import MasterPasswordModal from '@/components/auth/MasterPasswordModal.vue'
import NewConnectionModal from '@/components/connection/NewConnectionModal.vue'

/**
 * 验证主密码弹窗表单校验与新建连接模态框数据装配
 */
describe('应用集成与模态框测试 (Integration & Modals Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    // 模拟 jsdom 缺失的 getContext 与 matchMedia，保证纯净测试输出
    HTMLCanvasElement.prototype.getContext = () => ({
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
    } as unknown as RenderingContext)

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
      }
    })

    const saveBtn = wrapper.find('button[data-test="save-host-btn"]')
    await saveBtn.trigger('click')

    expect(wrapper.text()).toContain('主机地址不得为空')
  })

  it('工作区工具栏的新建连接和会话列表按钮应正确触发事件', async () => {
    const { default: MainWorkspace } = await import('@/views/MainWorkspace.vue')
    const wrapper = mount(MainWorkspace, {
      props: {
        savedHosts: [
          { id: 'h1', label: 'Server-1', host: '192.168.1.1', port: 22 }
        ]
      },
      global: {
        stubs: {
          SplitWorkspace: true
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
})
