<script lang="ts">
  import { Canvas } from '@threlte/core'
  import { onMount } from 'svelte'
  import ControlPanel from './lib/ui/ControlPanel.svelte'
  import VisualizerScene from './lib/scene/VisualizerScene.svelte'
  import { audioControls, audioState } from './lib/stores/audio'

  onMount(() => {
    audioControls.start()

    return () => {
      audioControls.stop()
    }
  })
</script>

<svelte:head>
  <title>Music Visualizer</title>
  <meta
    name="description"
    content="Standalone Tauri and Threlte music visualizer starter."
  />
</svelte:head>

<div class="shell">
  <header class="hero">
    <div>
      <p class="eyebrow">Tauri + Threlte starter</p>
      <h1>Particles first. Audio pipeline next.</h1>
      <p class="lede">
        The scene is wired to a mock beat engine now so the visual system can be shaped before
        microphone and macOS playback-output capture land.
      </p>
    </div>

    <div class="hero-stats">
      <div>
        <span>Source</span>
        <strong>{$audioState.source === 'microphone' ? 'Microphone' : 'Mac Output'}</strong>
      </div>
      <div>
        <span>Status</span>
        <strong>{$audioState.running ? 'Reactive' : 'Paused'}</strong>
      </div>
      <div>
        <span>Beat</span>
        <strong>{Math.round($audioState.beat * 100)}%</strong>
      </div>
    </div>
  </header>

  <main class="workspace">
    <section class="viewport">
      <div class="canvas-frame">
        <Canvas dpr={[1, 2]}>
          <VisualizerScene />
        </Canvas>
      </div>
    </section>

    <aside class="sidebar">
      <ControlPanel />
    </aside>
  </main>
</div>
