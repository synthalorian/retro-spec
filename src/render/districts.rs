use bevy::prelude::*;
use crate::city::builder::DistrictMesh;

/// Spawn district ground tint planes
pub fn spawn_districts(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    districts: &[DistrictMesh],
) {
    for d in districts {
        let color = Color::srgba(d.color[0], d.color[1], d.color[2], d.color[3]);

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(d.size.0, d.size.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_xyz(d.center.0, -0.05, d.center.1),
        ));
    }
}