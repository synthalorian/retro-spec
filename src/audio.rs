use bevy::prelude::*;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackMode, PlaybackSettings};
use std::sync::Arc;

/// Generate a WAV-formatted synthwave ambient drone in memory.
/// Creates a deep bass drone with warm pad and shimmer harmonics.
fn generate_synthwave_wav(sample_rate: u32, duration_secs: f32) -> Vec<u8> {
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // Drone parameters: A1 bass, A3 pad, A4 shimmer
    let base_freq = 55.0;
    let pad_freq = 220.0;
    let shim_freq = 440.0;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // Sub-bass sine
        let bass = (t * base_freq * std::f32::consts::TAU).sin() * 0.3;
        // Fifth above
        let fifth = (t * base_freq * 1.5 * std::f32::consts::TAU).sin() * 0.12;
        // Warm pad with slow LFO wobble
        let pad_mod = (t * 0.08).sin() * 0.3 + 0.7;
        let pad = (t * pad_freq * std::f32::consts::TAU).sin() * 0.15 * pad_mod;
        // Shimmer with faster modulation
        let shim_mod = (t * 0.25).sin() * 0.5 + 0.5;
        let shim = (t * shim_freq * std::f32::consts::TAU).sin() * 0.06 * shim_mod;

        let sample = (bass + fifth + pad + shim).tanh(); // soft clip
        samples.push(sample);
    }

    // Pack as 16-bit PCM WAV
    let bits_per_sample = 16u16;
    let channels = 1u16;
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample / 8) as u32;
    let block_align = channels * (bits_per_sample / 8);
    let data_size = num_samples as u32 * block_align as u32;

    let mut wav = Vec::with_capacity(44 + data_size as usize);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes()); // file size - 8
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes());  // PCM format
    wav.extend_from_slice(&channels.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&bits_per_sample.to_le_bytes());

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());

    // Write 16-bit PCM samples
    for &s in &samples {
        let clamped = s.clamp(-1.0, 1.0);
        let val = (clamped * i16::MAX as f32) as i16;
        wav.extend_from_slice(&val.to_le_bytes());
    }

    wav
}

/// Set up ambient synthwave audio on startup.
/// Generates a procedural WAV drone and plays it on loop.
pub fn setup_audio(mut commands: Commands, mut audio_assets: ResMut<Assets<AudioSource>>) {
    let wav_bytes = generate_synthwave_wav(44100, 300.0); // 5 minutes
    let source = AudioSource {
        bytes: wav_bytes.into(),
    };
    let handle = audio_assets.add(source);

    commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    ));

    tracing::info!("Ambient synthwave drone started (5min loop)");
}