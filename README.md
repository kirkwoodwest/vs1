# Music Visualizer

Standalone Tauri + Svelte + Threlte starter for a desktop music visualizer.

## Current Scope

- desktop shell: Tauri 2
- frontend: Svelte 5 + Vite
- 3D scene: Threlte / Three.js
- v1 visual target: glowing particles that pulse in brightness and space
- v1 input target: selectable microphone or macOS playback output

The current scaffold includes a mock audio engine so the visual system is already live before real capture adapters are implemented.

## Run

```bash
npm install
npm run tauri:dev
```

## Useful Commands

- `npm run dev` starts the frontend only
- `npm run tauri:dev` starts the desktop app
- `npm run check` runs Svelte and TypeScript checks
- `npm run build` builds the frontend bundle

## Notes

- microphone capture can stay frontend-side through the Web Audio API
- macOS playback-output capture will likely require a native Tauri-side adapter or loopback strategy
- the project plan lives in `PLAN.md`
