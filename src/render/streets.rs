use bevy::prelude::*;
use crate::city::builder::{StreetMesh, MergePlazaMesh, SkybridgeMesh};

/// Spawn street meshes from a city plan
pub fn spawn_streets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    streets: &[StreetMesh],
) {
    for s in streets {
        let dx = s.end.0 - s.start.0;
        let dz = s.end.2 - s.start.2;
        let length = (dx * dx + dz * dz).sqrt();
        let mid_x = (s.start.0 + s.end.0) / 2.0;
        let mid_z = (s.start.2 + s.end.2) / 2.0;
        let angle = f32::atan2(dz, dx);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(length, 0.05, s.width)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(s.color[0], s.color[1], s.color[2], s.color[3]),
                emissive: Color::srgba(s.color[0], s.color[1], s.color[2], s.color[3]).into(),
                unlit: true,
                ..Default::default()
            })),
            Transform::from_xyz(mid_x, 0.03, mid_z).with_rotation(Quat::from_rotation_y(angle)),
        ));
    }

    // Branch boulevard labels — floating text above each street
    for s in streets {
        let mid_x = (s.start.0 + s.end.0) / 2.0;
        let mid_z = (s.start.2 + s.end.2) / 2.0;
        commands.spawn((
            Text2d::new(format!("🌆 {}", s.name)),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
            TextColor(Color::srgba(s.color[0], s.color[1], s.color[2], 1.0)),
            Transform::from_xyz(mid_x, 2.5, mid_z)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2 * 0.6)),
        ));
    }
}

/// Spawn glowing merge intersection plazas — glowing rings at branch crossroads
pub fn spawn_merge_plazas(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    plazas: &[MergePlazaMesh],
) {
    for p in plazas {
        let color = Color::srgba(p.color[0], p.color[1], p.color[2], p.color[3]);

        // Outer glow ring — torus
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(p.radius, 0.15))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::from(color).into(),
                unlit: true,
                ..Default::default()
            })),
            Transform::from_xyz(p.position.0, 0.1, p.position.1),
        ));

        // Inner disc — flat glowing circle
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.05, p.radius * 0.6))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(p.color[0], p.color[1], p.color[2], 0.3),
                emissive: LinearRgba::from(color) * 0.3,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_xyz(p.position.0, 0.05, p.position.1),
        ));
    }
}

/// Spawn skybridge connectors — thin glass cylinders between matching branch lots
pub fn spawn_skybridges(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    skybridges: &[SkybridgeMesh],
) {
    for s in skybridges {
        let dx = s.end.0 - s.start.0;
        let dy = s.end.1 - s.start.1;
        let dz = s.end.2 - s.start.2;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();

        if length < 0.01 {
            continue;
        }

        let mid_x = (s.start.0 + s.end.0) / 2.0;
        let mid_y = (s.start.1 + s.end.1) / 2.0;
        let mid_z = (s.start.2 + s.end.2) / 2.0;

        // Direction vector and rotation to align cylinder
        let dir = Vec3::new(dx, dy, dz).normalize();
        let up = Vec3::Y;
        let rotation = Quat::from_axis_angle(
            up.cross(dir).normalize(),
            up.dot(dir).acos(),
        );

        let glass_color = Color::srgba(s.color[0], s.color[1], s.color[2], s.color[3]);

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.08, length))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: glass_color,
                emissive: LinearRgba::from(glass_color) * 0.5,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_xyz(mid_x, mid_y, mid_z).with_rotation(rotation),
        ));
    }
}