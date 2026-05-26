use bevy::prelude::*;
use crate::theme::Theme;

/// Set up directional + ambient lighting for the city, themed
pub fn setup_lighting(commands: &mut Commands, theme: &Theme) {
    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: false,
            ..Default::default()
        },
        Transform::from_xyz(50.0, 80.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light from theme
    commands.insert_resource(AmbientLight {
        color: Color::srgb(
            theme.ambient_light[0],
            theme.ambient_light[1],
            theme.ambient_light[2],
        ),
        brightness: theme.ambient_brightness,
    });

    // Point light accent colors from theme
    let accent = theme.accent_color;
    for x in (-40..=40).step_by(20) {
        for z in (-40..=40).step_by(20) {
            commands.spawn((
                PointLight {
                    intensity: 50.0,
                    color: Color::srgb(accent[0], accent[1], accent[2]),
                    range: 15.0,
                    ..Default::default()
                },
                Transform::from_xyz(x as f32, 2.0, z as f32),
            ));
        }
    }
}