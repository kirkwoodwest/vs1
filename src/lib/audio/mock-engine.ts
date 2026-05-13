import type { AudioSource, AudioState } from './types'

const WAVEFORM_SIZE = 64
const SPECTRUM_SIZE = 32

const clamp = (value: number, min = 0, max = 1) => Math.min(max, Math.max(min, value))

export const createMockAudioEngine = (
  source: AudioSource,
  update: (next: Partial<AudioState>) => void
) => {
  const sourceBias = source === 'system' ? 1.12 : 0.94
  const sourceOffset = source === 'system' ? 0.24 : 0.52

  let elapsed = 0
  let beatEnergy = 0
  let smooth = 0.22
  let disposed = false

  const tick = () => {
    if (disposed) return

    elapsed += 1 / 30

    if (Math.random() > 0.84 - sourceBias * 0.03) {
      beatEnergy = 1
    } else {
      beatEnergy *= 0.83
    }

    const pulse = beatEnergy * beatEnergy
    const low = clamp(0.35 + Math.sin(elapsed * 2.2 + sourceOffset) * 0.22 + pulse * 0.62)
    const mid = clamp(0.34 + Math.sin(elapsed * 1.55 + 0.8) * 0.18 + pulse * 0.16)
    const high = clamp(0.3 + Math.cos(elapsed * 3.6 + 1.1) * 0.24 + pulse * 0.1)

    const level = clamp((low * 0.5 + mid * 0.3 + high * 0.2) * sourceBias)
    smooth += (level - smooth) * 0.14

    const waveform = Array.from({ length: WAVEFORM_SIZE }, (_, index) => {
      const position = index / WAVEFORM_SIZE
      return (
        Math.sin(position * 12 + elapsed * 6.2) * (0.12 + smooth * 0.46) +
        Math.sin(position * 32 - elapsed * 3.4) * 0.06 +
        pulse * Math.sin(position * 10 + elapsed * 10) * 0.14
      )
    })

    const spectrum = Array.from({ length: SPECTRUM_SIZE }, (_, index) => {
      const ratio = index / SPECTRUM_SIZE
      const bandWeight = 1 - ratio * 0.7
      const shimmer = Math.sin(elapsed * (4 + ratio * 9) + ratio * 8) * 0.08
      return clamp(
        (low * bandWeight + mid * (1 - Math.abs(0.45 - ratio)) + high * ratio) * 0.52 + shimmer
      )
    })

    update({
      level,
      smoothedLevel: smooth,
      beat: pulse,
      bands: { low, mid, high },
      waveform,
      spectrum
    })
  }

  tick()
  const timer = window.setInterval(tick, 1000 / 30)

  return () => {
    disposed = true
    window.clearInterval(timer)
  }
}
