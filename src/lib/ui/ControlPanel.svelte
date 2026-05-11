<script lang="ts">
  import { audioControls, audioState } from '../stores/audio'

  const sourceLabel = {
    microphone: 'Microphone',
    system: 'Mac Output'
  } as const

  const modeLabel = {
    mock: 'Mock engine active',
    'microphone-planned': 'Microphone capture planned',
    'system-native-planned': 'Native output capture planned'
  } as const
</script>

<section class="panel">
  <div class="section">
    <p class="subtle">Control surface</p>
    <h2>Visualizer bootstrap</h2>
    <p class="body">
      The scene is already reactive. The next implementation pass swaps the mock source for live
      microphone and macOS output adapters.
    </p>
  </div>

  <div class="section">
    <p class="subtle">Audio source</p>
    <div class="button-row">
      <button
        type="button"
        class:active={$audioState.source === 'microphone'}
        on:click={() => audioControls.setSource('microphone')}
      >
        Microphone
      </button>
      <button
        type="button"
        class:active={$audioState.source === 'system'}
        on:click={() => audioControls.setSource('system')}
      >
        Mac Output
      </button>
    </div>
  </div>

  <div class="section">
    <p class="subtle">Transport</p>
    <button
      type="button"
      class="toggle"
      on:click={() => audioControls.toggle()}
    >
      {$audioState.running ? 'Pause reactive signal' : 'Start reactive signal'}
    </button>
    <p class="mode">{modeLabel[$audioState.captureMode]}</p>
  </div>

  <div class="section">
    <label>
      <span>Sensitivity</span>
      <input
        type="range"
        min="0.6"
        max="1.6"
        step="0.01"
        value={$audioState.sensitivity}
        on:input={(event) => audioControls.setSensitivity(Number(event.currentTarget.value))}
      />
    </label>
    <label>
      <span>Glow</span>
      <input
        type="range"
        min="0.2"
        max="1.4"
        step="0.01"
        value={$audioState.glow}
        on:input={(event) => audioControls.setGlow(Number(event.currentTarget.value))}
      />
    </label>
    <label>
      <span>Spread</span>
      <input
        type="range"
        min="0.2"
        max="1.3"
        step="0.01"
        value={$audioState.spread}
        on:input={(event) => audioControls.setSpread(Number(event.currentTarget.value))}
      />
    </label>
  </div>

  <div class="section metrics">
    <div class="metric">
      <span>Source</span>
      <strong>{sourceLabel[$audioState.source]}</strong>
    </div>
    <div class="metric">
      <span>Level</span>
      <strong>{Math.round($audioState.smoothedLevel * 100)}%</strong>
    </div>
    <div class="metric">
      <span>Beat Pulse</span>
      <strong>{Math.round($audioState.beat * 100)}%</strong>
    </div>
    <div class="metric">
      <span>Low / Mid / High</span>
      <strong>
        {Math.round($audioState.bands.low * 100)} / {Math.round($audioState.bands.mid * 100)} /
        {Math.round($audioState.bands.high * 100)}
      </strong>
    </div>
  </div>
</section>

<style>
  .panel {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 16px;
    border-radius: 28px;
    padding: 22px;
  }

  .section {
    border-radius: 20px;
    background: rgba(255, 255, 255, 0.025);
    padding: 16px;
  }

  h2 {
    margin: 4px 0 10px;
    font-size: 1.5rem;
    line-height: 1.05;
  }

  .body,
  .mode {
    margin: 0;
    color: rgba(227, 238, 250, 0.7);
    font-size: 0.95rem;
  }

  .button-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  button {
    border: 1px solid rgba(151, 194, 255, 0.2);
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.04);
    color: #edf6ff;
    padding: 12px 14px;
    transition:
      background 120ms ease,
      border-color 120ms ease,
      transform 120ms ease;
  }

  button:hover {
    transform: translateY(-1px);
  }

  button.active,
  .toggle {
    border-color: rgba(111, 227, 255, 0.45);
    background: linear-gradient(180deg, rgba(53, 127, 170, 0.28), rgba(18, 42, 66, 0.36));
  }

  .toggle {
    width: 100%;
    margin-bottom: 10px;
  }

  label {
    display: block;
  }

  label + label {
    margin-top: 14px;
  }

  label span {
    display: flex;
    justify-content: space-between;
    margin-bottom: 6px;
    color: rgba(227, 238, 250, 0.82);
  }

  input[type='range'] {
    width: 100%;
    accent-color: #6fe3ff;
  }

  .metrics {
    display: grid;
    gap: 10px;
  }
</style>
