/**
 * SFTP 数据模型定义
 */

export interface SftpFileItem {
  name: string
  path: string
  is_dir: boolean
  size: number
  permissions: number
  modified_at: number
}

export type TransferStatus = 'pending' | 'transferring' | 'completed' | 'failed' | 'cancelled'

export interface TransferTask {
  id: string
  name: string
  localPath: string
  remotePath: string
  direction: 'upload' | 'download'
  totalBytes: number
  transferredBytes: number
  progress: number
  status: TransferStatus
  errorMsg?: string
}

export interface RawTransferProgress {
  transfer_id: string
  transferred_bytes: number
  total_bytes: number
  is_finished: boolean
  error?: string | null
}
