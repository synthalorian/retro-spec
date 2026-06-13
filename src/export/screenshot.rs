use bevy::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// Resource holding the path where the screenshot will be saved
#[derive(Resource)]
pub struct ScreenshotPath(pub String);

static CAPTURED: AtomicBool = AtomicBool::new(false);

/// Take a screenshot using the OS screenshot tool (grim on Hyprland/Wayland, scrot on X11).
/// Fires once as a startup system — captures the window immediately on launch.
pub fn capture_screenshot(
    keys: Res<ButtonInput<KeyCode>>,
    path: Res<ScreenshotPath>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    if CAPTURED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        let output_path = &path.0;
        tracing::info!("Capturing screenshot to: {}", output_path);

        // Try grim (Wayland) first
        let grim_result = std::process::Command::new("grim")
            .arg(output_path)
            .output();
        if grim_result.is_ok_and(|o| o.status.success()) {
            tracing::info!("Screenshot saved: {}", output_path);
        } else {
            // Try scrot (X11)
            let scrot_result = std::process::Command::new("scrot")
                .arg(output_path)
                .output();
            if scrot_result.is_ok_and(|o| o.status.success()) {
                tracing::info!("Screenshot saved: {}", output_path);
            } else {
                // Try ImageMagick import
                let import_result = std::process::Command::new("import")
                    .args(["-window", "root", output_path])
                    .output();
                match import_result {
                    Ok(output) if output.status.success() => {
                        tracing::info!("Screenshot saved: {}", output_path);
                    }
                    Ok(output) => {
                        tracing::error!(
                            "Screenshot failed (exit code: {}): {}",
                            output.status,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                    Err(e) => {
                        tracing::error!("Failed to run screenshot command: {}", e);
                        tracing::info!("Install grim (Wayland), scrot (X11), or ImageMagick's import for screenshots");
                    }
                }
            }
        }

        // Auto-exit after screenshot
        exit.send(bevy::app::AppExit::Success);
    } else if keys.just_pressed(KeyCode::F12) {
        let output_path = &path.0;
        // F12 also triggers screenshot via OS tool
        let _ = std::process::Command::new("grim")
            .arg(output_path)
            .output();
        let _ = std::process::Command::new("scrot")
            .arg(output_path)
            .output();
        let _ = std::process::Command::new("import")
            .args(["-window", "root", output_path])
            .output();
    }
}