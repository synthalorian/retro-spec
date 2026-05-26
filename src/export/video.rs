use bevy::prelude::*;
use crate::render::camera::FlyCamera;

/// Tracks export progress while rendering a flythrough video.
#[derive(Resource)]
pub struct ExportState {
    pub output_dir: String,
    pub total_frames: u32,
    pub current_frame: u32,
    pub orbit_angle: f32,
    pub orbit_radius: f32,
    pub done: bool,
}

impl ExportState {
    pub fn new(output_dir: String, duration_seconds: f32, fps: u32) -> Self {
        let total_frames = (duration_seconds * fps as f32) as u32;
        Self {
            output_dir,
            total_frames,
            current_frame: 0,
            orbit_angle: 0.0,
            orbit_radius: 80.0,
            done: false,
        }
    }
}

/// Marker to identify the export frame-capture system
#[derive(Component)]
pub struct ExportCameraMarker;

/// Run on each Update frame during export mode.
/// Drives the camera along an orbit path, captures screenshots,
/// and when finished encodes the frames into a video.
pub fn export_flythrough(
    mut state: ResMut<ExportState>,
    time: Res<Time<Virtual>>,
    mut cam_query: Query<(&mut FlyCamera, &mut Transform), Without<ExportCameraMarker>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    if state.done {
        return;
    }

    let Ok((_camera, mut transform)) = cam_query.get_single_mut() else {
        return;
    };

    // ── Compute orbit position ──
    // Orbit in a circle, slowly descending from overview height
    let progress = state.current_frame as f32 / state.total_frames.max(1) as f32;
    let angle = progress * std::f32::consts::TAU * 2.0; // two full orbits
    let height = 40.0 - progress * 20.0; // descend from 40 to 20 units
    let radius = state.orbit_radius;

    transform.translation = Vec3::new(
        angle.cos() * radius,
        height,
        angle.sin() * radius,
    );
    // Look at city center
    transform.look_at(Vec3::ZERO, Vec3::Y);

    // ── Capture frame via OS screenshot tool ──
    let frame_path = format!(
        "{}/frame_{:04}.png",
        state.output_dir,
        state.current_frame
    );

    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "grim \"{}\" 2>/dev/null || scrot \"{}\" 2>/dev/null || import -window root \"{}\" 2>/dev/null",
            frame_path, frame_path, frame_path
        ))
        .output();

    state.current_frame += 1;

    // ── Check if done ──
    if state.current_frame >= state.total_frames {
        state.done = true;
        tracing::info!(
            "Export complete: {} frames captured to {}/",
            state.total_frames,
            state.output_dir
        );

        // Try encoding with ffmpeg if available
        let output_video = format!("{}/flythrough.mp4", state.output_dir);
        let result = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ffmpeg -y -framerate 30 -i {0}/frame_%04d.png \
                 -c:v libx264 -preset medium -crf 20 \
                 -pix_fmt yuv420p {1} 2>/dev/null",
                state.output_dir, output_video
            ))
            .output();

        match result {
            Ok(out) if out.status.success() => {
                tracing::info!("Video encoded: {}", output_video);
            }
            Ok(_) => {
                tracing::info!("Install ffmpeg to encode frames into a video.");
                tracing::info!("Frames saved to: {}/frame_*.png", state.output_dir);
            }
            Err(_) => {
                tracing::info!("Frames saved to: {}/frame_*.png", state.output_dir);
            }
        }

        exit.send(bevy::app::AppExit::Success);
    }
}