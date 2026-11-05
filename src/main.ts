import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { i18n } from './locales'
import './style.css'

/**
 * 挂载 Candyterm 桌面端应用主实例
 */
const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(i18n)

app.mount('#app')
