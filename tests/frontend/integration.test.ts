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
})
