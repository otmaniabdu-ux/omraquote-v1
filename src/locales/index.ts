import { createI18n } from 'vue-i18n'
import fr from './fr.json'
import ar from './ar.json'

const messages = { fr, ar }

const i18n = createI18n({
  legacy: false,
  locale: 'fr',
  fallbackLocale: 'fr',
  messages,
})

export default i18n