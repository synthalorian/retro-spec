use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::city::builder::BuildingMesh;
use crate::git::blame::BlameHeatmap;
use crate::CommitData;
use std::collections::HashMap;

const LOD_HIDE_DIST: f32 = 200.0;

/// Component marking an entity as a building
#[derive(Component)]
pub struct Building {
    pub commit_id: String,
    pub height: f32,
    pub is_tagged: bool,
    pub timestamp: i64,
}

/// Component marking a rotating tag beacon on top of a landmark building
#[derive(Component)]
pub struct TagBeacon {
    pub rotate_speed: f32,
}

/// Spawn all building meshes with procedural window grid textures.
/// Optionally applies blame heat map coloring when blame data is provided.
pub fn spawn_buildings(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    buildings: &[BuildingMesh],
    blame_heatmap: Option<&BlameHeatmap>,
    commit_data: Option<&CommitData>,
) {
    let mut texture_cache: HashMap<u32, Handle<Image>> = HashMap::new();

    // Build file_path -> heat lookup from blame hotspots
    let file_heat: HashMap<&str, f32> = if let Some(bh) = blame_heatmap {
        bh.hotspots.iter().map(|h| (h.path.as_str(), h.heat)).collect()
    } else {
        HashMap::new()
    };

    // Build commit_id -> files lookup for quick access
    let commit_files: HashMap<&str, &Vec<String>> = if let Some(cd) = commit_data {
        cd.commits.iter().map(|c| (c.id.as_str(), &c.files)).collect()
    } else {
        HashMap::new()
    };

    for b in buildings {
        let seed = simple_hash(&b.commit_id);
        let lit_pct = if b.is_tagged { 65 } else { 45 };

        // ── Compute heat-tinted color ──
        // Average the heat values for this commit's files
        let heat = commit_files
            .get(b.commit_id.as_str())
            .map(|files| {
                let mut sum = 0.0f32;
                let mut count = 0usize;
                for f in files.iter() {
                    if let Some(h) = file_heat.get(f.as_str()) {
                        sum += h;
                        count += 1;
                    }
                }
                if count > 0 { sum / count as f32 } else { 0.0 }
            })
            .unwrap_or(0.0);

        // Blend base color toward red/orange based on heat
        let mut tinted = heat_tint_color(b.color, heat);

        // Commit type tint (feat=blue, fix=green, docs=yellow, etc.)
        if let Some(cd) = commit_data {
            if let Some(c) = cd.commits.iter().find(|c| c.id == b.commit_id) {
                let type_color = commit_type_color(&c.commit_type);
                // 85% existing tinted, 15% type color
                tinted = [
                    (tinted[0] * 0.85 + type_color[0] * 0.15).clamp(0.0, 1.0),
                    (tinted[1] * 0.85 + type_color[1] * 0.15).clamp(0.0, 1.0),
                    (tinted[2] * 0.85 + type_color[2] * 0.15).clamp(0.0, 1.0),
                    tinted[3],
                ];
            }
        }

        let tex_handle = texture_cache
            .entry(seed)
            .or_insert_with(|| images.add(generate_window_texture(seed, tinted, lit_pct)));

        let color = Color::srgba(tinted[0], tinted[1], tinted[2], tinted[3]);
        let emissive_strength = if b.is_tagged { 0.8 } else { 0.2 };

        let base_color = if b.is_tagged {
            Color::srgba(
                (tinted[0] + 1.0) / 2.0,
                (tinted[1] + 0.8) / 2.0,
                tinted[2] / 2.0,
                1.0,
            )
        } else {
            color
        };

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(b.width, b.height, b.depth)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color,
                base_color_texture: Some(tex_handle.clone()),
                emissive: LinearRgba::from(color) * emissive_strength,
                ..Default::default()
            })),
            Transform::from_xyz(b.position.0, b.height / 2.0, b.position.2),
            Building {
                commit_id: b.commit_id.clone(),
                height: b.height,
                is_tagged: b.is_tagged,
                timestamp: b.timestamp,
            },
        ));

        // For tagged buildings: spawn a rotating beacon on top
        if b.is_tagged {
            let beacon_color = Color::srgb(1.0, 0.85, 0.3);
            // Gold-colored beacon light marker
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.25, 0.6))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: beacon_color,
                    emissive: LinearRgba::from(beacon_color) * 0.8,
                    unlit: true,
                    ..Default::default()
                })),
                Transform::from_xyz(b.position.0, b.height + 0.3, b.position.2),
                TagBeacon { rotate_speed: 1.2 },
            ));

            // Small point light for glow
            commands.spawn((
                PointLight {
                    intensity: 100.0,
                    color: beacon_color,
                    range: 8.0,
                    ..Default::default()
                },
                Transform::from_xyz(b.position.0, b.height + 0.5, b.position.2),
            ));
        }
    }
}

/// Blend an author color toward red/orange based on heat (0.0 = no change, 1.0 = fully hot).
fn heat_tint_color(color: [f32; 4], heat: f32) -> [f32; 4] {
    let heat = heat.clamp(0.0, 1.0);
    if heat <= 0.0 {
        return color;
    }
    // Hot color: bright orange-red
    let hot = [1.0, 0.3, 0.1, 1.0];
    [
        color[0] * (1.0 - heat) + hot[0] * heat,
        color[1] * (1.0 - heat) + hot[1] * heat,
        color[2] * (1.0 - heat) + hot[2] * heat,
        color[3],
    ]
}

/// Rotate all tag beacons around the Y axis
pub fn rotate_tag_beacons(
    time: Res<Time<Virtual>>,
    mut query: Query<(&TagBeacon, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (beacon, mut transform) in query.iter_mut() {
        transform.rotate_y(beacon.rotate_speed * dt);
    }
}

/// Generate a procedural 64×128 window grid texture.
/// Layout: 4 columns × 8 rows. Each cell is 16×16px with 2px walls.
/// Windows are randomly lit based on `lit_pct`, seeded by commit hash.
fn generate_window_texture(seed: u32, color: [f32; 4], lit_pct: u32) -> Image {
    let cols = 4u32;
    let rows = 8u32;
    let cell_size = 16u32;
    let wall = 2u32;

    let width = cols * cell_size;
    let height = rows * cell_size;
    let pixels = (width * height) as usize;
    let mut data = vec![0u8; pixels * 4];

    let wall_r = (color[0].clamp(0.0, 1.0) * 80.0) as u8;
    let wall_g = (color[1].clamp(0.0, 1.0) * 80.0) as u8;
    let wall_b = (color[2].clamp(0.0, 1.0) * 80.0) as u8;

    let mut rng_state = seed;

    for row in 0..rows {
        for col in 0..cols {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let is_lit = (rng_state % 100) < lit_pct;

            let (wr, wg, wb) = if is_lit {
                (
                    200 + (rng_state % 55) as u8,
                    160 + (rng_state % 60) as u8,
                    60 + (rng_state % 60) as u8,
                )
            } else {
                (12, 18, 45)
            };

            for cy in 0..cell_size {
                for cx in 0..cell_size {
                    let px = col * cell_size + cx;
                    let py = row * cell_size + cy;
                    let idx = ((py * width + px) * 4) as usize;

                    let is_wall =
                        cx < wall || cx >= cell_size - wall || cy < wall || cy >= cell_size - wall;

                    if is_wall {
                        data[idx] = wall_r;
                        data[idx + 1] = wall_g;
                        data[idx + 2] = wall_b;
                        data[idx + 3] = 255;
                    } else {
                        data[idx] = wr;
                        data[idx + 1] = wg;
                        data[idx + 2] = wb;
                        data[idx + 3] = 255;
                    }
                }
            }
        }
    }

    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}

/// Map commit type to a subtle tint color
fn commit_type_color(commit_type: &str) -> [f32; 4] {
    match commit_type {
        "feat" => [0.3, 0.5, 1.0, 1.0],      // blue
        "fix"  => [0.3, 1.0, 0.4, 1.0],      // green
        "docs" => [1.0, 1.0, 0.3, 1.0],      // yellow
        "chore" => [0.5, 0.5, 0.5, 1.0],     // grey
        "refactor" => [0.6, 0.3, 1.0, 1.0],  // purple
        "test"  => [1.0, 0.6, 0.2, 1.0],     // orange
        "style" => [1.0, 0.4, 0.7, 1.0],     // pink
        "perf"  => [1.0, 0.2, 0.2, 1.0],     // red
        _       => [0.0, 0.0, 0.0, 1.0],     // no tint
    }
}

/// DJB2 hash for string to u32 seed
fn simple_hash(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u32);
    }
    hash
}

/// LOD system — hide buildings beyond a distance threshold
/// For repos with 10K+ commits, this prevents rendering distant buildings
/// and reduces draw calls by 90%+ when flying high above the city.
pub fn apply_lod(
    camera: Query<&GlobalTransform, With<Camera3d>>,
    mut buildings: Query<(&GlobalTransform, &mut Visibility), With<Building>>,
) {
    let Ok(cam_transform) = camera.get_single() else {
        return;
    };
    let cam_pos = cam_transform.translation();

    for (transform, mut visibility) in buildings.iter_mut() {
        let dist = transform.translation().distance(cam_pos);
        if dist > LOD_HIDE_DIST {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

/// Pulse animation on the focused building (diff preview). 
/// When the player is near a building, it subtly pulses in scale.
pub fn animate_focused_building(
    time: Res<Time<Virtual>>,
    focused: Res<crate::FocusedBuilding>,
    mut query: Query<(&Building, &mut Transform)>,
) {
    let elapsed = time.elapsed_secs();
    let Some(ref focused_id) = focused.commit_id else {
        return;
    };

    for (building, mut transform) in query.iter_mut() {
        if &building.commit_id == focused_id {
            let pulse = 1.0 + (elapsed * 3.0).sin() * 0.03;
            // Only scale X and Z (not Y) — building grows wider but not taller
            let current = transform.scale;
            transform.scale = Vec3::new(pulse, 1.0, pulse);
        }
    }
}
