use crate::theme::Theme;

/// Matrix theme: green phosphor, high contrast
pub fn theme() -> Theme {
    Theme {
        name: "matrix",
        sky_color_top: [0.0, 0.0, 0.0, 1.0],
        sky_color_bottom: [0.0, 0.05, 0.0, 1.0],
        grid_color: [0.0, 1.0, 0.2, 0.4],
        ground_color: [0.0, 0.01, 0.0, 1.0],
        building_base: [0.0, 0.3, 0.0, 1.0],
        building_emissive: [0.0, 1.0, 0.2, 1.0],
        street_color: [0.0, 0.8, 0.1, 0.8],
        accent_color: [0.0, 1.0, 0.0, 1.0],
        ambient_light: [0.0, 0.05, 0.0],
        ambient_brightness: 0.3,
    }
}
