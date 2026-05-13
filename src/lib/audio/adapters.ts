import { startMicrophoneAdapter } from './microphone-adapter'
import { startSystemAdapter } from './system-adapter'
import type { AudioSource, AudioState } from './types'

export type AudioAdapterSession = {
  captureMode: AudioState['captureMode']
  statusMessage: string
  stop: () => void
}

export const startAudioAdapter = async (
  source: AudioSource,
  update: (next: Partial<AudioState>) => void
): Promise<AudioAdapterSession> => {
  if (source === 'system') {
    const stop = await startSystemAdapter(update)

    return {
      captureMode: 'system-live',
      statusMessage:
        'Live macOS playback-output capture active. Screen Recording permission is required.',
      stop: () => {
        void stop()
      }
    }
  }

  const stop = await startMicrophoneAdapter(update)

  return {
    captureMode: 'microphone-live',
    statusMessage: 'Live microphone capture active.',
    stop
  }
}
