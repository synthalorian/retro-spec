use crate::theme::Theme;

/// Default theme: neon sunset with magenta/cyan accents
pub fn theme() -> Theme {
    Theme {
        name: "synthwave84",
        sky_color_top: [0.0, 0.0, 0.08, 1.0],
        sky_color_bottom: [1.0, 0.3, 0.4, 1.0],
        grid_color: [1.0, 0.2, 0.6, 0.3],
        ground_color: [0.02, 0.01, 0.05, 1.0],
        building_base: [0.2, 0.1, 0.6, 1.0],
        building_emissive: [0.6, 0.2, 1.0, 1.0],
        street_color: [1.0, 0.2, 0.6, 0.8],
        accent_color: [0.2, 1.0, 1.0, 1.0],
        ambient_light: [0.1, 0.05, 0.2],
        ambient_brightness: 0.4,
    }
}
