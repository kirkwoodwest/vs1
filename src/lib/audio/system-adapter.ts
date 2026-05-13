import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { AudioState } from './types'

type SystemAudioFrame = Pick<
  AudioState,
  'level' | 'smoothedLevel' | 'beat' | 'bands' | 'waveform' | 'spectrum'
>

export const startSystemAdapter = async (
  update: (next: Partial<AudioState>) => void
) => {
  if (!isTauri()) {
    throw new Error('System output capture is only available in the Tauri desktop app.')
  }

  const unlisten = await listen<SystemAudioFrame>('system-audio-frame', (event) => {
    update(event.payload)
  })

  try {
    await invoke<string>('start_system_audio_capture')
  } catch (error) {
    unlisten()
    throw error
  }

  return async () => {
    unlisten()
    await invoke('stop_system_audio_capture')
  }
}
