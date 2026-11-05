<template>
  <n-config-provider :theme="themeStore.isDark ? darkTheme : null">
    <n-message-provider>
      <div class="h-screen w-screen overflow-hidden bg-slate-900 font-sans text-slate-100 select-none">
        <!-- 主工作区 -->
        <MainWorkspace
          :saved-hosts="savedHosts"
          @new-connection="showNewConnectionModal = true"
          @open-sessions="showSessionListModal = true"
        >
          <!-- 随动 SFTP 抽屉 -->
          <template #pane-drawer="{ pane, close }">
            <SftpDrawer :is-open="pane.isDrawerOpen ?? false" :pane-id="pane.id" @close="close" />
          </template>

          <!-- 全屏独立 SFTP 视图 -->
          <template #full-sftp>
            <SftpFullView />
          </template>

          <!-- 设置视图 -->
          <template #settings-view>
            <div class="max-w-2xl mx-auto space-y-6">
              <h2 class="text-xl font-bold text-slate-100 flex items-center space-x-2">
                <span>⚙️</span>
                <span>Candyterm 系统设置</span>
              </h2>

              <div class="bg-slate-950 p-5 rounded-xl border border-slate-800 space-y-4">
                <h3 class="text-sm font-bold text-slate-300">外观与主题</h3>
                <div class="flex items-center justify-between">
                  <span class="text-xs text-slate-400">色彩模式</span>
                  <div class="flex space-x-2">
                    <button
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border"
                      :class="themeStore.mode === 'auto' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-900 border-slate-700 text-slate-400'"
                      @click="themeStore.setThemeMode('auto')"
                    >
                      跟随系统
                    </button>
                    <button
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border"
                      :class="themeStore.mode === 'dark' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-900 border-slate-700 text-slate-400'"
                      @click="themeStore.setThemeMode('dark')"
                    >
                      暗黑模式
                    </button>
                    <button
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border"
                      :class="themeStore.mode === 'light' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-900 border-slate-700 text-slate-400'"
                      @click="themeStore.setThemeMode('light')"
                    >
                      明亮模式
                    </button>
                  </div>
                </div>

                <div class="flex items-center justify-between pt-3 border-t border-slate-800">
                  <span class="text-xs text-slate-400">显示语言</span>
                  <div class="flex space-x-2">
                    <button
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border"
                      :class="i18nStore.locale === 'zh-CN' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-900 border-slate-700 text-slate-400'"
                      @click="i18nStore.setLocale('zh-CN')"
                    >
                      简体中文
                    </button>
                    <button
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border"
                      :class="i18nStore.locale === 'en-US' ? 'bg-blue-600/20 border-blue-500 text-blue-400' : 'bg-slate-900 border-slate-700 text-slate-400'"
                      @click="i18nStore.setLocale('en-US')"
                    >
                      English
                    </button>
                  </div>
                </div>
              </div>

              <div class="bg-slate-950 p-5 rounded-xl border border-slate-800 space-y-3">
                <h3 class="text-sm font-bold text-slate-300">安全与主密码</h3>
                <p class="text-xs text-slate-400">所有服务器连接与密码均使用 Argon2id + AES-256-GCM 硬件加密安全保存在本地。</p>
                <button
                  class="px-4 py-2 bg-red-600/20 text-red-400 hover:bg-red-600 hover:text-white rounded-lg text-xs font-medium transition-colors"
                  @click="handleLock"
                >
                  🔒 立即锁定保险箱
                </button>
              </div>
            </div>
          </template>
        </MainWorkspace>

        <!-- 主密码解锁/初始化模态框 -->
        <MasterPasswordModal
          :show="showMasterPasswordModal"
          :is-initialized="isVaultInitialized"
          @submit="handleMasterPasswordSubmit"
        />

        <!-- 新建连接模态框 -->
        <NewConnectionModal
          :show="showNewConnectionModal"
          :existing-hosts="savedHosts"
          @close="showNewConnectionModal = false"
          @save="handleSaveHost"
        />

        <!-- 会话管理器模态框 -->
        <SessionListModal
          :show="showSessionListModal"
          :hosts="savedHosts"
          @close="showSessionListModal = false"
          @new-host="showSessionListModal = false; showNewConnectionModal = true"
          @connect="handleConnectHost"
          @delete="handleDeleteHost"
        />
      </div>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, darkTheme } from 'naive-ui'
import { useThemeStore } from '@/stores/theme'
import { useI18nStore } from '@/stores/i18n'
import { useWorkspaceStore } from '@/stores/workspace'
import MainWorkspace from '@/views/MainWorkspace.vue'
import SftpDrawer from '@/components/sftp/SftpDrawer.vue'
import SftpFullView from '@/components/sftp/SftpFullView.vue'
import MasterPasswordModal from '@/components/auth/MasterPasswordModal.vue'
import NewConnectionModal, { type HostFormData } from '@/components/connection/NewConnectionModal.vue'
import SessionListModal from '@/components/connection/SessionListModal.vue'

const themeStore = useThemeStore()
const i18nStore = useI18nStore()
const workspaceStore = useWorkspaceStore()

const showMasterPasswordModal = ref(false)
const isVaultInitialized = ref(true)

const showNewConnectionModal = ref(false)
const showSessionListModal = ref(false)

const savedHosts = ref<HostFormData[]>([
  {
    id: 'demo_host_1',
    label: 'Prod-Kubernetes-Master',
    host: '10.0.8.1',
    port: 22,
    username: 'admin',
    auth_type: 'Password'
  },
  {
    id: 'demo_host_2',
    label: 'Dev-Web-Cluster',
    host: '192.168.1.120',
    port: 2222,
    username: 'web',
    auth_type: 'PrivateKey'
  }
])

onMounted(async () => {
  // 检查是否运行在 Tauri 环境
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import('@tauri-apps/api/core')
      const initialized = await invoke<boolean>('is_vault_initialized')
      isVaultInitialized.value = initialized
      const unlocked = await invoke<boolean>('is_vault_unlocked')
      if (!unlocked) {
        showMasterPasswordModal.value = true
      }
    }
  } catch (e) {
    console.log('[Candyterm Web Mode]', e)
  }
})

async function handleMasterPasswordSubmit(password: string) {
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import('@tauri-apps/api/core')
      if (!isVaultInitialized.value) {
        await invoke('init_vault', { password })
      } else {
        await invoke('unlock_vault', { password })
      }
    }
    showMasterPasswordModal.value = false
  } catch (err) {
    console.error('解锁失败:', err)
  }
}

function handleLock() {
  showMasterPasswordModal.value = true
}

async function handleSaveHost(host: HostFormData) {
  savedHosts.value.push(host)
  if (window.__TAURI_INTERNALS__) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('save_host', { host })
    } catch (e) {
      console.error('保存主机到后端保险箱失败:', e)
    }
  }
  showNewConnectionModal.value = false
  workspaceStore.splitPane(
    workspaceStore.activePaneId,
    'horizontal',
    host.id,
    host.label || `${host.username}@${host.host}`
  )
}

function handleConnectHost(host: HostFormData) {
  workspaceStore.splitPane(
    workspaceStore.activePaneId,
    'horizontal',
    host.id,
    host.label || `${host.username}@${host.host}`
  )
  showSessionListModal.value = false
}

function handleDeleteHost(hostId: string) {
  savedHosts.value = savedHosts.value.filter(h => h.id !== hostId)
}
</script>
