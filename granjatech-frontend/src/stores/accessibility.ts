import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'

const STORAGE_KEY = 'granjatech-accessibility-preferences'
const DEFAULT_MODE = 'light'
const DEFAULT_FONT_SCALE = 1
const FONT_SCALE_STEP = 0.1
const MIN_FONT_SCALE = 0.85
const MAX_FONT_SCALE = 1.3

function clamp(value: number): number {
  return Math.min(MAX_FONT_SCALE, Math.max(MIN_FONT_SCALE, Number.isNaN(value) ? DEFAULT_FONT_SCALE : value))
}

function readStored(): { mode: string; fontScale: number } | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return null
    const parsed = JSON.parse(raw)
    if (!parsed || typeof parsed !== 'object') return null
    return {
      mode: parsed.mode === 'dark' ? 'dark' : DEFAULT_MODE,
      fontScale: clamp(parseFloat(parsed.fontScale)),
    }
  } catch {
    return null
  }
}

export const useAccessibilityStore = defineStore('accessibility', () => {
  const stored = readStored()
  const mode = ref(stored?.mode || DEFAULT_MODE)
  const fontScale = ref(stored?.fontScale || DEFAULT_FONT_SCALE)

  const canIncreaseFont = computed(() => fontScale.value < MAX_FONT_SCALE - 1e-3)
  const canDecreaseFont = computed(() => fontScale.value > MIN_FONT_SCALE + 1e-3)

  function toggleColorMode() {
    mode.value = mode.value === 'light' ? 'dark' : 'light'
  }

  function increaseFontScale() {
    fontScale.value = clamp(fontScale.value + FONT_SCALE_STEP)
  }

  function decreaseFontScale() {
    fontScale.value = clamp(fontScale.value - FONT_SCALE_STEP)
  }

  function resetSettings() {
    mode.value = DEFAULT_MODE
    fontScale.value = DEFAULT_FONT_SCALE
    localStorage.removeItem(STORAGE_KEY)
  }

  // Persist to localStorage
  watch([mode, fontScale], ([m, fs]) => {
    if (m === DEFAULT_MODE && fs === DEFAULT_FONT_SCALE) {
      localStorage.removeItem(STORAGE_KEY)
    } else {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({ mode: m, fontScale: fs }))
    }
  })

  // Apply font scale to root element
  watch(fontScale, (scale) => {
    document.documentElement.style.fontSize = `${scale * 100}%`
  }, { immediate: true })

  return {
    mode,
    fontScale,
    canIncreaseFont,
    canDecreaseFont,
    toggleColorMode,
    increaseFontScale,
    decreaseFontScale,
    resetSettings,
  }
})
