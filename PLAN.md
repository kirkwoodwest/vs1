# Music Visualizer Plan

## Goal

Build a standalone desktop music visualizer using Tauri, Svelte, and Threlte.

Initial focus:

- accept live audio input into the app
- analyze the incoming signal in real time
- drive a 3D visual scene from that analysis
- keep the architecture simple enough to evolve into a real repo later

## V1 Decisions

- audio sources: selectable microphone input or playback output from the Mac
- first visual: glowing particle field
- visual behavior: particles pulse in brightness and spatial movement to the beat
- analysis target: strong beat response first, fuller band-driven behavior second

## Working Assumptions

- frontend: Svelte + Threlte
- desktop shell: Tauri
- audio analysis: Web Audio API for microphone, native Tauri path likely required for macOS playback-output capture
- rendering: Three.js via Threlte
- packaging: local standalone app first, repo hygiene second

## Phase 0: Definition

Objective: remove ambiguity before scaffolding.

Deliverables:

- decide whether audio input means microphone, system audio, file playback, or all three
- decide whether the first visualizer is 2D-reactive, 3D-reactive, or fully scene-based
- define a minimal success case for v1

Recommended v1 scope:

- input source: selectable microphone or Mac playback output
- analysis: volume, waveform, frequency bands, and beat emphasis
- visuals: one 3D particle scene with glow and reactive motion
- controls: source selection, start/stop, sensitivity, glow/intensity

Technical note:

- microphone input can stay in a browser-style audio pipeline
- Mac playback-output capture is the harder path and may require a native capture implementation or OS-assisted loopback route

## Phase 1: Scaffold App

Objective: get a runnable desktop shell with frontend rendering.

Tasks:

- initialize a Svelte app suitable for Tauri
- add Tauri shell and confirm desktop startup
- add Threlte and render a minimal 3D scene
- confirm hot reload for frontend iteration
- set up project structure so frontend audio services and any native capture bridge can coexist cleanly

Exit criteria:

- app launches locally
- a Threlte scene renders inside the Tauri window

## Phase 2: Audio Pipeline

Objective: ingest audio and expose clean analysis data.

Tasks:

- create an audio service abstraction with separate source adapters
- implement microphone capture with the Web Audio API
- investigate and then implement playback-output capture for macOS through the Tauri side
- support permission flow for microphone and any OS capture requirements
- connect analyser nodes for waveform and FFT data
- normalize output into a stable app-facing store
- expose beat-oriented derived metrics for the first visual system

Data outputs:

- master amplitude
- smoothed amplitude
- low / mid / high band energy
- beat strength / onset signal
- waveform buffer
- spectrum buffer

Exit criteria:

- app can start audio capture from at least one source, with source switching designed in
- analysis values update continuously and are inspectable in UI

## Phase 3: Reactive Visual System

Objective: translate audio features into motion and visual state.

Tasks:

- define a visual mapping layer between audio data and scene parameters
- implement a glowing particle field as the first scene
- map beat strength to pulse, bloom/glow, scale spread, and outward motion
- map band energy to particle color drift and spatial turbulence
- smooth animation to avoid jitter
- keep scene components isolated so more presets can be added later

Potential first-scene ideas:

- glowing particle field driven by beat and frequency bands
- future option: deforming sphere driven by amplitude and bass
- future option: tunnel or ribbon scene driven by waveform history

Exit criteria:

- scene responds visibly and musically to live input
- parameters can be tuned without changing core audio code

## Phase 4: Controls and UX

Objective: make the prototype usable without code edits.

Tasks:

- add source selection and start/stop controls
- add sliders for gain, smoothing, and intensity
- add controls for particle density, glow, and pulse response
- add preset switching for visual modes
- add a lightweight debug overlay for live metrics

Exit criteria:

- a user can launch, enable input, and change the experience from the UI

## Phase 5: Stabilization

Objective: make the app durable enough to become a real repo.

Tasks:

- organize directories for scene, audio, stores, and UI modules
- document setup and run steps
- validate performance in desktop windowed mode
- add basic tests where practical
- identify what should remain frontend-side versus move into Rust

Exit criteria:

- structure is clean enough to publish
- setup is repeatable
- obvious technical debt is documented

## Proposed Initial Structure

```text
src/
  lib/
    audio/
    scene/
    stores/
    ui/
  routes/
src-tauri/
```

## Decisions To Lock Next

1. Confirm the preferred macOS playback-output capture strategy after scaffold and research spike.
2. Decide whether preset switching belongs in v1 or waits until the first particle scene feels right.
3. Decide whether the app should only analyze external audio in v1 or also support internal file playback later.

## Recommended Next Step

Scaffold the app next, then treat playback-output capture as an explicit spike inside the audio phase so it does not block the entire frontend setup.
