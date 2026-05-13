import { get, writable } from 'svelte/store'
import { startAudioAdapter } from '../audio/adapters'
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
  captureMode: 'microphone-permission-needed',
  statusMessage: 'Ready to request microphone access.',
  errorMessage: null
}

const resolveCaptureMode = (source: AudioSource): AudioState['captureMode'] =>
  source === 'system' ? 'system-native-planned' : 'microphone-permission-needed'

const store = writable<AudioState>(initialState)

let stopEngine: (() => void) | null = null
let startToken = 0

const resetReactiveState = () => ({
  level: 0,
  smoothedLevel: 0,
  beat: 0,
  bands: {
    low: 0,
    mid: 0,
    high: 0
  },
  waveform: Array.from({ length: 64 }, () => 0),
  spectrum: Array.from({ length: 32 }, () => 0)
})

const stop = () => {
  startToken += 1
  stopEngine?.()
  stopEngine = null
  store.update((state) => ({
    ...state,
    ...resetReactiveState(),
    running: false,
    errorMessage: null,
    captureMode: resolveCaptureMode(state.source),
    statusMessage:
      state.source === 'system'
        ? 'Mac output capture still needs a native adapter.'
        : 'Ready to request microphone access.'
  }))
}

const start = async () => {
  const state = get(store)
  if (stopEngine) return
  const token = ++startToken

  store.update((current) => ({
    ...current,
    running: true,
    errorMessage: null,
    statusMessage:
      state.source === 'system'
        ? 'Starting mock system signal while native output capture is pending.'
        : 'Requesting microphone access...'
  }))

  try {
    const session = await startAudioAdapter(state.source, (next) => {
      store.update((current) => ({
        ...current,
        ...next
      }))
    })

    if (token !== startToken) {
      session.stop()
      return
    }

    stopEngine = session.stop
    store.update((current) => ({
      ...current,
      running: true,
      captureMode: session.captureMode,
      statusMessage: session.statusMessage,
      errorMessage: null
    }))
  } catch (error) {
    if (token !== startToken) return

    const message = error instanceof Error ? error.message : 'Unknown audio startup error.'

    store.update((current) => ({
      ...current,
      ...resetReactiveState(),
      running: false,
      captureMode:
        current.source === 'microphone' ? 'microphone-unavailable' : 'system-native-planned',
      statusMessage:
        current.source === 'microphone'
          ? 'Microphone capture could not start.'
          : 'Mac output capture still needs a native adapter.',
      errorMessage: message
    }))
  }
}

const restart = () => {
  stop()
  void start()
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

    void start()
  },
  setSource(source: AudioSource) {
    const running = get(store).running

    store.update((state) => ({
      ...state,
      source,
      captureMode: resolveCaptureMode(source),
      statusMessage:
        source === 'system'
          ? 'Mac output capture still needs a native adapter.'
          : 'Ready to request microphone access.',
      errorMessage: null
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
