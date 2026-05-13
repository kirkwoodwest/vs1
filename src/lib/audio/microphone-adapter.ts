import type { AudioState } from './types'

const WAVEFORM_SIZE = 64
const SPECTRUM_SIZE = 32
const FFT_SIZE = 512
const SMOOTHING = 0.42
const LEVEL_ATTACK = 0.52
const LEVEL_RELEASE = 0.18
const BASS_FOLLOW = 0.2
const BEAT_DECAY = 0.72

const clamp = (value: number, min = 0, max = 1) => Math.min(max, Math.max(min, value))

const average = (buffer: Uint8Array, from: number, to: number) => {
  let total = 0
  const upper = Math.max(from + 1, to)

  for (let index = from; index < upper; index += 1) {
    total += buffer[index] ?? 0
  }

  return total / ((upper - from) * 255)
}

const downsampleTimeDomain = (buffer: Uint8Array) => {
  const stride = Math.max(1, Math.floor(buffer.length / WAVEFORM_SIZE))

  return Array.from({ length: WAVEFORM_SIZE }, (_, index) => {
    const sample = buffer[index * stride] ?? 128
    return (sample - 128) / 128
  })
}

const downsampleSpectrum = (buffer: Uint8Array) => {
  const bucketSize = Math.max(1, Math.floor(buffer.length / SPECTRUM_SIZE))

  return Array.from({ length: SPECTRUM_SIZE }, (_, bucketIndex) => {
    let total = 0
    let count = 0

    const start = bucketIndex * bucketSize
    const end = Math.min(buffer.length, start + bucketSize)

    for (let index = start; index < end; index += 1) {
      total += buffer[index] ?? 0
      count += 1
    }

    return count > 0 ? total / (count * 255) : 0
  })
}

export const startMicrophoneAdapter = async (
  update: (next: Partial<AudioState>) => void
) => {
  const stream = await navigator.mediaDevices.getUserMedia({
    audio: {
      echoCancellation: false,
      noiseSuppression: false,
      autoGainControl: false
    },
    video: false
  })

  const audioContext = new window.AudioContext({
    latencyHint: 'interactive'
  })

  if (audioContext.state === 'suspended') {
    await audioContext.resume()
  }

  const source = audioContext.createMediaStreamSource(stream)
  const analyser = audioContext.createAnalyser()
  analyser.fftSize = FFT_SIZE
  analyser.smoothingTimeConstant = SMOOTHING
  source.connect(analyser)

  const timeDomain = new Uint8Array(analyser.fftSize)
  const frequencyDomain = new Uint8Array(analyser.frequencyBinCount)

  let frameId = 0
  let stopped = false
  let smoothedLevel = 0
  let lowAverage = 0
  let beat = 0

  const tick = () => {
    if (stopped) return

    analyser.getByteTimeDomainData(timeDomain)
    analyser.getByteFrequencyData(frequencyDomain)

    const low = average(frequencyDomain, 0, Math.floor(frequencyDomain.length * 0.08))
    const mid = average(
      frequencyDomain,
      Math.floor(frequencyDomain.length * 0.08),
      Math.floor(frequencyDomain.length * 0.32)
    )
    const high = average(
      frequencyDomain,
      Math.floor(frequencyDomain.length * 0.32),
      Math.floor(frequencyDomain.length * 0.78)
    )

    const level = clamp(low * 0.52 + mid * 0.3 + high * 0.18)
    const levelResponse = level > smoothedLevel ? LEVEL_ATTACK : LEVEL_RELEASE
    smoothedLevel += (level - smoothedLevel) * levelResponse

    lowAverage += (low - lowAverage) * BASS_FOLLOW
    const onset = clamp((low - lowAverage * 0.9) * 6.4)
    beat = Math.max(onset, beat * BEAT_DECAY)

    update({
      level,
      smoothedLevel,
      beat,
      bands: { low, mid, high },
      waveform: downsampleTimeDomain(timeDomain),
      spectrum: downsampleSpectrum(frequencyDomain)
    })

    frameId = window.requestAnimationFrame(tick)
  }

  tick()

  return () => {
    stopped = true
    window.cancelAnimationFrame(frameId)
    source.disconnect()
    analyser.disconnect()
    stream.getTracks().forEach((track) => track.stop())
    void audioContext.close()
  }
}
