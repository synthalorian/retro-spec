pub mod synthwave84;
pub mod matrix;
pub mod chrome;
pub mod omarchy;

use serde::{Deserialize, Serialize};

/// A complete visual theme
#[derive(Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: &'static str,
    pub sky_color_top: [f32; 4],
    pub sky_color_bottom: [f32; 4],
    pub grid_color: [f32; 4],
    pub ground_color: [f32; 4],
    pub building_base: [f32; 4],
    pub building_emissive: [f32; 4],
    pub street_color: [f32; 4],
    pub accent_color: [f32; 4],
    pub ambient_light: [f32; 3],
    pub ambient_brightness: f32,
}

/// Get a theme by name
pub fn get_theme(name: &str) -> Theme {
    match name {
        "synthwave84" => synthwave84::theme(),
        "matrix" => matrix::theme(),
        "chrome" => chrome::theme(),
        "omarchy" => omarchy::theme(),
        _ => synthwave84::theme(),
    }
}

/// List all available theme names
pub fn list_themes() -> Vec<&'static str> {
    vec!["synthwave84", "matrix", "chrome", "omarchy"]
}
