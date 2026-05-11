<script lang="ts">
  import { T, useTask } from '@threlte/core'
  import { get } from 'svelte/store'
  import * as THREE from 'three'
  import { audioState } from '../stores/audio'

  interface Props {
    index: number
    count: number
  }

  let { index, count }: Props = $props()

  const hash = (value: number) => {
    const x = Math.sin(value * 127.1) * 43758.5453123
    return x - Math.floor(x)
  }

  const seed = $derived.by(() => ({
    orbit: (index / count) * Math.PI * 2,
    depthPhase: hash(index + 12.4) * Math.PI * 2,
    driftPhase: hash(index + 4.1) * Math.PI * 2,
    sizeBias: 0.58 + hash(index + 1.2) * 1.4,
    radius: 1.9 + hash(index + 5.4) * 5.6,
    baseHeight: -1.4 + hash(index + 9.7) * 3.6,
    speed: 0.3 + hash(index + 2.8) * 0.9
  }))

  let mesh = $state.raw<THREE.Mesh>()
  let material = $state.raw<THREE.MeshStandardMaterial>()
  let elapsed = 0

  useTask((delta) => {
    if (!mesh || !material) return

    elapsed += delta

    const audio = get(audioState)
    const beat = audio.beat * audio.sensitivity
    const bandMix = audio.bands.low * 0.45 + audio.bands.mid * 0.35 + audio.bands.high * 0.2
    const orbitMotion = elapsed * seed.speed + seed.driftPhase
    const spread = 1 + audio.spread * 0.55 + beat * 0.35
    const swirl = seed.orbit + elapsed * 0.08 + beat * 0.2
    const localRadius = seed.radius * spread + Math.sin(orbitMotion * 1.6) * (0.18 + beat * 0.8)

    mesh.position.x = Math.cos(swirl) * localRadius + Math.cos(orbitMotion) * 0.65
    mesh.position.z =
      Math.sin(swirl) * localRadius + Math.sin(orbitMotion * 1.2 + seed.depthPhase) * 0.75
    mesh.position.y =
      seed.baseHeight +
      Math.sin(orbitMotion + seed.depthPhase) * (0.25 + audio.bands.high * 0.65) +
      beat * 1.45

    const scale = (0.12 + bandMix * 0.3 + beat * 0.7) * seed.sizeBias
    mesh.scale.setScalar(scale)

    material.opacity = THREE.MathUtils.clamp(0.45 + bandMix * 0.25 + beat * 0.4, 0.35, 1)
    material.roughness = 0.22
    material.metalness = 0.04

    const hue = (0.52 + audio.bands.high * 0.14 + hash(index + 13.2) * 0.12) % 1
    material.color.setHSL(hue, 0.72, 0.64 + beat * 0.08)
    material.emissive.setHSL(0.54 + audio.bands.mid * 0.08, 0.9, 0.58)
    material.emissiveIntensity = 1.8 + audio.glow * 2.6 + beat * 7.4
  })
</script>

<T.Mesh bind:ref={mesh}>
  <T.SphereGeometry args={[0.2, 12, 12]} />
  <T.MeshStandardMaterial
    bind:ref={material}
    transparent
    toneMapped={false}
  />
</T.Mesh>
