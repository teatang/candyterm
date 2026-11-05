import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSftpStore, joinSftpPath, getParentSftpPath } from '@/stores/sftp'

/**
 * 验证 SFTP 路径处理算法、传输队列状态机与 0 字节文件边界情况
 */
describe('SFTP 文件传输与状态管理测试 (SFTP Store Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('路径拼接与父级路径回退算法正确性', () => {
    expect(joinSftpPath('/var/www', 'html')).toBe('/var/www/html')
    expect(joinSftpPath('/var/www/', 'html')).toBe('/var/www/html')
    expect(joinSftpPath('/', 'home')).toBe('/home')

    expect(getParentSftpPath('/var/www/html')).toBe('/var/www')
    expect(getParentSftpPath('/var')).toBe('/')
    expect(getParentSftpPath('/')).toBe('/')
  })

  it('传输队列任务添加与状态流转', () => {
    const store = useSftpStore()
    expect(store.transfers.length).toBe(0)

    const taskId = store.enqueueTransfer({
      name: 'archive.tar.gz',
      localPath: '/local/archive.tar.gz',
      remotePath: '/remote/archive.tar.gz',
      direction: 'upload',
      totalBytes: 1048576
    })

    expect(store.transfers.length).toBe(1)
    const task = store.transfers[0]
    expect(task.id).toBe(taskId)
    expect(task.status).toBe('transferring')
    expect(task.progress).toBe(0)

    // 更新传输进度 (50%)
    store.updateProgress({
      transfer_id: taskId,
      transferred_bytes: 524288,
      total_bytes: 1048576,
      is_finished: false,
      error: null
    })

    expect(store.transfers[0].transferredBytes).toBe(524288)
    expect(store.transfers[0].progress).toBe(50)
    expect(store.transfers[0].status).toBe('transferring')

    // 传输完成 (100%)
    store.updateProgress({
      transfer_id: taskId,
      transferred_bytes: 1048576,
      total_bytes: 1048576,
      is_finished: true,
      error: null
    })

    expect(store.transfers[0].status).toBe('completed')
    expect(store.transfers[0].progress).toBe(100)
  })

  it('0 字节空文件传输直接即时标记完成 (Review Focus 5)', () => {
    const store = useSftpStore()
    const emptyTaskId = store.enqueueTransfer({
      name: 'empty.txt',
      localPath: '/tmp/empty.txt',
      remotePath: '/remote/empty.txt',
      direction: 'upload',
      totalBytes: 0
    })

    store.updateProgress({
      transfer_id: emptyTaskId,
      transferred_bytes: 0,
      total_bytes: 0,
      is_finished: true,
      error: null
    })

    const task = store.transfers.find(t => t.id === emptyTaskId)
    expect(task?.status).toBe('completed')
    expect(task?.progress).toBe(100)
  })

  it('传输出现异常错误时正确标记失败 (Review Focus 5)', () => {
    const store = useSftpStore()
    const errorTaskId = store.enqueueTransfer({
      name: 'broken.iso',
      localPath: '/tmp/broken.iso',
      remotePath: '/remote/broken.iso',
      direction: 'upload',
      totalBytes: 5000000
    })

    store.updateProgress({
      transfer_id: errorTaskId,
      transferred_bytes: 12000,
      total_bytes: 5000000,
      is_finished: true,
      error: 'SSH 连接被远程对端重置 (Connection Reset)'
    })

    const task = store.transfers.find(t => t.id === errorTaskId)
    expect(task?.status).toBe('failed')
    expect(task?.errorMsg).toBe('SSH 连接被远程对端重置 (Connection Reset)')
  })
})
