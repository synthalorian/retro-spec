use crate::theme::Theme;

/// Chrome theme: clean, minimal, glass-morphism aesthetic
pub fn theme() -> Theme {
    Theme {
        name: "chrome",
        sky_color_top: [0.85, 0.87, 0.9, 1.0],
        sky_color_bottom: [0.75, 0.8, 0.9, 1.0],
        grid_color: [0.7, 0.75, 0.8, 0.2],
        ground_color: [0.9, 0.92, 0.95, 1.0],
        building_base: [0.6, 0.65, 0.75, 1.0],
        building_emissive: [0.8, 0.85, 1.0, 1.0],
        street_color: [0.7, 0.75, 0.8, 0.6],
        accent_color: [0.3, 0.5, 1.0, 1.0],
        ambient_light: [0.5, 0.55, 0.6],
        ambient_brightness: 0.6,
    }
}
