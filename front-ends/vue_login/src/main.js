import './assets/css/fonts.css'
import './assets/css/index.css'

import { createApp } from 'vue'
import { createI18n } from 'vue-i18n'
import App from './App.vue'
import router from './router'

import { z } from 'zod'
import { makeZodI18nMap } from 'zod-vue-i18n'

import fr from './assets/translations/fr.json'
import en from './assets/translations/en.json'

// Detect browser locale
function getBrowserLocale(options = { fallback: 'en' }) {
  const navigatorLocale = navigator.language

  if (!navigatorLocale) {
    return options.fallback;
  }

  const trimmedLocale = navigatorLocale.trim().split(/-|_/)[0];
  return trimmedLocale;
}

const browserLocale = getBrowserLocale();

const i18n = createI18n({
  legacy: false,
  locale: ['en', 'fr'].includes(browserLocale) ? browserLocale : 'en', // Set locale to browser's or fallback
  fallbackLocale: 'en',
  messages: {
    en: en,
    fr: fr
  },
});

z.setErrorMap(makeZodI18nMap(i18n))

const app = createApp(App)

app.use(router)
app.use(i18n)

app.mount('#app')
