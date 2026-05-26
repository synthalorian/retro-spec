use bevy::prelude::*;

/// Resource holding the path where the screenshot will be saved
#[derive(Resource)]
pub struct ScreenshotPath(pub String);

/// Take a screenshot using the OS screenshot tool (grim on Hyprland/Wayland, scrot on X11).
/// Fires once as a startup system — captures the window immediately on launch.
pub fn capture_screenshot(
    keys: Res<ButtonInput<KeyCode>>,
    path: Res<ScreenshotPath>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    // Check for F12 keypress or fire on first frame
    static mut CAPTURED: bool = false;
    // SAFETY: single-threaded Bevy system
    unsafe {
        if !CAPTURED {
            CAPTURED = true;
            let output_path = &path.0;
            tracing::info!("Capturing screenshot to: {}", output_path);

            // Try grim (Wayland), scrot (X11), or import (ImageMagick)
            let result = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "grim \"{}\" 2>/dev/null || scrot \"{}\" 2>/dev/null || import -window root \"{}\" 2>/dev/null",
                    output_path, output_path, output_path
                ))
                .output();

            match result {
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

            // Auto-exit after screenshot
            exit.send(bevy::app::AppExit::Success);
        } else if keys.just_pressed(KeyCode::F12) {
            CAPTURED = true;
            // F12 also triggers screenshot via OS tool
            let output_path = &path.0;
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "grim \"{}\" 2>/dev/null || scrot \"{}\" 2>/dev/null || import -window root \"{}\" 2>/dev/null",
                    output_path, output_path, output_path
                ))
                .output()
                .ok();
        }
    }
}