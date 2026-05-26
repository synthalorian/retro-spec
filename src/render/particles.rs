use bevy::prelude::*;
use rand::Rng;

/// Component for ambient floating particles
#[derive(Component)]
pub struct NeonParticle {
    pub float_speed: f32,
    pub float_offset: f32,
    pub initial_y: f32,
}

/// Spawn ambient neon dust particles — small glowing spheres floating above the city
pub fn spawn_particles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mut rng = rand::thread_rng();
    let particle_count = 60;

    for _ in 0..particle_count {
        let x = rng.gen_range(-90.0..90.0);
        let z = rng.gen_range(-90.0..90.0);
        let y = rng.gen_range(0.5..15.0);
        let size = rng.gen_range(0.08..0.25);

        // Random neon color from palette
        let palette = [
            Color::srgb(1.0, 0.2, 0.6),  // pink
            Color::srgb(0.2, 0.6, 1.0),  // cyan
            Color::srgb(1.0, 0.8, 0.2),  // gold
            Color::srgb(0.6, 0.2, 1.0),  // purple
            Color::srgb(0.2, 1.0, 0.6),  // mint
        ];
        let color = palette[rng.gen_range(0..palette.len())];

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size).mesh().ico(2).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::from(color) * 0.6,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_xyz(x, y, z),
            NeonParticle {
                float_speed: rng.gen_range(0.2..0.8),
                float_offset: rng.gen_range(0.0..6.28),
                initial_y: y,
            },
        ));
    }
}

/// Float particles up and down slowly
pub fn animate_particles(
    time: Res<Time<Virtual>>,
    mut query: Query<(&NeonParticle, &mut Transform)>,
) {
    let elapsed = time.elapsed_secs();
    for (particle, mut transform) in query.iter_mut() {
        let float = (elapsed * particle.float_speed + particle.float_offset).sin() * 2.0;
        transform.translation.y = particle.initial_y + float;
    }
}