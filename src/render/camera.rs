use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

/// Marker component for the camera entity
#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub boost_multiplier: f32,
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 10.0,
            boost_multiplier: 3.0,
            sensitivity: 0.002,
            yaw: 0.0,
            pitch: -15.0_f32.to_radians(),
        }
    }
}

/// Spawn the free-fly camera
pub fn spawn_camera(commands: &mut Commands) {
    let yaw = 0.0;
    let pitch = -15.0_f32.to_radians();
    commands.spawn((
        Camera3d::default(),
        FlyCamera {
            yaw,
            pitch,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 30.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Fly camera controls system — WASD movement, right-click mouse look, scroll speed
pub fn fly_camera_controls(
    time: Res<Time<Virtual>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    let Ok((mut camera, mut transform)) = query.get_single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mut speed = camera.speed;

    // ── Mouse wheel: adjust speed ──
    for ev in mouse_wheel.read() {
        speed *= 1.0 + ev.y * 0.1;
        speed = speed.clamp(0.5, 200.0);
    }
    camera.speed = speed;

    // ── Right mouse button: pitch / yaw look ──
    let mut total_delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        if mouse_buttons.pressed(MouseButton::Right) {
            total_delta += ev.delta;
        }
    }

    if total_delta != Vec2::ZERO {
        camera.yaw -= total_delta.x * camera.sensitivity;
        camera.pitch -= total_delta.y * camera.sensitivity;
        camera.pitch = camera.pitch.clamp(
            -89.0_f32.to_radians(),
            89.0_f32.to_radians(),
        );
    }

    // Apply rotation
    transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw)
        * Quat::from_axis_angle(Vec3::X, camera.pitch);

    // ── Keyboard movement ──
    let forward = transform.forward();
    let right = transform.right();
    let up = Vec3::Y; // world-space up for vertical movement

    let boost = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        camera.boost_multiplier
    } else {
        1.0
    };

    let move_speed = speed * boost * dt;

    let mut movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        movement += *forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        movement -= *forward;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        movement -= *right;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        movement += *right;
    }
    if keys.pressed(KeyCode::Space) {
        movement += up;
    }
    if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        // Shift also used for boost — descend with Ctrl
    }
    if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
        movement -= up;
    }

    if movement != Vec3::ZERO {
        movement = movement.normalize_or_zero() * move_speed;
        transform.translation += movement;
    }

    // ── Home key: reset to overview ──
    if keys.just_pressed(KeyCode::Home) {
        camera.yaw = 0.0;
        camera.pitch = -15.0_f32.to_radians();
        transform.translation = Vec3::new(0.0, 30.0, 60.0);
    }
}