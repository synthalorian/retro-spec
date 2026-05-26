use bevy::prelude::*;

/// Color-coded legend — maps authors and directories to their neon colors
#[derive(Resource)]
pub struct LegendState {
    pub authors: Vec<(String, [f32; 4])>,
    pub directories: Vec<(String, [f32; 4])>,
    pub visible: bool,
}

impl LegendState {
    pub fn new() -> Self {
        Self {
            authors: vec![],
            directories: vec![],
            visible: true,
        }
    }
}

/// Marker for the legend container
#[derive(Component)]
pub struct LegendContainer;

/// Marker for the legend toggle hint
#[derive(Component)]
pub struct LegendToggleText;

/// Set up the legend overlay UI — positioned top-right
pub fn setup_legend(
    mut commands: Commands,
    state: Res<LegendState>,
) {
    if !state.visible {
        return;
    }

    let font_size = 13.0;
    let header_size = 15.0;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                min_width: Val::Px(160.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            LegendContainer,
        ))
        .with_children(|parent| {
            // Authors header
            parent.spawn((
                Text::new("👤 Authors"),
                TextFont {
                    font_size: header_size,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.8, 0.8, 1.0)),
                TextLayout::new_with_justify(JustifyText::Left),
            ));

            // Author entries
            for (i, (author, color)) in state.authors.iter().enumerate() {
                let display_name = if author.len() > 14 {
                    format!("{}…", &author[..13])
                } else {
                    author.clone()
                };

                parent.spawn((
                    Text::new(format!(
                        "  {} {}",
                        color_swatch(*color),
                        display_name
                    )),
                    TextFont {
                        font_size,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(color[0], color[1], color[2])),
                    TextLayout::new_with_justify(JustifyText::Left),
                ));
            }

            // Spacer
            parent.spawn((
                Text::new(""),
                TextFont { font_size: 4.0, ..Default::default() },
                TextColor(Color::WHITE),
            ));

            // Directories header
            parent.spawn((
                Text::new("📁 Districts"),
                TextFont {
                    font_size: header_size,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.6, 1.0, 0.6)),
                TextLayout::new_with_justify(JustifyText::Left),
            ));

            // Directory entries (limit to 8 to avoid clutter)
            for (i, (dir, color)) in state.directories.iter().enumerate().take(8) {
                let display_name = if dir.len() > 14 {
                    format!("{}…", &dir[..13])
                } else {
                    dir.clone()
                };

                let muted_color = [
                    color[0] * 0.7 + 0.3,
                    color[1] * 0.7 + 0.3,
                    color[2] * 0.7 + 0.3,
                    1.0,
                ];

                parent.spawn((
                    Text::new(format!(
                        "  {} {}",
                        color_swatch(muted_color),
                        display_name
                    )),
                    TextFont {
                        font_size,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(muted_color[0], muted_color[1], muted_color[2])),
                    TextLayout::new_with_justify(JustifyText::Left),
                ));
            }

            if state.directories.len() > 8 {
                parent.spawn((
                    Text::new(format!("  +{} more…", state.directories.len() - 8)),
                    TextFont { font_size: 11.0, ..Default::default() },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                ));
            }

            // Toggle hint
            parent.spawn((
                Text::new(""),
                TextFont { font_size: 4.0, ..Default::default() },
                TextColor(Color::WHITE),
                LegendToggleText,
            ));
        });
}

/// Toggle legend visibility with the L key
pub fn toggle_legend_system(
    mut legend: ResMut<LegendState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<LegendContainer>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        legend.visible = !legend.visible;
        if let Ok(mut vis) = query.get_single_mut() {
            *vis = if legend.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Render a text-based color swatch
fn color_swatch(color: [f32; 4]) -> String {
    // Use block characters filled proportionally to the color brightness
    let brightness = (color[0] * 0.299 + color[1] * 0.587 + color[2] * 0.114).clamp(0.0, 1.0);
    if brightness > 0.6 {
        "■".to_string()
    } else {
        "■".to_string()
    }
}