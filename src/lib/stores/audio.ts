import { get, writable } from 'svelte/store'
import { createMockAudioEngine } from '../audio/mock-engine'
import type { AudioSource, AudioState } from '../audio/types'

const initialState: AudioState = {
  source: 'microphone',
  running: false,
  level: 0.18,
  smoothedLevel: 0.18,
  beat: 0,
  bands: {
    low: 0.24,
    mid: 0.18,
    high: 0.16
  },
  sensitivity: 1,
  glow: 0.82,
  spread: 0.72,
  waveform: Array.from({ length: 64 }, () => 0),
  spectrum: Array.from({ length: 32 }, () => 0),
  captureMode: 'microphone-planned'
}

const resolveCaptureMode = (source: AudioSource): AudioState['captureMode'] =>
  source === 'system' ? 'system-native-planned' : 'microphone-planned'

const store = writable<AudioState>(initialState)

let stopEngine: (() => void) | null = null

const stop = () => {
  stopEngine?.()
  stopEngine = null
  store.update((state) => ({
    ...state,
    running: false,
    beat: 0
  }))
}

const start = () => {
  const state = get(store)
  if (stopEngine) return

  store.update((current) => ({
    ...current,
    running: true,
    captureMode: 'mock'
  }))

  stopEngine = createMockAudioEngine(state.source, (next) => {
    store.update((current) => ({
      ...current,
      ...next
    }))
  })
}

const restart = () => {
  stop()
  start()
}

export const audioState = {
  subscribe: store.subscribe
}

export const audioControls = {
  start,
  stop,
  toggle() {
    if (get(store).running) {
      stop()
      return
    }

    start()
  },
  setSource(source: AudioSource) {
    const running = get(store).running

    store.update((state) => ({
      ...state,
      source,
      captureMode: resolveCaptureMode(source)
    }))

    if (running) restart()
  },
  setSensitivity(value: number) {
    store.update((state) => ({
      ...state,
      sensitivity: value
    }))
  },
  setGlow(value: number) {
    store.update((state) => ({
      ...state,
      glow: value
    }))
  },
  setSpread(value: number) {
    store.update((state) => ({
      ...state,
      spread: value
    }))
  }
}
