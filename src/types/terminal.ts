/**
 * 终端与分屏相关类型定义
 */

export type SplitDirection = 'horizontal' | 'vertical'

/**
 * 窗格信息模型
 */
export interface TerminalPaneItem {
  /** 窗格唯一 ID */
  id: string
  /** 关联的主机配置 ID（空表示未绑定或本地终端） */
  hostId?: string
  /** 窗格标题 */
  title: string
  /** 所占比例（0.1 - 0.9） */
  ratio: number
  /** 分割方向 */
  splitDirection?: SplitDirection
  /** 是否打开了随动 SFTP 抽屉 */
  isDrawerOpen?: boolean
}
