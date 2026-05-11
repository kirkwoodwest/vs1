<script lang="ts">
  import { T } from '@threlte/core'
  import { OrbitControls } from '@threlte/extras'
  import Particle from './Particle.svelte'

  const particleCount = 90
  const particles = Array.from({ length: particleCount }, (_, index) => index)
</script>

<T.Color
  attach="background"
  args={['#04070d']}
/>

<T.Fog
  color="#04070d"
  near={10}
  far={34}
/>

<T.PerspectiveCamera
  makeDefault
  position={[0, 2.8, 11]}
  fov={46}
>
  <OrbitControls
    autoRotate
    autoRotateSpeed={0.28}
    enablePan={false}
    enableZoom={false}
    minPolarAngle={0.9}
    maxPolarAngle={1.9}
  />
</T.PerspectiveCamera>

<T.AmbientLight intensity={0.24} />

<T.PointLight
  color="#78e9ff"
  intensity={42}
  position={[0, 5, 6]}
/>

<T.PointLight
  color="#ff9358"
  intensity={20}
  position={[0, -3, -4]}
/>

<T.Mesh
  position={[0, -2.8, 0]}
  rotation={[-Math.PI / 2, 0, 0]}
>
  <T.CircleGeometry args={[13, 64]} />
  <T.MeshStandardMaterial
    color="#08101c"
    emissive="#10233f"
    emissiveIntensity={1.4}
  />
</T.Mesh>

<T.Mesh position={[0, 0.6, 0]}>
  <T.TorusGeometry args={[3.9, 0.05, 8, 120]} />
  <T.MeshStandardMaterial
    color="#6ee6ff"
    emissive="#6ee6ff"
    emissiveIntensity={2.8}
    transparent
    opacity={0.28}
    toneMapped={false}
  />
</T.Mesh>

{#each particles as index (index)}
  <Particle
    {index}
    count={particleCount}
  />
{/each}
