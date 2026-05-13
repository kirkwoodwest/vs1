use screencapturekit::prelude::*;
use screencapturekit::{AudioBuffer, AudioBufferList};
use serde::Serialize;
use std::{
  collections::VecDeque,
  f32::consts::PI,
  sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
  },
  thread,
  time::Duration,
};
use tauri::{AppHandle, Emitter, State};

const WAVEFORM_SIZE: usize = 64;
const SPECTRUM_SIZE: usize = 32;
const ANALYSIS_WINDOW: usize = 512;
const MAX_RING_BUFFER: usize = 4096;
const EMIT_INTERVAL_MS: u64 = 33;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FrequencyBandsPayload {
  low: f32,
  mid: f32,
  high: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AudioFramePayload {
  level: f32,
  smoothed_level: f32,
  beat: f32,
  bands: FrequencyBandsPayload,
  waveform: Vec<f32>,
  spectrum: Vec<f32>,
}

#[derive(Clone, Copy)]
struct AudioFormatInfo {
  sample_rate: f64,
  bits_per_channel: u32,
  is_float: bool,
  is_big_endian: bool,
}

impl Default for AudioFormatInfo {
  fn default() -> Self {
    Self {
      sample_rate: 48_000.0,
      bits_per_channel: 32,
      is_float: true,
      is_big_endian: false,
    }
  }
}

struct AnalysisState {
  format: AudioFormatInfo,
  waveform_ring: VecDeque<f32>,
  level: f32,
  smoothed_level: f32,
  low_average: f32,
  beat: f32,
}

impl Default for AnalysisState {
  fn default() -> Self {
    Self {
      format: AudioFormatInfo::default(),
      waveform_ring: VecDeque::with_capacity(MAX_RING_BUFFER),
      level: 0.0,
      smoothed_level: 0.0,
      low_average: 0.0,
      beat: 0.0,
    }
  }
}

impl AnalysisState {
  fn ingest(&mut self, sample: &CMSampleBuffer) {
    if let Some(format_description) = sample.format_description() {
      self.format.sample_rate = format_description
        .audio_sample_rate()
        .unwrap_or(self.format.sample_rate);
      self.format.bits_per_channel = format_description
        .audio_bits_per_channel()
        .unwrap_or(self.format.bits_per_channel);
      self.format.is_float = format_description.audio_is_float();
      self.format.is_big_endian = format_description.audio_is_big_endian();
    }

    let Some(audio_buffers) = sample.audio_buffer_list() else {
      return;
    };

    let mono = collapse_to_mono(&audio_buffers, self.format);
    if mono.is_empty() {
      return;
    }

    let mut energy = 0.0f32;
    for &value in &mono {
      self.waveform_ring.push_back(value);
      if self.waveform_ring.len() > MAX_RING_BUFFER {
        self.waveform_ring.pop_front();
      }
      energy += value * value;
    }

    self.level = (energy / mono.len() as f32).sqrt().min(1.0);
    let response = if self.level > self.smoothed_level { 0.55 } else { 0.22 };
    self.smoothed_level += (self.level - self.smoothed_level) * response;
  }

  fn snapshot(&mut self) -> Option<AudioFramePayload> {
    if self.waveform_ring.is_empty() {
      return None;
    }

    let buffer = latest_window(&self.waveform_ring, ANALYSIS_WINDOW);
    let waveform = downsample_waveform(&buffer, WAVEFORM_SIZE);
    let spectrum = calculate_spectrum(&buffer, self.format.sample_rate, SPECTRUM_SIZE);

    let low = average_range(&spectrum, 0, 6);
    let mid = average_range(&spectrum, 6, 16);
    let high = average_range(&spectrum, 16, SPECTRUM_SIZE);

    self.low_average += (low - self.low_average) * 0.18;
    let onset = ((low - self.low_average * 0.9) * 5.8).clamp(0.0, 1.0);
    self.beat = onset.max(self.beat * 0.72);

    Some(AudioFramePayload {
      level: self.level,
      smoothed_level: self.smoothed_level,
      beat: self.beat,
      bands: FrequencyBandsPayload { low, mid, high },
      waveform,
      spectrum,
    })
  }
}

struct SystemAudioCaptureSession {
  stop_flag: Arc<AtomicBool>,
  join_handle: Option<thread::JoinHandle<()>>,
}

#[derive(Default)]
pub struct SystemAudioCaptureState {
  session: Mutex<Option<SystemAudioCaptureSession>>,
}

#[tauri::command]
pub fn start_system_audio_capture(
  app: AppHandle,
  state: State<'_, SystemAudioCaptureState>,
) -> Result<String, String> {
  stop_existing_session(&state)?;

  let stop_flag = Arc::new(AtomicBool::new(false));
  let worker_stop_flag = Arc::clone(&stop_flag);
  let worker_app = app.clone();
  let (startup_tx, startup_rx) = mpsc::channel();

  let join_handle = thread::spawn(move || {
    if let Err(error) = run_capture_loop(worker_app, worker_stop_flag, startup_tx) {
      log::error!("system audio capture thread failed: {error}");
    }
  });

  let startup_result = startup_rx
    .recv_timeout(Duration::from_secs(5))
    .map_err(|_| "Timed out while starting system audio capture.".to_string())?;

  match startup_result {
    Ok(()) => {
      let mut session = state
        .session
        .lock()
        .map_err(|_| "System audio state lock poisoned.".to_string())?;
      *session = Some(SystemAudioCaptureSession {
        stop_flag,
        join_handle: Some(join_handle),
      });
      Ok("Live macOS playback-output capture active.".to_string())
    }
    Err(error) => {
      stop_flag.store(true, Ordering::Relaxed);
      let _ = join_handle.join();
      Err(error)
    }
  }
}

#[tauri::command]
pub fn stop_system_audio_capture(
  state: State<'_, SystemAudioCaptureState>,
) -> Result<(), String> {
  stop_existing_session(&state)
}

fn stop_existing_session(state: &State<'_, SystemAudioCaptureState>) -> Result<(), String> {
  let previous = {
    let mut guard = state
      .session
      .lock()
      .map_err(|_| "System audio state lock poisoned.".to_string())?;
    guard.take()
  };

  if let Some(mut session) = previous {
    session.stop_flag.store(true, Ordering::Relaxed);
    if let Some(join_handle) = session.join_handle.take() {
      let _ = join_handle.join();
    }
  }

  Ok(())
}

fn run_capture_loop(
  app: AppHandle,
  stop_flag: Arc<AtomicBool>,
  startup_tx: mpsc::Sender<Result<(), String>>,
) -> Result<(), String> {
  let content = SCShareableContent::get().map_err(|error| {
    format!(
      "Unable to access ScreenCaptureKit shareable content. Grant Screen Recording permission in macOS Settings, then restart the app. Details: {error}"
    )
  })?;

  let display = content
    .displays()
    .into_iter()
    .next()
    .ok_or_else(|| {
      "No shareable display found. Screen Recording permission may still be blocked.".to_string()
    })?;

  let filter = SCContentFilter::create()
    .with_display(&display)
    .with_excluding_windows(&[])
    .build();

  let config = SCStreamConfiguration::new()
    .with_width(display.width())
    .with_height(display.height())
    .with_captures_audio(true)
    .with_excludes_current_process_audio(true)
    .with_sample_rate(48_000)
    .with_channel_count(2);

  let analysis = Arc::new(Mutex::new(AnalysisState::default()));
  let handler_state = Arc::clone(&analysis);

  let mut stream = SCStream::new(&filter, &config);
  stream.add_output_handler(
    move |sample: CMSampleBuffer, output_type: SCStreamOutputType| {
      if output_type != SCStreamOutputType::Audio {
        return;
      }

      if let Ok(mut state) = handler_state.lock() {
        state.ingest(&sample);
      }
    },
    SCStreamOutputType::Audio,
  );

  if let Err(error) = stream.start_capture() {
    let _ = startup_tx.send(Err(format!(
      "Failed to start macOS playback-output capture. Confirm Screen Recording permission and try again. Details: {error}"
    )));
    return Err(error.to_string());
  }

  let _ = startup_tx.send(Ok(()));

  while !stop_flag.load(Ordering::Relaxed) {
    thread::sleep(Duration::from_millis(EMIT_INTERVAL_MS));

    let payload = {
      let Ok(mut state) = analysis.lock() else {
        continue;
      };
      state.snapshot()
    };

    if let Some(payload) = payload {
      let _ = app.emit("system-audio-frame", payload);
    }
  }

  stream
    .stop_capture()
    .map_err(|error| format!("Failed to stop system audio capture cleanly: {error}"))?;

  Ok(())
}

fn latest_window(ring: &VecDeque<f32>, size: usize) -> Vec<f32> {
  let len = ring.len();
  let start = len.saturating_sub(size);
  ring.iter().skip(start).copied().collect()
}

fn downsample_waveform(buffer: &[f32], output_size: usize) -> Vec<f32> {
  if buffer.is_empty() {
    return vec![0.0; output_size];
  }

  let stride = (buffer.len() / output_size.max(1)).max(1);
  (0..output_size)
    .map(|index| {
      let sample_index = (index * stride).min(buffer.len() - 1);
      buffer[sample_index].clamp(-1.0, 1.0)
    })
    .collect()
}

fn calculate_spectrum(buffer: &[f32], sample_rate: f64, bins: usize) -> Vec<f32> {
  if buffer.is_empty() {
    return vec![0.0; bins];
  }

  let windowed: Vec<f32> = buffer
    .iter()
    .enumerate()
    .map(|(index, sample)| {
      let weight = 0.5 - 0.5 * ((2.0 * PI * index as f32) / buffer.len() as f32).cos();
      sample * weight
    })
    .collect();

  let nyquist = (sample_rate as f32 / 2.0).max(1.0);

  (0..bins)
    .map(|bin| {
      let frequency = ((bin + 1) as f32 / bins as f32) * nyquist;
      let mut real = 0.0f32;
      let mut imaginary = 0.0f32;

      for (index, sample) in windowed.iter().enumerate() {
        let phase = 2.0 * PI * frequency * index as f32 / sample_rate as f32;
        real += sample * phase.cos();
        imaginary -= sample * phase.sin();
      }

      ((real * real + imaginary * imaginary).sqrt() / windowed.len() as f32 * 5.0).clamp(0.0, 1.0)
    })
    .collect()
}

fn average_range(values: &[f32], start: usize, end: usize) -> f32 {
  if values.is_empty() || start >= end || start >= values.len() {
    return 0.0;
  }

  let upper = end.min(values.len());
  let slice = &values[start..upper];
  if slice.is_empty() {
    0.0
  } else {
    slice.iter().copied().sum::<f32>() / slice.len() as f32
  }
}

fn collapse_to_mono(audio_buffers: &AudioBufferList, format: AudioFormatInfo) -> Vec<f32> {
  let decoded: Vec<Vec<f32>> = audio_buffers
    .iter()
    .map(|buffer| decode_audio_buffer(buffer, format))
    .filter(|samples| !samples.is_empty())
    .collect();

  if decoded.is_empty() {
    return Vec::new();
  }

  if decoded.len() == 1 {
    return decoded[0].clone();
  }

  let min_len = decoded.iter().map(Vec::len).min().unwrap_or(0);
  (0..min_len)
    .map(|index| decoded.iter().map(|samples| samples[index]).sum::<f32>() / decoded.len() as f32)
    .collect()
}

fn decode_audio_buffer(buffer: &AudioBuffer, format: AudioFormatInfo) -> Vec<f32> {
  let data = buffer.data();
  if data.is_empty() {
    return Vec::new();
  }

  let bytes_per_sample = match format.bits_per_channel {
    16 => 2,
    32 => 4,
    _ => 4,
  };

  let channel_count = buffer.number_channels.max(1) as usize;

  if format.is_float && bytes_per_sample == 4 {
    let frame_count = data.len() / bytes_per_sample / channel_count;
    let mut mono = Vec::with_capacity(frame_count);

    for frame_index in 0..frame_count {
      let mut mixed = 0.0f32;
      for channel_index in 0..channel_count {
        let start = (frame_index * channel_count + channel_index) * bytes_per_sample;
        let bytes = [data[start], data[start + 1], data[start + 2], data[start + 3]];
        let sample = if format.is_big_endian {
          f32::from_bits(u32::from_be_bytes(bytes))
        } else {
          f32::from_bits(u32::from_le_bytes(bytes))
        };
        mixed += sample;
      }
      mono.push((mixed / channel_count as f32).clamp(-1.0, 1.0));
    }

    return mono;
  }

  if bytes_per_sample == 2 {
    let frame_count = data.len() / bytes_per_sample / channel_count;
    let mut mono = Vec::with_capacity(frame_count);

    for frame_index in 0..frame_count {
      let mut mixed = 0.0f32;
      for channel_index in 0..channel_count {
        let start = (frame_index * channel_count + channel_index) * bytes_per_sample;
        let bytes = [data[start], data[start + 1]];
        let sample = if format.is_big_endian {
          i16::from_be_bytes(bytes)
        } else {
          i16::from_le_bytes(bytes)
        } as f32
          / i16::MAX as f32;
        mixed += sample;
      }
      mono.push((mixed / channel_count as f32).clamp(-1.0, 1.0));
    }

    return mono;
  }

  Vec::new()
}
