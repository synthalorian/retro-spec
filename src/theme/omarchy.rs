use crate::theme::Theme;

/// Omarchy theme: synthwave '84 deep purple aesthetic
pub fn theme() -> Theme {
    Theme {
        name: "omarchy",
        // #240037 deep purple
        sky_color_top: [0.141, 0.0, 0.216, 1.0],
        // Slightly lighter purple for bottom sky gradient
        sky_color_bottom: [0.2, 0.0, 0.3, 1.0],
        // #240037 deep purple ground
        ground_color: [0.141, 0.0, 0.216, 1.0],
        // #f3e70f yellow grid
        grid_color: [0.953, 0.906, 0.059, 0.4],
        // #8f00ff purple accent
        accent_color: [0.561, 0.0, 1.0, 1.0],
        // #ff7edb pink building base
        building_base: [1.0, 0.494, 0.859, 0.8],
        // #8f00ff purple emissive
        building_emissive: [0.561, 0.0, 1.0, 1.0],
        // #ff7edb pink streets
        street_color: [1.0, 0.494, 0.859, 0.8],
        // #ff00ff magenta secondary → ambient light
        ambient_light: [0.3, 0.0, 0.3],
        ambient_brightness: 0.35,
    }
}