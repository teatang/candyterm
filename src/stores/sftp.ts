import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { SftpFileItem, TransferTask, RawTransferProgress } from '@/types/sftp'

/**
 * 规范化拼接路径
 */
export function joinSftpPath(base: string, part: string): string {
  const cleanBase = base.endsWith('/') ? base.slice(0, -1) : base
  const cleanPart = part.startsWith('/') ? part.slice(1) : part
  const res = `${cleanBase}/${cleanPart}`
  return res.startsWith('/') ? res : `/${res}`
}

/**
 * 计算父级目录路径
 */
export function getParentSftpPath(current: string): string {
  if (current === '/' || !current) return '/'
  const parts = current.split('/').filter(Boolean)
  if (parts.length <= 1) return '/'
  parts.pop()
  return `/${parts.join('/')}`
}

/**
 * 格式化文件大小为易读字符串 (KB, MB, GB)
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`
}

/**
 * SFTP 传输与文件状态管理器
 */
export const useSftpStore = defineStore('sftp', () => {
  const currentRemotePath = ref<string>('/root')
  const currentLocalPath = ref<string>('/Users')
  const remoteFiles = ref<SftpFileItem[]>([])
  const localFiles = ref<SftpFileItem[]>([])
  const transfers = ref<TransferTask[]>([])

  // 添加新的传输任务入队
  function enqueueTransfer(params: {
    name: string
    localPath: string
    remotePath: string
    direction: 'upload' | 'download'
    totalBytes: number
  }): string {
    const id = `trans_${Date.now()}_${Math.floor(Math.random() * 1000)}`
    const newTask: TransferTask = {
      id,
      name: params.name,
      localPath: params.localPath,
      remotePath: params.remotePath,
      direction: params.direction,
      totalBytes: params.totalBytes,
      transferredBytes: 0,
      progress: 0,
      status: 'transferring'
    }
    transfers.value.unshift(newTask)
    return id
  }

  // 根据底层事件更新传输进度
  function updateProgress(p: RawTransferProgress) {
    const task = transfers.value.find(t => t.id === p.transfer_id)
    if (!task) return

    task.transferredBytes = p.transferred_bytes
    task.totalBytes = p.total_bytes

    if (p.error) {
      task.status = 'failed'
      task.errorMsg = p.error
      return
    }

    if (p.total_bytes === 0) {
      // 0 字节文件特判 (Review Focus 5)
      task.progress = 100
      task.status = 'completed'
      return
    }

    const calcProgress = Math.min(
      100,
      Math.floor((p.transferred_bytes / p.total_bytes) * 100)
    )
    task.progress = calcProgress

    if (p.is_finished || p.transferred_bytes >= p.total_bytes) {
      task.progress = 100
      task.status = 'completed'
    }
  }

  // 取消任务
  function cancelTransfer(id: string) {
    const task = transfers.value.find(t => t.id === id)
    if (task && task.status === 'transferring') {
      task.status = 'cancelled'
    }
  }

  // 清除已完成任务
  function clearCompleted() {
    transfers.value = transfers.value.filter(
      t => t.status !== 'completed' && t.status !== 'cancelled'
    )
  }

  return {
    currentRemotePath,
    currentLocalPath,
    remoteFiles,
    localFiles,
    transfers,
    enqueueTransfer,
    updateProgress,
    cancelTransfer,
    clearCompleted
  }
})
