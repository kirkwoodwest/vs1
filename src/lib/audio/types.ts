export type AudioSource = 'microphone' | 'system'

export interface FrequencyBands {
  low: number
  mid: number
  high: number
}

export interface AudioState {
  source: AudioSource
  running: boolean
  level: number
  smoothedLevel: number
  beat: number
  bands: FrequencyBands
  sensitivity: number
  glow: number
  spread: number
  waveform: number[]
  spectrum: number[]
  captureMode: 'mock' | 'microphone-planned' | 'system-native-planned'
}
