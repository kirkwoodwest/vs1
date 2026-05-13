<script lang="ts">
  import { audioControls, audioState } from '../stores/audio'

  const sourceLabel = {
    microphone: 'Microphone',
    system: 'Mac Output'
  } as const

  const modeLabel = {
    mock: 'Mock engine active',
    'microphone-live': 'Microphone live',
    'microphone-permission-needed': 'Microphone permission needed',
    'microphone-unavailable': 'Microphone unavailable',
    'system-live': 'Mac output live',
    'system-native-planned': 'Native output capture planned'
  } as const
</script>

<section class="panel">
  <div class="section">
    <p class="subtle">Control surface</p>
    <h2>Visualizer bootstrap</h2>
    <p class="body">
      Microphone capture is live. Mac output now uses a native macOS path and depends on Screen
      Recording permission.
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
      {$audioState.running ? 'Stop capture' : 'Start capture'}
    </button>
    <p class="mode">{modeLabel[$audioState.captureMode]}</p>
    <p class="mode">{$audioState.statusMessage}</p>
    {#if $audioState.errorMessage}
      <p class="error">{$audioState.errorMessage}</p>
    {/if}
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
      <span>Mode</span>
      <strong>{modeLabel[$audioState.captureMode]}</strong>
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
    min-height: 0;
    overflow: auto;
    border-radius: 24px;
    padding: 16px;
  }

  .section {
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.025);
    padding: 14px;
  }

  h2 {
    margin: 4px 0 10px;
    font-size: 1.25rem;
    line-height: 1.05;
  }

  .body,
  .mode {
    margin: 0;
    color: rgba(227, 238, 250, 0.7);
    font-size: 0.88rem;
  }

  .mode + .mode {
    margin-top: 8px;
  }

  .error {
    margin: 10px 0 0;
    color: #ffb199;
    font-size: 0.9rem;
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
    padding: 10px 12px;
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
    margin-top: 12px;
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
