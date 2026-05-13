#[cfg(target_os = "macos")]
mod system_audio;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(target_os = "macos")]
      app.manage(system_audio::SystemAudioCaptureState::default());

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      #[cfg(target_os = "macos")]
      system_audio::start_system_audio_capture,
      #[cfg(target_os = "macos")]
      system_audio::stop_system_audio_capture,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
