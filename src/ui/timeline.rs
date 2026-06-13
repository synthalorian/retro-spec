use bevy::prelude::*;
use crate::render::buildings::Building;

/// Timeline scrubber — drag through history
#[derive(Resource)]
pub struct TimelineState {
    pub current_position: f32,   // 0.0 (earliest commit) to 1.0 (latest)
    pub total_commits: usize,
    pub min_ts: i64,
    pub max_ts: i64,
    pub span_ts: i64,
    pub is_dragging: bool,
}

impl TimelineState {
    pub fn new(total_commits: usize) -> Self {
        Self {
            current_position: 1.0,
            total_commits,
            min_ts: 0,
            max_ts: 0,
            span_ts: 1,
            is_dragging: false,
        }
    }

    /// Get the current scrub time (commits before this are visible)
    pub fn current_time(&self) -> i64 {
        self.min_ts + (self.span_ts as f32 * self.current_position) as i64
    }

    /// Initialize from commit data
    pub fn init_from_commits(&mut self, commits: &[crate::git::commit::CommitInfo]) {
        if commits.is_empty() {
            return;
        }
        self.total_commits = commits.len();
        self.min_ts = commits.last().map(|c| c.timestamp).unwrap_or(0);
        self.max_ts = commits.first().map(|c| c.timestamp).unwrap_or(0);
        self.span_ts = (self.max_ts - self.min_ts).max(1);
        self.current_position = 1.0;
    }
}

/// Marker for the timeline slider track
#[derive(Component)]
pub struct TimelineTrack;

/// Marker for the timeline slider thumb
#[derive(Component)]
pub struct TimelineThumb;

/// Set up the timeline scrubber UI
pub fn setup_timeline(mut commands: Commands) {
    // Timeline track bar at the bottom
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(40.0),
                left: Val::Px(100.0),
                right: Val::Px(100.0),
                height: Val::Px(8.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.8)),
            TimelineTrack,
        ))
        .with_children(|parent| {
            // Thumb (draggable indicator)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(100.0), // starts at end (latest)
                    width: Val::Px(16.0),
                    height: Val::Px(24.0),
                    top: Val::Px(-8.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(1.0, 0.2, 0.6, 1.0)),
                TimelineThumb,
            ));
        });
}

/// Handle timeline slider interaction — click/drag to scrub
pub fn timeline_interaction(
    mut timeline: ResMut<TimelineState>,
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    track_query: Query<&Node, With<TimelineTrack>>,
    mut thumb_query: Query<&mut Node, (With<TimelineThumb>, Without<TimelineTrack>)>,
) {
    let Ok(_track_node) = track_query.get_single() else {
        return;
    };
    let Ok(mut thumb_node) = thumb_query.get_single_mut() else {
        return;
    };

    let window = if let Some(w) = windows.iter().next() {
        w
    } else {
        return;
    };

    // Track bounds in screen coords
    let window_width = window.resolution.physical_width() as f32;
    let track_left = 100.0; // left: Val::Px(100)
    let track_right = window_width - 100.0; // right: Val::Px(100)
    let track_width = track_right - track_left;

    // Start/stop dragging on mouse press/release
    if mouse_button.just_pressed(MouseButton::Left) {
        timeline.is_dragging = true;
    }
    if mouse_button.just_released(MouseButton::Left) {
        timeline.is_dragging = false;
    }

    // Update position while dragging
    if timeline.is_dragging
        && let Some(cursor) = cursor_moved.read().last()
    {
            let mouse_x = cursor.position.x * window.scale_factor();
            let ratio = ((mouse_x - track_left) / track_width).clamp(0.0, 1.0);
            timeline.current_position = ratio;

            // Move thumb to match
            thumb_node.left = Val::Percent(ratio * 100.0);
        }

    // Always sync thumb position with the current timeline value
    thumb_node.left = Val::Percent(timeline.current_position * 100.0);
}

/// Update building visibility based on timeline position
pub fn update_building_visibility(
    timeline: Res<TimelineState>,
    mut building_query: Query<(&Building, &mut Visibility)>,
) {
    let scrub_time = timeline.current_time();

    for (building, mut visibility) in building_query.iter_mut() {
        if building.timestamp <= scrub_time {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}