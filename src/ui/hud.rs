use bevy::prelude::*;
use crate::render::buildings::Building;

/// HUD overlay — commit details, author info, stats
#[derive(Resource)]
#[allow(dead_code)]
pub struct HudState {
    pub selected_commit: Option<String>,
    pub commit_count: usize,
    pub branch_count: usize,
    pub author_count: usize,
}

impl HudState {
    pub fn new(commit_count: usize, branch_count: usize, author_count: usize) -> Self {
        Self {
            selected_commit: None,
            commit_count,
            branch_count,
            author_count,
        }
    }
}

/// Marker component for HUD text entities we update each frame
#[derive(Component)]
pub struct HudDescriptionText;

/// Marker for the HUD container
#[derive(Component)]
pub struct HudContainer;

/// Set up the HUD overlay UI
pub fn setup_hud(mut commands: Commands) {
    let font_size = 16.0;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            HudContainer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                TextLayout::new_with_justify(JustifyText::Left),
                HudDescriptionText,
            ));
        });
}

/// HUD system — find nearest building and update overlay text
pub fn hud_system(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    building_query: Query<(&GlobalTransform, &Building)>,
    mut hud_text: Query<&mut Text, With<HudDescriptionText>>,
    commit_data: Res<crate::CommitData>,
    _hud_state: Res<HudState>,
    mut focused: ResMut<crate::FocusedBuilding>,
) {
    let Ok(mut text) = hud_text.get_single_mut() else {
        return;
    };

    let Ok((_camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let camera_pos = camera_transform.translation();

    // Find nearest building within 50 units
    let nearest = building_query
        .iter()
        .filter_map(|(gt, building)| {
            let dist = gt.translation().distance(camera_pos);
            if dist < 50.0 {
                Some((dist, building, gt))
            } else {
                None
            }
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    if let Some((_dist, building, _gt)) = nearest {
        // Look up commit info
        let info = commit_data
            .commits
            .iter()
            .find(|c| c.id == building.commit_id);

        if let Some(commit) = info {
            let date = chrono::DateTime::from_timestamp(commit.timestamp, 0)
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            let msg_first_line = commit.message.lines().next().unwrap_or("").to_string();

            let tag_indicator = if commit.is_tagged {
                format!(" [{}]", commit.tags.join(", "))
            } else {
                String::new()
            };

            // Truncate long messages
            let msg = if msg_first_line.len() > 60 {
                format!("{}...", &msg_first_line[..57])
            } else {
                msg_first_line
            };

            text.0 = format!(
                "═══ {} ═══{}\n\
                 {} by {}\n\
                 │ +{} / -{}  {} files\n\
                 │ {}{}",
                &commit.id[..7],
                if commit.is_merge { " 🔀" } else { "" },
                date,
                commit.author,
                commit.lines_added,
                commit.lines_deleted,
                commit.files_changed,
                msg,
                tag_indicator,
            );

                        // Set focused building for diff preview pulse
                        focused.commit_id = Some(commit.id.clone());
                    } else {
                        text.0 = String::new();
                        focused.commit_id = None;
                    }
                } else {
                    text.0 = String::new();
                    focused.commit_id = None;
                }
}